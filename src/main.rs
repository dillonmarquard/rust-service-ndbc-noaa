mod ndbc;

use actix_web::{get, web, App, HttpServer, Responder};
use log::{info, warn};
use ndbc::{
    historic::{self, get_historic_files, get_station_historical_stdmet_data},
    ndbc_schema::{Station, StationDataType, StationFile},
    realtime::{get_active_stations, get_station_realtime_stdmet_data},
};

#[get("/station")]
async fn service_active_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_active_stations");
    let active_stations: Vec<Station> = get_active_stations().await?;

    if active_stations.is_empty() {
        warn!("No active stations were found");
    }

    Ok(web::Json(active_stations))
}

#[get("/station/stdmet")]
async fn service_active_stdmet_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_active_stdmet_stations");
    let active_stations: Vec<Station> = get_active_stations().await?;
    let active_stdmet_stations: Vec<Station> = active_stations
        .into_iter()
        .filter(|s: &Station| s.met.is_some_and(|x: bool| x))
        .collect();

    if active_stdmet_stations.is_empty() {
        warn!("No active stdmet stations were found");
    }

    Ok(web::Json(active_stdmet_stations))
}

#[get("/station/currents")]
async fn service_active_currents_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_active_currents_stations");
    let active_stations: Vec<Station> = get_active_stations().await?;
    let active_currents_stations: Vec<Station> = active_stations
        .into_iter()
        .filter(|s: &Station| s.currents.is_some_and(|x: bool| x))
        .collect();

    if active_currents_stations.is_empty() {
        warn!("No active currents stations were found");
    }

    Ok(web::Json(active_currents_stations))
}

#[get("/history/stdmet")]
async fn service_historic_stdmet_files() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let historic_files: Vec<StationFile> = get_historic_files(StationDataType::StandardMeteorological).await?;

    Ok(web::Json(historic_files))
}

#[get("/station/{id}")]
async fn service_station_metadata(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_station_stdmet_historic_data");

    let id: String= path.into_inner();
    let active_stations: Vec<Station> = get_active_stations().await?;
    let active_stdmet_stations: Vec<Station> = active_stations
        .into_iter()
        .filter(|s: &Station| s.id == id)
        .collect();

    if active_stdmet_stations.is_empty() {
        warn!("No metadata was found for station: {id}");
    }

    Ok(web::Json(active_stdmet_stations))
}

#[get("/station/{id}/realtime")]
async fn service_station_stdmet_realtime_data(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_station_stdmet_realtime_data");
    let id: String = path.into_inner();
    let res: Vec<ndbc::ndbc_schema::StationStdMetData> = get_station_realtime_stdmet_data(&id).await?;

    if res.is_empty() {
        warn!("No realtime stdmet data was found for the station: {id}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/{year}")]
async fn service_station_stdmet_historic_data(
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_station_stdmet_historic_data");
    let (id, year) = path.into_inner();
    let res: Vec<ndbc::ndbc_schema::StationStdMetData> = get_station_historical_stdmet_data(&id, &year).await?;

    if res.is_empty() {
        warn!("No stdmet data was found for the station: {id} for the year of {year}");
    }

    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting `rust-service-ndbc-noaa` API");

    HttpServer::new(|| {
        App::new()
            .service(service_active_stations)
            .service(service_active_stdmet_stations)
            .service(service_active_currents_stations)
            .service(service_station_metadata)
            .service(service_historic_stdmet_files)
            .service(service_station_stdmet_realtime_data) // pattern match takes order from service declaration
            .service(service_station_stdmet_historic_data) // overlapping patterns should be ordered with special routes first (eg. /station/ABC/realtime vs. /station/ABC/2023)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await?;

    Ok(())
}
