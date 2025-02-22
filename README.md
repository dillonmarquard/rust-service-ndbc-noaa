# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Useful Resources
Any questions regarding the meaning of an attribute or measurement can be found in the ndbc website.  
* https://www.ndbc.noaa.gov/faq/

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

### Future Considerations
I hope to continue development to make more than just stdmet and stdmet drift data available.
* ocean, cwinds, etc.
* filters (future consideration)
    * available data eg. `/station?available=stdmet` or `/station?available=currents`
    * distance from point (lat + lon) eg. `/station?lat=10&lon=4&dist=1`
 
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

