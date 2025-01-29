# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Dependencies

* Rust ^1.81

### Endpoints

* /station
   * metadata for all active stations
* /station/{id}
    * metadata for a specific station
    * historic stdmet data available for download
* /station/{id}/stdmet (future consideration)
    * all historic Standard Meteorological sensor data for the specified station
* /station/{id}/stdmet/{year}
    * historic Standard Meteorological sensor data for the specified station and year
* /station/{id}/stdmet/realtime
    * realtime (last 45 days) stdmet sensor data for the specified station
* /station/{id}/stdmetdrift/realtime
    * realtime (last 45 days) stdmet drift sensor data for the specified station
    * drifting buoys do not provide stdmet data in the same format
    * other realtime formats will be supported in the future eg. ocean

* filters (future consideration)
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

