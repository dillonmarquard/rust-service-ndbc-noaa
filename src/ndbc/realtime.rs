use chrono::NaiveDateTime;
use regex::Regex;
use serde_xml_rs::from_str;

use super::ndbc_schema::{ActiveStationsResponse, Station, StationStdMetData, StationContinuousWindsData, check_null_string};

use log::debug;

pub async fn get_active_stations() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    // This function returns a list of active stations.
    // just because a station is active does not mean it has stdmet data.
    debug!("get_active_stations");
    let url: &str = "https://www.ndbc.noaa.gov/activestations.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsResponse>(body.as_str()).unwrap();

    Ok(res.stations)
}

pub async fn get_station_realtime_stdmet_data(
    station: &str,
) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_stdmet_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/data/realtime2/"
        + station.to_uppercase().as_str()
        + ".txt";
    let re = Regex::new(
        &(r"([0-9M\+\.-]+)[\s\n]+".repeat(19)),
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, ptdy, tide])| 
            StationStdMetData {
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
                ptdy: if !check_null_string(ptdy) { Some(ptdy.to_string())} else {None},
                tide: if !check_null_string(tide) { Some(tide.to_string())} else {None},
            }
        )
        .collect();

    Ok(res)
}

pub async fn get_station_realtime_stdmetdrift_data(
    station: &str,
) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_stdmetdrift_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/data/realtime2/"
        + station.to_uppercase().as_str()
        + ".drift";
    
    let re = Regex::new(
        &(r"([0-9M\+\.-]+)[\s\n]+".repeat(16)),
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hourminute, _lat, _lon, wdir, gst, wspd, pres, ptdy, atmp, wtmp, dewp, wvht, dpd])| 
            StationStdMetData { 
                // re-use stdmet struct even though some data is dropped
                station: station.to_string().to_uppercase(),
                timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hourminute).as_str(), "%Y-%m-%d %H%M").unwrap(),
                wdir: if wdir != "MM" { Some(wdir.to_string()) } else {None} ,
                wspd: if wspd != "MM" { Some(wspd.to_string())} else {None},
                gst: if gst != "MM" { Some(gst.to_string())} else {None},
                wvht: if wvht != "MM" { Some(wvht.to_string())} else {None},
                dpd: if dpd != "MM" { Some(dpd.to_string())} else {None},
                apd: None,
                mwd: None,
                pres: if pres != "MM" { Some(pres.to_string())} else {None},
                atmp: if atmp != "MM" { Some(atmp.to_string())} else {None},
                wtmp: if wtmp != "MM" { Some(wtmp.to_string())} else {None},
                dewp: if dewp != "MM" { Some(dewp.to_string())} else {None},
                vis: None,
                ptdy: if ptdy != "MM" { Some(ptdy.to_string())} else {None},
                tide: None,
            }
        )
        .collect();

    Ok(res)
}

pub async fn get_station_realtime_cwind_data(
    station: &str,
) -> Result<Vec<StationContinuousWindsData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.
    // This only collects data for stationary buoys, there is a separate function to grab drifting buoy stdmet sensor data.
    debug!("get_station_realtime_cwind_data");

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/data/realtime2/"
        + station.to_uppercase().as_str()
        + ".cwind";
    
    let re = Regex::new(
        &(r"([0-9M\+\.-]+)[\s\n]+".repeat(10)),
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gdr, gst, _gtime])| StationContinuousWindsData {
            station: station.to_string().to_uppercase(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
            wdir: if !check_null_string(wdir) { Some(wdir.to_string()) } else {None},
            wspd: if !check_null_string(wspd) { Some(wspd.to_string())} else {None},
            gdr: if !check_null_string(gdr) { Some(gdr.to_string())} else {None},
            gst: if !check_null_string(gst) { Some(gst.to_string())} else {None},
        })
        .collect();

    Ok(res)
}