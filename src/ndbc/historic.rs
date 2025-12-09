use chrono::NaiveDateTime;
use regex::Regex;
use serde_xml_rs::from_str;

use super::ndbc_schema::{
    check_null_string, StationContinuousWindsData, StationDataType, StationHistoricFile,
    StationMetadata, StationStdMetData, StationsMetadataResponse,
};

use log::debug;

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

pub async fn get_station_available_downloads(
    station: &str,
    data_type: StationDataType,
) -> Result<Vec<StationHistoricFile>, Box<dyn std::error::Error>> {
    // This function returns a list of historic files for the given station and data_type (eg. stdmet, cwind, swden)
    // Please use get_historic_files for bulk lookup (and filter the desired stations) to avoid spamming the resource.
    debug!("called get_station_available_downloads");

    let url: String = format!("https://www.ndbc.noaa.gov/station_history.php?station={station}");
    let re = Regex::new(
        ("".to_string()
            + r###"<a href="/download_data\.php\?filename=(.{5,25})\.(.{2,25})\&dir=data/historical/"###
            + data_type.as_str()
            + r###"/">(.{1,6})</a>"###)
            .as_str(),
    )
    .unwrap();
    debug!("url {}", &url);
    debug!("re {}", &re);
    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [f, t, y])| StationHistoricFile {
            filename: f.to_string() + "." + t,
            station: f.to_string(),
            data_type: StationDataType::Unsupported,
            year: y.to_string(),
        })
        .collect();

    Ok(res)
}

pub async fn get_historic_files(
    data_type: StationDataType,
) -> Result<Vec<StationHistoricFile>, Box<dyn std::error::Error>> {
    // This function returns a list of all downloadable historic files for a specified data_type (eg. stdmet, cwind, swden)
    debug!("called get_historic_files");

    let url: String =
        "".to_string() + "https://www.ndbc.noaa.gov/data/historical/" + data_type.as_str();
    let re = Regex::new(
        r###"<tr><td valign="top"><img src="/icons/compressed.gif" alt="\[   \]"></td><td><a href="(.{5,50})">(.{5,50})</a></td><td align="right">(.{5,50})</td><td align="right">(.{1,50})</td><td>(.{1,50})</td></tr>"###,
    )
    .unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [f, _, _, _, _])| StationHistoricFile {
            filename: f.to_string(),
            station: f[0..=4].to_string().to_uppercase(),
            data_type: StationDataType::Unsupported,
            year: f[6..=9].to_string(),
        })
        .collect();

    Ok(res)
}

pub async fn get_station_historical_stdmet_data(
    station: &str,
    year: &str,
) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the historic raw stdmet sensor data for a given station over a given year.
    debug!("called get_station_historical_stdmet_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station.to_lowercase().as_str() // filenames are in lower-case and is case sensitive
        + "h"
        + year
        + ".txt.gz"
        + "&dir=data/historical/"
        + StationDataType::StandardMeteorological.as_str()
        + "/";
    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(18))).unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, tide])| StationStdMetData {
            station: station.to_string().to_uppercase(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
            wdir: if !check_null_string(wdir) { Some(wdir.to_string()) } else {None},
            wspd: if !check_null_string(wspd) { Some(wspd.to_string())} else {None},
            gst: if !check_null_string(gst) { Some(gst.to_string())} else {None},
            wvht: if !check_null_string(wvht) { Some(wvht.to_string())} else {None},
            dpd: if !check_null_string(dpd) { Some(dpd.to_string())} else {None},
            apd: if !check_null_string(apd) { Some(apd.to_string())} else {None},
            mwd: if !check_null_string(mwd) { Some(mwd.to_string())} else {None},
            pres: if !check_null_string(pres) { Some(pres.to_string())} else {None},
            atmp: if !check_null_string(atmp) { Some(atmp.to_string())} else {None},
            wtmp: if !check_null_string(wtmp) { Some(wtmp.to_string())} else {None},
            dewp: if !check_null_string(dewp) { Some(dewp.to_string())} else {None},
            vis: if !check_null_string(vis) { Some(vis.to_string())} else {None},
            ptdy: None, // ptdy is not available in historic files
            tide: if !check_null_string(tide) { Some(tide.to_string())} else {None}
        })
        .collect();

    Ok(res)
}

pub async fn get_station_historical_cwind_data(
    station: &str,
    year: &str,
) -> Result<Vec<StationContinuousWindsData>, Box<dyn std::error::Error>> {
    // This function returns the historic raw stdmet sensor data for a given station over a given year.
    debug!("called get_station_historical_cwind_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station.to_lowercase().as_str() // filenames are in lower-case and is case sensitive
        + "c"
        + year
        + ".txt.gz"
        + "&dir=data/historical/"
        + StationDataType::ContinuousWinds.as_str()
        + "/";
    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(10))).unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(
            |(_, [year, month, day, hour, minute, wdir, wspd, gdr, gst, _gtime])| {
                StationContinuousWindsData {
                    station: station.to_string().to_uppercase(),
                    timestamp: NaiveDateTime::parse_from_str(
                        ("".to_string()
                            + year
                            + "-"
                            + month
                            + "-"
                            + day
                            + " "
                            + hour
                            + ":"
                            + minute)
                            .as_str(),
                        "%Y-%m-%d %H:%M",
                    )
                    .unwrap(),
                    wdir: if !check_null_string(wdir) {
                        Some(wdir.to_string())
                    } else {
                        None
                    },
                    wspd: if !check_null_string(wspd) {
                        Some(wspd.to_string())
                    } else {
                        None
                    },
                    gdr: if !check_null_string(gdr) {
                        Some(gdr.to_string())
                    } else {
                        None
                    },
                    gst: if !check_null_string(gst) {
                        Some(gst.to_string())
                    } else {
                        None
                    },
                }
            },
        )
        .collect();

    Ok(res)
}
