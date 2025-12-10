use super::ndbc_schema::{check_null_string, ActiveStationsResponse, Station, StationContinuousWindsData, StationDataType, StationRealtimeFile, StationSpectralWaveSummary, StationStdMetData};
use chrono::NaiveDateTime;
use log::debug;
use regex::Regex;
use serde_xml_rs::from_str;

pub async fn get_active_stations() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    // This function returns a list of all active stations.
    // just because a station is active does not mean it has stdmet data.
    debug!("get_active_stations");
    let url: &str = "https://www.ndbc.noaa.gov/activestations.xml";
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsResponse>(body.as_str()).unwrap();

    Ok(res.stations)
}

pub async fn get_realtime_files(data_type: StationDataType) -> Result<Vec<StationRealtimeFile>, Box<dyn std::error::Error>> {
    // This function returns a list of all downloadable realtime files for a specified data_type (eg. stdmet, cwind, swden)
    debug!("get_realtime_files");

    let url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/";
    let re = Regex::new(r###"<tr><td valign="top"><img src="/icons/text.gif" alt="\[TXT\]"></td><td><a href="(.{5,50})\.(.{2,50})">(.{5,50})</a></td><td align="right">(.{5,50})</td><td align="right">(.{1,50})</td><td>(.{1,50})</td></tr>"###).unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [s, t, _, ts, _, _])| StationRealtimeFile {
            filename: s.to_string() + "." + t,
            station: s.to_string().to_uppercase(),
            data_type: match t {
                "txt" => StationDataType::StandardMeteorological,
                "cwind" => StationDataType::ContinuousWinds,
                "spec" => StationDataType::SpectralWaveSummary,
                _ => StationDataType::Unsupported,
            },
            timestamp: NaiveDateTime::parse_from_str(&ts.to_string().trim(), "%Y-%m-%d %H:%M").unwrap(),
        })
        .filter(|c| c.data_type == data_type)
        .collect();

    Ok(res)
}

pub async fn get_station_realtime_stdmet_data(station: &str) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_stdmet_data");

    let url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/" + station.to_uppercase().as_str() + ".txt";
    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(19))).unwrap();
    debug!("url {}", &url);

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, ptdy, tide])| StationStdMetData {
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
            ptdy: if !check_null_string(ptdy) { Some(ptdy.parse().unwrap()) } else { None },
            tide: if !check_null_string(tide) { Some(tide.parse().unwrap()) } else { None },
        })
        .collect();

    Ok(res)
}

pub async fn get_station_realtime_stdmetdrift_data(station: &str) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_stdmetdrift_data");

    let url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/" + station.to_uppercase().as_str() + ".drift";
    debug!("{}", &url);

    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(16))).unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hourminute, _lat, _lon, wdir, gst, wspd, pres, ptdy, atmp, wtmp, dewp, wvht, dpd])| StationStdMetData {
            // re-use stdmet struct even though some data is dropped
            station: station.to_string().to_uppercase(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hourminute).as_str(), "%Y-%m-%d %H%M").unwrap(),
            wdir: if !check_null_string(wdir) { Some(wdir.parse().unwrap()) } else { None },
            wspd: if !check_null_string(wspd) { Some(wspd.parse().unwrap()) } else { None },
            gst: if !check_null_string(gst) { Some(gst.parse().unwrap()) } else { None },
            wvht: if !check_null_string(wvht) { Some(wvht.parse().unwrap()) } else { None },
            dpd: if !check_null_string(dpd) { Some(dpd.parse().unwrap()) } else { None },
            apd: None,
            mwd: None,
            pres: if !check_null_string(pres) { Some(pres.parse().unwrap()) } else { None },
            atmp: if !check_null_string(atmp) { Some(atmp.parse().unwrap()) } else { None },
            wtmp: if !check_null_string(wtmp) { Some(wtmp.parse().unwrap()) } else { None },
            dewp: if !check_null_string(dewp) { Some(dewp.parse().unwrap()) } else { None },
            vis: None,
            ptdy: if !check_null_string(ptdy) { Some(ptdy.parse().unwrap()) } else { None },
            tide: None,
        })
        .collect();

    Ok(res)
}

pub async fn get_station_realtime_cwind_data(station: &str) -> Result<Vec<StationContinuousWindsData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_cwind_data");

    let url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/" + station.to_uppercase().as_str() + ".cwind";
    debug!("{}", &url);

    let re = Regex::new(&(r"([0-9M\+\.-]+)[\s\n]+".repeat(10))).unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re.captures_iter(&body).map(|c| c.extract()).map(|(_, [year, month, day, hour, minute, wdir, wspd, gdr, gst, _gtime])| StationContinuousWindsData { station: station.to_string().to_uppercase(), timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(), wdir: if !check_null_string(wdir) { Some(wdir.parse().unwrap()) } else { None }, wspd: if !check_null_string(wspd) { Some(wspd.parse().unwrap()) } else { None }, gdr: if !check_null_string(gdr) { Some(gdr.parse().unwrap()) } else { None }, gst: if !check_null_string(gst) { Some(gst.parse().unwrap()) } else { None } }).collect();

    Ok(res)
}

pub async fn get_station_realtime_spec_data(station: &str) -> Result<Vec<StationSpectralWaveSummary>, Box<dyn std::error::Error>> {
    // This function returns the spectral wave summary sensor data for a given station over the last 45 days.
    debug!("get_station_realtime_spec_data");

    let url: String = "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/" + station.to_uppercase().as_str() + ".spec";
    debug!("{:?}", &url);

    let re = Regex::new(&(r"(?m)^".to_string() + &r"([A-Z0-9\+\.\-_]+)[\s\n]+".repeat(15))).unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wvht, swh, swp, wwh, wwp, swd, wwd, steep, apd, mwd])| StationSpectralWaveSummary {
            station: station.to_string().to_uppercase(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
            wvht: if !check_null_string(wvht) { Some(wvht.parse().unwrap()) } else { None },
            swh: if !check_null_string(swh) { Some(swh.parse().unwrap()) } else { None },
            swp: if !check_null_string(swp) { Some(swp.parse().unwrap()) } else { None },
            wwh: if !check_null_string(wwh) { Some(wwh.parse().unwrap()) } else { None },
            wwp: if !check_null_string(wwp) { Some(wwp.parse().unwrap()) } else { None },
            swd: if !check_null_string(swd) { Some(swd.parse().unwrap()) } else { None },
            wwd: if !check_null_string(wwd) { Some(wwd.parse().unwrap()) } else { None },
            steep: if !check_null_string(steep) { Some(steep.parse().unwrap()) } else { None },
            apd: if !check_null_string(apd) { Some(apd.parse().unwrap()) } else { None },
            mwd: if !check_null_string(mwd) { Some(mwd.parse().unwrap()) } else { None },
        })
        .collect();

    Ok(res)
}
