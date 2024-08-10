use serde_xml_rs::from_str;

use super::ndbc_schema::ActiveStationsResponse;

pub async fn get_active_stations() -> Result<ActiveStationsResponse, Box<dyn std::error::Error>> {
    // This function returns a list of active stations.

    let url: &str = "https://www.ndbc.noaa.gov/activestations.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsResponse>(body.as_str()).unwrap();

    Ok(res)
}

pub async fn get_station_realtime_data(station: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This function returns a list of active stations.

    Ok(())
}
