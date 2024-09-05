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
        r"([0-9a-zA-Z\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)[\s]+([0-9\.-]+)\n",
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [year, month, day, hour, minute, wdir, wspd, gst, wvht, dpd, apd, mwd, pres, atmp, wtmp, dewp, vis, tide])| StationStdMetData {
            station: station.to_string(),
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%y/%m/%d %H:%M").unwrap(),
            wdir: wdir.to_string(),
            wspd: wspd.to_string(),
            gst: gst.to_string(),
            wvht: wvht.to_string(),
            dpd: dpd.to_string(),
            apd: apd.to_string(),
            mwd: mwd.to_string(),
            pres: pres.to_string(),
            atmp: atmp.to_string(),
            wtmp: wtmp.to_string(),
            dewp: dewp.to_string(),
            vis: vis.to_string(),
            tide: tide.to_string()
        })
        .collect();

    Ok(res)
}
