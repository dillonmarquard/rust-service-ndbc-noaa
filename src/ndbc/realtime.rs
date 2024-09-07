use chrono::NaiveDateTime;
use regex::Regex;
use serde_xml_rs::from_str;

use super::ndbc_schema::{ActiveStationsResponse, Station, StationStdMetData};

pub async fn get_active_stations() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    // This function returns a list of active stations.
    // just because a station is active does not mean it has stdmet data.

    let url: &str = "https://www.ndbc.noaa.gov/activestations.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsResponse>(body.as_str()).unwrap();

    Ok(res.stations)
}

pub async fn get_station_realtime_stdmet_data(
    station: &str,
) -> Result<Vec<StationStdMetData>, Box<dyn std::error::Error>> {
    // This function returns the raw stdmet sensor data for a given station over the last 45 days.

    let url: String =
        "".to_string() + "https://www.ndbc.noaa.gov/data/realtime2/" + station + ".txt";
    let re = Regex::new(
        r"([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)[\s]+([0-9M\+\.-]+)\n",
    )
    .unwrap();

    println!("{url:?}");

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, _ptdy, tide])| 
            StationStdMetData {
                station: station.to_string(),
                timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
                wdir: if wdir != "MM" { Some(wdir.to_string()) } else {None} ,
                wspd: if wspd != "MM" { Some(wspd.to_string())} else {None},
                gst: if gst != "MM" { Some(gst.to_string())} else {None},
                wvht: if wvht != "MM" { Some(wvht.to_string())} else {None},
                dpd: if dpd != "MM" { Some(dpd.to_string())} else {None},
                apd: if apd != "MM" { Some(apd.to_string())} else {None},
                mwd: if mwd != "MM" { Some(mwd.to_string())} else {None},
                pres: if pres != "MM" { Some(pres.to_string())} else {None},
                atmp: if atmp != "MM" { Some(atmp.to_string())} else {None},
                wtmp: if wtmp != "MM" { Some(wtmp.to_string())} else {None},
                dewp: if dewp != "MM" { Some(dewp.to_string())} else {None},
                vis: if vis != "MM" { Some(vis.to_string())} else {None},
                tide: if tide != "MM" { Some(tide.to_string())} else {None}
            }
        )
        .collect();

    Ok(res)
}
