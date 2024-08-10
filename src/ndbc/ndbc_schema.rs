// https://www.ndbc.noaa.gov/docs/ndbc_web_data_guide.pdf

use chrono::prelude::{DateTime, Utc};
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    pub id: String,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub elev: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub pgm: Option<String>,
    pub r#type: Option<String>,
    pub met: Option<String>,
    pub currents: Option<String>,
    pub waterquality: Option<String>,
    pub dart: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActiveStationsResponse {
    pub created: DateTime<Utc>,
    pub count: Option<String>,
    #[serde(alias = "station")]
    pub stations: Vec<Station>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationMetadataHistory {
    pub start: Option<String>,
    pub stop: Option<String>,
    pub lat: Option<String>,
    pub lng: Option<String>,
    pub elev: Option<String>,
    pub met: Option<String>,
    pub hull: Option<String>,
    pub anemom_height: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationMetadata {
    pub id: String,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub pgm: Option<String>,
    pub r#type: Option<String>,
    pub history: Vec<StationMetadataHistory>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActiveStationsMetadataResponse {
    pub created: DateTime<Utc>,
    #[serde(alias = "station")]
    pub stations: Vec<StationMetadata>,
}

pub async fn get_active_stations() -> Result<ActiveStationsResponse, Box<dyn std::error::Error>> {
    let url: &str = "https://www.ndbc.noaa.gov/activestations.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsResponse>(body.as_str()).unwrap();

    Ok(res)
}

pub async fn get_stations_metadata(
) -> Result<ActiveStationsMetadataResponse, Box<dyn std::error::Error>> {
    let url: &str = "https://www.ndbc.noaa.gov/metadata/stationmetadata.xml";

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsMetadataResponse>(body.as_str()).unwrap();

    Ok(res)
}

pub async fn get_station_realtime_data(
    station: &str,
) -> Result<ActiveStationsMetadataResponse, Box<dyn std::error::Error>> {
    let url: String = format!("https://www.ndbc.noaa.gov/data/realtime2/{station}.spec");

    let body = reqwest::get(url).await?.text().await?;

    let res = from_str::<ActiveStationsMetadataResponse>(body.as_str()).unwrap();

    Ok(res)
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationHistorySTDMET {
    pub filename: String,
    pub year: String,
}

pub enum StationDataType {
    StandardMeteorological,
    ContinuousWinds,
    SpectralWaveDensity,
    SpectralWaveA1Density,
    SpectralWaveA2Density,
    SpectralWaveR1Density,
    SpectralWaveR2Density,
    SolarRadiation,
}
impl StationDataType {
    fn as_str(&self) -> &'static str {
        match self {
            StationDataType::StandardMeteorological => "stdmet",
            StationDataType::ContinuousWinds => "cwind",
            StationDataType::SpectralWaveDensity => "swden",
            StationDataType::SpectralWaveA1Density => "swdir",
            StationDataType::SpectralWaveA2Density => "swdir2",
            StationDataType::SpectralWaveR1Density => "swr1",
            StationDataType::SpectralWaveR2Density => "swr2",
            StationDataType::SolarRadiation => "srad",
        }
    }
}

pub async fn get_station_available_history(
    station: &str,
    data_type: StationDataType,
) -> Result<Vec<StationHistorySTDMET>, Box<dyn std::error::Error>> {
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
        .map(|(_, [f, y])| StationHistorySTDMET {
            filename: f.to_string(),
            year: y.to_string(),
        })
        .collect();

    Ok(res)
}
