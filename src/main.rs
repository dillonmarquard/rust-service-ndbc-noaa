pub mod ndbc;

use actix_web::{get, web, App, HttpServer, Responder};
use log::{info, warn};
use ndbc::{
    historic::get_station_historical_stdmet_data,
    ndbc_schema::Station,
    realtime::{get_active_stations, get_station_realtime_stdmet_data},
};

#[get("/station")]
async fn service_active_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_active_stations");
    let active_stations = get_active_stations().await?;

    if active_stations.is_empty() {
        warn!("No active stations were found");
    }

    Ok(web::Json(active_stations))
}

#[get("/station/stdmet")]
async fn service_active_stdmet_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_active_stdmet_stations");
    let active_stations = get_active_stations().await?;
    let active_stdmet_stations: Vec<Station> = active_stations
        .into_iter()
        .filter(|s| s.met.is_some_and(|x| x))
        .collect();

    if active_stdmet_stations.is_empty() {
        warn!("No active stdmet stations were found");
    }

    Ok(web::Json(active_stdmet_stations))
}

#[get("/station/{id}")]
async fn service_station_metadata(
    path: web::Path<String>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    info!("service_station_stdmet_historic_data");

    let id= path.into_inner();
    let active_stations = get_active_stations().await?;
    let active_stdmet_stations: Vec<Station> = active_stations
        .into_iter()
        .filter(|s| s.id == id)
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
    let id = path.into_inner();
    let res = get_station_realtime_stdmet_data(&id).await?;

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
    let res = get_station_historical_stdmet_data(&id, &year).await?;

    if res.is_empty() {
        warn!("No stdmet data was found for the station: {id} for the year of {year}");
    }

    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting `surf_quality_estimator` API");

    HttpServer::new(|| {
        App::new()
            .service(service_active_stations)
            .service(service_active_stdmet_stations)
            .service(service_station_metadata)
            .service(service_station_stdmet_realtime_data) // pattern match takes order from service declaration
            .service(service_station_stdmet_historic_data) // overlapping patterns should be ordered with special routes first eg. /station/ABC/realtime then /station/ABC/2023
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await?;

    Ok(())
}
