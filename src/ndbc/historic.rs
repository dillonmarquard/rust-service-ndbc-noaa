use chrono::NaiveDateTime;
use regex::Regex;
use serde_xml_rs::from_str;

use super::ndbc_schema::{
    StationDataType, StationFile, StationMetadata, StationStdMetData, StationsMetadataResponse,
};

pub async fn get_stations_metadata() -> Result<Vec<StationMetadata>, Box<dyn std::error::Error>> {
    // This function returns the historical station metadata back to 2000 for all stations on the NDBC.

    let url: &str = "https://www.ndbc.noaa.gov/metadata/stationmetadata.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<StationsMetadataResponse>(body.as_str()).unwrap();

    Ok(res.stations)
}

pub async fn get_station_available_downloads(
    station: &str,
    data_type: StationDataType,
) -> Result<Vec<StationFile>, Box<dyn std::error::Error>> {
    // This function returns a list of historic files for the given station and data_type (eg. stdmet, cwind, swden)
    // Please use get_datatype_historic_files and filter the desired stations to avoid spamming the resource.

    let url: String = format!("https://www.ndbc.noaa.gov/station_history.php?station={station}");
    let re = Regex::new(
        ("".to_string()
            + r###"<a href="/download_data\.php\?filename=(.{5,25})&amp;dir=data/historical/"###
            + data_type.as_str()
            + r###"/">(.{1,6})</a>"###)
            .as_str(),
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [f, y])| StationFile {
            filename: f.to_string(),
            station: f[0..=4].to_string(),
            year: y.to_string(),
        })
        .collect();

    Ok(res)
}

pub async fn get_datatype_historic_files(
    data_type: StationDataType,
) -> Result<Vec<StationFile>, Box<dyn std::error::Error>> {
    // This function returns a list of all downloadable historic files for a specified data_type (eg. stdmet, cwind, swden)

    let url: String =
        "".to_string() + "https://www.ndbc.noaa.gov/data/historical/" + data_type.as_str();
    let re = Regex::new(
        r###"<tr><td valign="top"><img src="/icons/compressed.gif" alt="\[   \]"></td><td><a href="(.{5,50})">(.{5,50})</a></td><td align="right">(.{5,50})  </td><td align="right"> (.{1,50})</td><td>&nbsp;</td></tr>"###,
    )
    .unwrap();

    let body = reqwest::get(url).await?.text().await?;

    let res = re
        .captures_iter(&body)
        .map(|c| c.extract())
        .map(|(_, [f, _, _, _])| StationFile {
            filename: f.to_string(),
            station: f[0..=4].to_string(),
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

    let url: String = "".to_string()
        + "https://www.ndbc.noaa.gov/view_text_file.php?filename="
        + station
        + "h"
        + year
        + ".txt.gz"
        + "&dir=data/historical/"
        + StationDataType::StandardMeteorological.as_str()
        + "/";
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
            timestamp: NaiveDateTime::parse_from_str(("".to_string() + year + "-" + month + "-" + day + " " + hour + ":" + minute).as_str(), "%Y-%m-%d %H:%M").unwrap(),
            wdir: if wdir != "999" { Some(wdir.to_string()) } else {None} ,
            wspd: if wspd != "99.0" { Some(wspd.to_string())} else {None},
            gst: if gst != "99.0" { Some(gst.to_string())} else {None},
            wvht: if wvht != "99.00" { Some(wvht.to_string())} else {None},
            dpd: if dpd != "99.00" { Some(dpd.to_string())} else {None},
            apd: if apd != "99.00" { Some(apd.to_string())} else {None},
            mwd: if mwd != "999" { Some(mwd.to_string())} else {None},
            pres: if pres != "9999.0" { Some(pres.to_string())} else {None},
            atmp: if atmp != "999.0" { Some(atmp.to_string())} else {None},
            wtmp: if wtmp != "99.0" { Some(wtmp.to_string())} else {None}, // unsure of null value
            dewp: if dewp != "999.0" { Some(dewp.to_string())} else {None},
            vis: if vis != "99.0" { Some(vis.to_string())} else {None},
            tide: if tide != "99.0" { Some(tide.to_string())} else {None}
        })
        .collect();

    Ok(res)
}

// This is a streaming iterator to process batches of stdmet files from a list of stations.
// This was deprecated because historic::get_datatype_historic_files which pulls all files for a given data_type by parsing the directory.
// Code will be useful for async pulling the raw data from a list of files

// let station_metadata = get_stations_metadata().await?;
// let stations_available_history = stream::iter(station_metadata.stations.iter().map(
//     |station| async {
//         let mut rng = rand::thread_rng();
//         let distribution = Uniform::from(0..500);
//         let delay = distribution.sample(&mut rng);
//         // let id = &station.id;
//         // println!("id: {id:?} delay: {delay:?}");
//         thread::sleep(time::Duration::from_millis(delay));
//         get_station_available_history(&station.id, StationDataType::StandardMeteorological)
//             .await
//     },
// ))
// .buffer_unordered(10)
// .try_collect::<Vec<Vec<StationFile>>>()
// .await?;

// println!("{stations_available_history:#?}");
