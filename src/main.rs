mod ndbc;

use actix_web::{get, web, App, HttpServer, Responder};
use log::debug;
use ndbc::{
    historic::{get_historic_files, get_station_available_downloads, get_station_historical_cwind_data, get_station_historical_stdmet_data},
    ndbc_schema::{Station, StationDataType, StationHistoricFile, StationRealtimeFile,StationStdMetData, StationContinuousWindsData, StationSpectralWaveSummary},
    realtime::{get_active_stations, get_realtime_files, get_station_realtime_cwind_data, get_station_realtime_spec_data, get_station_realtime_stdmet_data, get_station_realtime_stdmetdrift_data},
};

#[get("/station")]
async fn service_active_stations() -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_active_stations");
    let active_stations: Vec<Station> = get_active_stations().await?;

    if active_stations.is_empty() {
        debug!("No active stations were found");
    }

    let mut historic_data = get_historic_files(StationDataType::StandardMeteorological).await?;

    let mut enhanced_stations: Vec<Station> = active_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationHistoricFile> = historic_data.iter().filter(|d: &&StationHistoricFile| d.station == s.id).map(|x: &StationHistoricFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No historic data was found for station: {}", &s.id);
            } else {
                s.stdmet_history = Some(tmp);
            }

            s
        })
        .collect();

    historic_data = get_historic_files(StationDataType::ContinuousWinds).await?;

    enhanced_stations = enhanced_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationHistoricFile> = historic_data.iter().filter(|d: &&StationHistoricFile| d.station == s.id).map(|x: &StationHistoricFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No historic data was found for station: {}", &s.id);
            } else {
                s.cwind_history = Some(tmp);
            }

            s
        })
        .collect();

    let stdmet_realtime: Vec<StationRealtimeFile> = get_realtime_files(StationDataType::StandardMeteorological).await?;

    enhanced_stations = enhanced_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = stdmet_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == s.id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime data was found for station: {}", &s.id);
            } else {
                s.stdmet_realtime = Some(tmp);
            }

            s
        })
        .collect();

    let cwind_realtime = get_realtime_files(StationDataType::ContinuousWinds).await?;

    enhanced_stations = enhanced_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = cwind_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == s.id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime data was found for station: {}", &s.id);
            } else {
                s.cwind_realtime = Some(tmp);
            }

            s
        })
        .collect();

    let spec_realtime = get_realtime_files(StationDataType::SpectralWaveSummary).await?;

    enhanced_stations = enhanced_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = spec_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == s.id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime data was found for station: {}", &s.id);
            } else {
                s.spec_realtime = Some(tmp);
            }

            s
        })
        .collect();

    Ok(web::Json(enhanced_stations))
}

#[get("/station/{id}")]
async fn service_station_metadata(path: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_stdmet_historic_data");

    let id: String = path.into_inner();
    let active_stations: Vec<Station> = get_active_stations().await?;
    let mut active_stdmet_stations: Vec<Station> = active_stations.into_iter().filter(|s: &Station| s.id == id).collect();

    let historic_stdmet_data: Vec<StationHistoricFile> = get_station_available_downloads(&id, StationDataType::StandardMeteorological).await?;

    let historic_cwind_data: Vec<StationHistoricFile> = get_station_available_downloads(&id, StationDataType::ContinuousWinds).await?;

    let stdmet_realtime: Vec<StationRealtimeFile> = get_realtime_files(StationDataType::StandardMeteorological).await?;

    active_stdmet_stations = active_stdmet_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = stdmet_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime stdmet data was found for station: {}", &s.id);
            } else {
                s.stdmet_realtime = Some(tmp);
            }

            s
        })
        .collect();

    let cwind_realtime = get_realtime_files(StationDataType::ContinuousWinds).await?;

    active_stdmet_stations = active_stdmet_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = cwind_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == s.id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime cwind data was found for station: {}", &s.id);
            } else {
                s.cwind_realtime = Some(tmp);
            }

            s
        })
        .collect();

    let spec_realtime = get_realtime_files(StationDataType::SpectralWaveSummary).await?;

    active_stdmet_stations = active_stdmet_stations
        .into_iter()
        .map(|mut s: Station| {
            let tmp: Vec<StationRealtimeFile> = spec_realtime.iter().filter(|d: &&StationRealtimeFile| d.station == s.id).map(|x: &StationRealtimeFile| x.clone()).collect();

            if tmp.is_empty() {
                debug!("No realtime spec data was found for station: {}", &s.id);
            } else {
                s.spec_realtime = Some(tmp);
            }

            s
        })
        .collect();

    if active_stdmet_stations.is_empty() {
        debug!("No metadata was found for station: {id}");
    }

    if historic_stdmet_data.is_empty() {
        debug!("No historic stdmet data was found for station: {id}");
    } else {
        active_stdmet_stations[0].stdmet_history = Some(historic_stdmet_data);
    }

    if historic_cwind_data.is_empty() {
        debug!("No historic cwind data was found for station: {id}");
    } else {
        active_stdmet_stations[0].cwind_history = Some(historic_cwind_data);
    }

    Ok(web::Json(active_stdmet_stations))
}

#[get("/station/{id}/stdmet/{year}")]
async fn service_station_stdmet_historic_data(path: web::Path<(String, String)>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_stdmet_historic_data");
    let (id, year) = path.into_inner();
    let res: Vec<StationStdMetData> = get_station_historical_stdmet_data(&id, &year).await?;

    if res.is_empty() {
        debug!("No stdmet data was found for the station: {id} for the year of {year}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/cwind/{year}")]
async fn service_station_cwind_historic_data(path: web::Path<(String, String)>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_cwind_historic_data");
    let (id, year) = path.into_inner();
    let res: Vec<StationContinuousWindsData> = get_station_historical_cwind_data(&id, &year).await?;

    if res.is_empty() {
        debug!("No cwind data was found for the station: {id} for the year of {year}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/stdmet/realtime")]
async fn service_station_stdmet_realtime_data(path: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_stdmet_realtime_data");
    let id: String = path.into_inner();
    let res: Vec<StationStdMetData> = get_station_realtime_stdmet_data(&id).await?;

    if res.is_empty() {
        debug!("No realtime stdmet data was found for the station: {id}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/stdmetdrift/realtime")]
async fn service_station_stdmetdrift_realtime_data(path: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_stdmetdrift_realtime_data");
    let id: String = path.into_inner();
    let res: Vec<StationStdMetData> = get_station_realtime_stdmetdrift_data(&id).await?;

    if res.is_empty() {
        debug!("No realtime stdmetdrift data was found for the station: {id}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/cwind/realtime")]
async fn service_station_cwind_realtime_data(path: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_cwind_realtime_data");
    let id: String = path.into_inner();
    let res: Vec<StationContinuousWindsData> = get_station_realtime_cwind_data(&id).await?;

    if res.is_empty() {
        debug!("No realtime cwind data was found for the station: {id}");
    }

    Ok(web::Json(res))
}

#[get("/station/{id}/spec/realtime")]
async fn service_station_spec_realtime_data(path: web::Path<String>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    debug!("service_station_spec_realtime_data");
    let id: String = path.into_inner();
    let res: Vec<StationSpectralWaveSummary> = get_station_realtime_spec_data(&id).await?;

    if res.is_empty() {
        debug!("No realtime spec data was found for the station: {id}");
    }

    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    debug!("Starting `rust-service-ndbc-noaa` API");

    HttpServer::new(|| {
        App::new()
            .service(service_active_stations)
            .service(service_station_metadata)
            .service(service_station_stdmet_realtime_data) // pattern match takes order from service declaration
            .service(service_station_stdmetdrift_realtime_data)
            .service(service_station_stdmet_historic_data) // overlapping patterns should be ordered with special routes first (eg. /station/ABC/realtime vs. /station/ABC/2023)
            .service(service_station_cwind_realtime_data)
            .service(service_station_cwind_historic_data)
            .service(service_station_spec_realtime_data)
    })
    .bind(("0.0.0.0", 2048))?
    .run()
    .await?;

    Ok(())
}
