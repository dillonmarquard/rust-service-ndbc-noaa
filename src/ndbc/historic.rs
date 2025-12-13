use super::ndbc_schema::{check_null_string, StationContinuousWindsData, StationDataType, StationHistoricFile, StationMetadata, StationStdMetData, StationsMetadataResponse};
use chrono::{Datelike, NaiveDateTime, Utc};
use log::debug;
use regex::Regex;
use serde_xml_rs::from_str;

pub async fn get_stations_metadata() -> Result<Vec<StationMetadata>, Box<dyn std::error::Error>> {
    // This function returns the historical station metadata for all stations on the NDBC.
    // This function is not currently used, and is only provided to ensure parity with source system.
    debug!("called get_stations_metadata");

    let url: &str = "https://www.ndbc.noaa.gov/metadata/stationmetadata.xml";
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<StationsMetadataResponse>(body.as_str()).unwrap();

    Ok(res.stations)
}

pub async fn get_station_available_downloads(station: &str, data_type: StationDataType) -> Result<Vec<StationHistoricFile>, Box<dyn std::error::Error>> {
    // This function returns a list of historic files for the given station and data_type (eg. stdmet, cwind, swden)
    // Please use get_historic_files for bulk lookup (and filter the desired stations) to avoid spamming the resource.
    debug!("called get_station_available_downloads");

    let url: String = format!("https://www.ndbc.noaa.gov/station_history.php?station={station}");
    let mut re = Regex::new(("".to_string() + r###"<a href="/download_data\.php\?filename=(.{5,25})\.(.{2,25})\&dir=data/historical/"### + data_type.as_str() + r###"/">(.{1,6})</a>"###).as_str()).unwrap();
    debug!("url {}", &url);
    debug!("re {}", &re);
    let body = reqwest::get(url).await?.text().await?;

    let mut res: Vec<StationHistoricFile> = re.captures_iter(&body).map(|c| c.extract()).map(|(_, [f, t, y])| StationHistoricFile { filename: f.to_string() + "." + t, station: f.to_string(), data_type: data_type.clone(), year: y.to_string() }).collect();

    re = Regex::new(("".to_string() + r###"<a href="/download_data\.php\?filename=(.{5,25})\.(.{2,25})\&dir=data/"### + data_type.as_str() + r###"/(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)/">(.{1,6})</a>"###).as_str()).unwrap();
    res.extend(re.captures_iter(&body).map(|c| c.extract()).map(|(_, [f, t, m, _])| StationHistoricFile { filename: f.to_string() + "." + t, station: f.to_string(), data_type: data_type.clone(), year: m.to_string() }));
    // the data for the current year is quality controlled separately from historic data

    Ok(res)
}

pub async fn get_historic_files(data_type: StationDataType) -> Result<Vec<StationHistoricFile>, Box<dyn std::error::Error>> {
    // This function returns a list of all downloadable historic files for a specified data_type (eg. stdmet, cwind, swden)
    debug!("called get_historic_files");

    let mut url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/historical/" + data_type.as_str();
    let re = Regex::new(r###"<tr><td valign="top"><img src="/icons/compressed.gif" alt="\[   \]"></td><td><a href="(.{5,50})">(.{5,50})</a></td><td align="right">(.{5,50})</td><td align="right">(.{1,50})</td><td>(.{1,50})</td></tr>"###).unwrap();
    debug!("url {}", &url);

    let mut body = reqwest::get(url).await?.text().await?;

    let mut res: Vec<StationHistoricFile> = re.captures_iter(&body).map(|c| c.extract()).map(|(_, [f, _, _, _, _])| StationHistoricFile { filename: f.to_string(), station: f[0..=4].to_string().to_uppercase(), data_type: data_type.clone(), year: f[6..=9].to_string() }).collect();

    for i in 1..=12 {
        url = "".to_string() + "https://www.ndbc.noaa.gov/data/" + data_type.as_str() + "/" +
        match i { 
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Error",
        } 
        + "/";
        body = reqwest::get(url).await?.text().await?;
        res.extend(re.captures_iter(&body).map(|c| c.extract()).map(|(_, [f, _, _, _, _])| StationHistoricFile { filename: f.to_string(), station: f[0..=4].to_string().to_uppercase(), data_type: data_type.clone(), year: match i { 
            1 => "Jan".to_string(),
            2 => "Feb".to_string(),
            3 => "Mar".to_string(),
            4 => "Apr".to_string(),
            5 => "May".to_string(),
            6 => "Jun".to_string(),
            7 => "Jul".to_string(),
            8 => "Aug".to_string(),
            9 => "Sep".to_string(),
            10 => "Oct".to_string(),
            11 => "Nov".to_string(),
            12 => "Dec".to_string(),
            _ => "Error".to_string(),
        } }));

    }
    
    Ok(res)
}

pub async fn get_station_historical_stdmet_data(station: &str, year: &str) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the historic raw stdmet sensor data for a given station over a given year.
    debug!("called get_station_historical_stdmet_data");
    
    let mut url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station.to_lowercase().as_str() // filenames are in lower-case and case sensitive
        + "h"
        + year
        + ".txt.gz"
        + "&dir=data/historical/"
        + StationDataType::StandardMeteorological.as_str()
        + "/";
    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(18))).unwrap();

    // this assumes NOAA compiles the annual reports for each buoy by the first day of the new year (unlikely)
    // this means this will likely not work at the start of a new year, until they compile the historic report
    // we could only avoid this by using the historic files for reference
    if year.chars().any(|c| c.is_alphabetic()) { // if year contains non-numeric character, then it must be a month of the year (supports mid-year historic data lookup)
        url = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station.to_lowercase().as_str() // filenames are in lower-case and case sensitive
        + match year { // in this case year represents a month (supports mid-year historic data)
            "Jan" => "1",
            "Feb" => "2",
            "Mar" => "3",
            "Apr" => "4",
            "May" => "5",
            "Jun" => "6",
            "Jul" => "7",
            "Aug" => "8",
            "Sep" => "9",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => "13"
        }
        + &Utc::now().year().to_string() // wont work if NOAA is slow to compile annual historic report
        + ".txt.gz"
        + "&dir=data/"
        + StationDataType::StandardMeteorological.as_str()
        + "/"
        + year // in this case year represents a month (supports mid-year historic data)
        + "/";
    }
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, tide])| StationStdMetData {
            station: station.to_string().to_uppercase(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
            wdir: if !check_null_string(wdir) { Some(wdir.parse().unwrap()) } else { None },
            wspd: if !check_null_string(wspd) { Some(wspd.parse().unwrap()) } else { None },
            gst: if !check_null_string(gst) { Some(gst.parse().unwrap()) } else { None },
            wvht: if !check_null_string(wvht) { Some(wvht.parse().unwrap()) } else { None },
            dpd: if !check_null_string(dpd) { Some(dpd.parse().unwrap()) } else { None },
            apd: if !check_null_string(apd) { Some(apd.parse().unwrap()) } else { None },
            mwd: if !check_null_string(mwd) { Some(mwd.parse().unwrap()) } else { None },
            pres: if !check_null_string(pres) { Some(pres.parse().unwrap()) } else { None },
            atmp: if !check_null_string(atmp) { Some(atmp.parse().unwrap()) } else { None },
            wtmp: if !check_null_string(wtmp) { Some(wtmp.parse().unwrap()) } else { None },
            dewp: if !check_null_string(dewp) { Some(dewp.parse().unwrap()) } else { None },
            vis: if !check_null_string(vis) { Some(vis.parse().unwrap()) } else { None },
            ptdy: None, // ptdy is not available in historic files
            tide: if !check_null_string(tide) { Some(tide.parse().unwrap()) } else { None },
        })
        .collect();

    Ok(res)
}

pub async fn get_station_historical_cwind_data(station: &str, year: &str) -> Result<Vec<StationContinuousWindsData>, Box<dyn std::error::Error>> {
    // This function returns the historic raw stdmet sensor data for a given station over a given year.
    debug!("called get_station_historical_cwind_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station.to_lowercase().as_str() // filenames are in lower-case and case sensitive
        + "c"
        + year
        + ".txt.gz"
        + "&dir=data/historical/"
        + StationDataType::ContinuousWinds.as_str()
        + "/";
    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(10))).unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re.captures_iter(&body).map(|c| c.extract()).map(|(_, [year, month, day, hour, minute, wdir, wspd, gdr, gst, _gtime])| StationContinuousWindsData { station: station.to_string().to_uppercase(), timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(), wdir: if !check_null_string(wdir) { Some(wdir.parse().unwrap()) } else { None }, wspd: if !check_null_string(wspd) { Some(wspd.parse().unwrap()) } else { None }, gdr: if !check_null_string(gdr) { Some(gdr.parse().unwrap()) } else { None }, gst: if !check_null_string(gst) { Some(gst.parse().unwrap()) } else { None } }).collect();

    Ok(res)
}
