// https://www.ndbc.noaa.gov/docs/ndbc_web_data_guide.pdf

use chrono::{
    prelude::{DateTime, Utc},
    NaiveDateTime,
};
use serde::Deserialize;

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
pub struct StationsMetadataResponse {
    pub created: DateTime<Utc>,
    #[serde(alias = "station")]
    pub stations: Vec<StationMetadata>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationFile {
    pub filename: String,
    pub station: String,
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
    pub fn as_str(&self) -> &'static str {
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

#[derive(Debug, Deserialize, Clone)]
pub struct StationStdMetData {
    pub station: String,
    pub timestamp: NaiveDateTime,
    pub wdir: String,
    pub wspd: String,
    pub gst: String,
    pub wvht: String,
    pub dpd: String,
    pub apd: String,
    pub mwd: String,
    pub pres: String,
    pub atmp: String,
    pub wtmp: String,
    pub dewp: String,
    pub vis: String,
    pub tide: String,
}
