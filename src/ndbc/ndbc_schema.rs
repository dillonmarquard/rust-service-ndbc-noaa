// https://www.ndbc.noaa.gov/docs/ndbc_web_data_guide.pdf

use chrono::{
    prelude::{DateTime, Utc},
    NaiveDateTime,
};
use serde::{de, Deserialize, Serialize};

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: Option<String> = de::Deserialize::deserialize(deserializer).unwrap_or(None);

    match s {
        Some(h) => {
            match h.as_str() {
                "y" => Ok(Some(true)),
                "n" => Ok(Some(false)),
                _ => Ok(None),
            }
        },
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Station {
    pub id: String,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub elev: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub pgm: Option<String>,
    pub r#type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub met: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub currents: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub waterquality: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub dart: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActiveStationsResponse {
    pub created: DateTime<Utc>,
    pub count: Option<String>,
    #[serde(alias = "station")]
    pub stations: Vec<Station>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationMetadata {
    pub id: String,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub pgm: Option<String>,
    pub r#type: Option<String>,
    pub history: Vec<StationMetadataHistory>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationsMetadataResponse {
    pub created: DateTime<Utc>,
    #[serde(alias = "station")]
    pub stations: Vec<StationMetadata>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationStdMetData {
    pub station: String,
    pub timestamp: NaiveDateTime,
    pub wdir: Option<String>,
    pub wspd: Option<String>,
    pub gst: Option<String>,
    pub wvht: Option<String>,
    pub dpd: Option<String>,
    pub apd: Option<String>,
    pub mwd: Option<String>,
    pub pres: Option<String>,
    pub atmp: Option<String>,
    pub wtmp: Option<String>,
    pub dewp: Option<String>,
    pub vis: Option<String>,
    pub tide: Option<String>,
}
