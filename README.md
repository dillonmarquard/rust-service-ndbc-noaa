# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Dependencies

* Rust ^1.81

### Endpoints (current state)

* station
    * metadata for all active stations
* station/stdmet
    * metadata for all active standard meteorological stations
* station/currents
    * metadata for all active standard meteorological stations
* station/{id}
    * metadata for a specific station
* station/{id}/{year}
    * historic sensor data for the specified station
* station/{id}/realtime
    * realtime (last 45 days) sensor data for the specified station

### Endpoints (future state)

* /station
   * metadata for all active stations
* /station/{id}
    * metadata for a specific station
    * historic data available for download? (future consideration)
        * `get_station_available_downloads(...)` can be used to pull only a specific station's historic data 
* /station/{id}/stdmet (future consideration)
    * all historic Standard Meteorological sensor data for the specified station
* /station/{id}/stdmet/{year} (migration)
    * historic Standard Meteorological sensor data for the specified station and year
    * if no year is filtered it will provide all historic data
* /station/{id}/stdmet/realtime (migration)
    * realtime (last 45 days) sensor data for the specified station

* filters
    * available data eg. `/station?available=stdmet` or `/station?available=currents`
    * distance from point (lat + lon) eg. `/station?lat=10&lon=4&dist=1`

#### Future Considerations
* I plan on consolidating the endpoints to be more user friendly for more application driven use-cases
* Currently, the endpoints were made to make previously inaccessible data available through an API, but actual usage is not wholly intuitive

#### To-do
* migrate the API to a more intuitive structure and provide QOL features not present in the raw interface

### Deployment
#### Docker
The core webserver for any deployment.
``` bash
docker build -t rust-service-ndbc-noaa .
docker run -it -p 8081:80 rust-service-ndbc-noaa
```

#### Kubernetes
Local deployment using k3d to create a small cluster.
``` bash
k3d cluster delete demo-cluster
docker build -t rust-service-ndbc-noaa .
k3d cluster create -p "3001:80@loadbalancer" --agents 2 --wait demo-cluster
k3d image import rust-service-ndbc-noaa -c demo-cluster
kubectl apply -f deployment.yaml
```

