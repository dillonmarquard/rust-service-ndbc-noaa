use crate::ndbc::ndbc_schema::{
    get_active_stations, get_station_available_history, get_stations_metadata, StationDataType,
};
use futures::{stream, StreamExt, TryStreamExt};
use ndbc::ndbc_schema::{ActiveStationsMetadataResponse, StationHistorySTDMET, StationMetadata};
use rand::distributions::{Distribution, Uniform};
use std::{thread, time};
use tokio;

pub mod ndbc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let active_stations = get_active_stations().await?;
    // let res = &active_stations.stations[0];

    // println!("{res:#?}");

    let station_metadata = get_stations_metadata().await?;

    let stations_available_history = stream::iter(station_metadata.stations[0..25].iter().map(
        |station| async {
            let mut rng = rand::thread_rng();
            let distribution = Uniform::from(0..500);
            let delay = distribution.sample(&mut rng);
            // let id = &station.id;
            // println!("id: {id:?} delay: {delay:?}");
            thread::sleep(time::Duration::from_millis(delay));
            get_station_available_history(&station.id, StationDataType::StandardMeteorological)
                .await
        },
    ))
    .buffer_unordered(10)
    .try_collect::<Vec<Vec<StationHistorySTDMET>>>()
    .await?;

    println!("{stations_available_history:#?}");

    // println!("{ids:#?}");
    // let res = &station_metadata.stations[0];
    // println!("{res:#?}");
    // let res: Vec<StationMetadata> = station_metadata.stations
    //     .iter()
    //     .filter(|&station| station.id == "42040")
    //     .cloned()
    //     .collect();

    // println!("{res:#?}");

    // println!("{station_metadata:#?}");

    // let res = get_station_stdmet_historical_data("0y2w3").await?;
    // println!("{res:#?}");

    Ok(())
}
