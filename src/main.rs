use ndbc::{
    historic::{get_datatype_historic_files, get_station_historical_stdmet_data},
    ndbc_schema::{Station, StationDataType, StationFile},
    realtime::{get_active_stations, get_station_realtime_stdmet_data},
};

use tokio;

pub mod ndbc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get all active stations
    // historic sync of stdmet data to db
    // (daily, 30-day) realtime sync to db
    // analysis on-top of db
    // -------------------------------------------------------------------------

    let active_stations = get_active_stations().await?;
    let active_met_stations: Vec<&Station> = active_stations
        .iter()
        .filter(|&s| s.met.as_ref().is_some_and(|m| m == "y"))
        .collect();

    println!("{active_met_stations:#?}");

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

    let files = get_datatype_historic_files(StationDataType::StandardMeteorological).await?;
    println!("{files:#?}");

    println!("");

    let tmp: Vec<&StationFile> = files.iter().filter(|&f| f.year == "2019").collect();
    let tmp_sf = tmp[0];
    let res = get_station_historical_stdmet_data(&tmp_sf.station, &tmp_sf.year).await?;
    println!("{res:#?}");
    // -------------------------------------------------------------------------

    let tmp_sf = active_met_stations[0];
    let res = get_station_realtime_stdmet_data(&tmp_sf.id).await?;
    println!("{res:#?}");

    Ok(())
}
