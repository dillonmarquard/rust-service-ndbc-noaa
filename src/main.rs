use ndbc::{
    historic::get_station_historical_stdmet_data,
    ndbc_schema::Station,
    realtime::{get_active_stations, get_station_realtime_stdmet_data},
};

use log::info;

pub mod ndbc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Starting `surf_quality_estimator` API");

    // get all active stations
    // historic sync of stdmet data to db
    // (daily, 30-day) realtime sync to db
    // analysis on-top of db
    // -------------------------------------------------------------------------

    // let active_stations = get_active_stations().await?;
    // let active_stdmet_stations: Vec<Station> = active_stations
    //     .into_iter()
    //     .filter(|s| s.met.as_ref().is_some_and(|&m| m == true ))
    //     .collect();

    // println!("{active_stdmet_stations:#?}");

    // -------------------------------------------------------------------------

    // let station_metadata = get_stations_metadata().await?;

    // let stations_available_history = stream::iter(station_metadata.stations[0..25].iter().map(
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
    // .try_collect::<Vec<Vec<StationFileStdMet>>>()
    // .await?;

    // println!("{stations_available_history:#?}");

    // -------------------------------------------------------------------------

    // let res = &station_metadata.stations[0];
    // println!("{res:#?}");
    // let res: Vec<StationMetadata> = station_metadata.stations
    //     .iter()
    //     .filter(|&station| station.id == "42040")
    //     .cloned()
    //     .collect();

    // println!("{res:#?}");

    // println!("{station_metadata:#?}");

    // -------------------------------------------------------------------------

    // let res = get_station_stdmet_historical_data("0y2w3").await?;
    // println!("{res:#?}");

    // -------------------------------------------------------------------------

    // let files = get_historic_files(StationDataType::StandardMeteorological).await?;
    // println!("{files:#?}");

    // let tmp: Vec<StationFile> = files.into_iter().filter(|f| f.year == "2023").collect();
    // let tmp_sf = &tmp[0];
    // let res = get_station_historical_stdmet_data(&tmp_sf.station, &tmp_sf.year).await?;
    // println!("{res:#?}");
    
    // let res = get_station_historical_stdmet_data("42040", "2023").await?;
    // println!("{res:#?}");
    
    // // -------------------------------------------------------------------------

    // let tmp_sf = &active_stdmet_stations[0];
    // let res = get_station_realtime_stdmet_data(&tmp_sf.id).await?;
    // println!("{res:#?}");

    Ok(())
}
