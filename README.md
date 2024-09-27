# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Dependencies

* Rust ^1.81

### Endpoints

* station
    * metadata for all active stations
* station/stdmet
    * metadata for all active standard meteorological stations
* station/{id}
    * metadata for a specific station
* station/{id}/{year}
    * historic sensor data for the specified station
* station/{id}/realtime
    * realtime (last 45 days) sensor data for the specified station

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

