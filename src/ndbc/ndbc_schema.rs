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
        Some(h) => match h.as_str() {
            "y" => Ok(Some(true)),
            "n" => Ok(Some(false)),
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

fn deserialize_string_upper<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;

    Ok(s.to_uppercase())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Station {
    #[serde(default, deserialize_with = "deserialize_string_upper")]
    pub id: String,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub elev: Option<f32>,
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
    pub stdmet_history: Option<Vec<StationHistoricFile>>,
    pub cwind_history: Option<Vec<StationHistoricFile>>,
    pub stdmet_realtime: Option<Vec<StationRealtimeFile>>,
    pub cwind_realtime: Option<Vec<StationRealtimeFile>>,
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
pub struct StationHistoricFile {
    pub filename: String,
    pub station: String,
    pub data_type: StationDataType,
    pub year: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationRealtimeFile {
    pub filename: String,
    pub station: String,
    pub data_type: StationDataType,
    pub timestamp: NaiveDateTime,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum StationDataType {
    StandardMeteorological,
    ContinuousWinds,
    OceanCurrent,
    SpectralWaveDensity,
    SpectralWaveA1Density,
    SpectralWaveA2Density,
    SpectralWaveR1Density,
    SpectralWaveR2Density,
    SolarRadiation,
    Unsupported,
}

impl StationDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StationDataType::StandardMeteorological => "stdmet",
            StationDataType::ContinuousWinds => "cwind",
            StationDataType::OceanCurrent => "adcp",
            StationDataType::SpectralWaveDensity => "swden",
            StationDataType::SpectralWaveA1Density => "swdir",
            StationDataType::SpectralWaveA2Density => "swdir2",
            StationDataType::SpectralWaveR1Density => "swr1",
            StationDataType::SpectralWaveR2Density => "swr2",
            StationDataType::SolarRadiation => "srad",
            StationDataType::Unsupported => "unsupported",
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
    pub ptdy: Option<String>,
    pub tide: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationContinuousWindsData {
    pub station: String,
    pub timestamp: NaiveDateTime,
    pub wdir: Option<String>,
    pub wspd: Option<String>,
    pub gdr: Option<String>,
    pub gst: Option<String>,
}

pub fn check_null_string(value: &str) -> bool {
    // This function may not be correct such that different attributes have different null values, but the null values are not shared between all of them.
    match value {
        "9" => true,
        "9.0" => true,
        "9.00" => true,
        "9.000" => true,
        "99" => true,
        "99.0" => true,
        "99.00" => true,
        "99.000" => true,
        "999" => true,
        "999.0" => true,
        "999.00" => true,
        "999.000" => true,
        "9999" => true,
        "9999.0" => true,
        "9999.00" => true,
        "9999.000" => true,
        "M" => true,
        "MM" => true,
        "MMM" => true,
        "MMMM" => true,
        _ => false,
    }
}
