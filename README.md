# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Useful Resources
Any questions regarding the meaning of an attribute or measurement can be found on the NDBC website.  
* https://www.ndbc.noaa.gov/faq/
* Useful Buoy https://www.ndbc.noaa.gov/station_history.php?station=46014
### Dependencies
* Rust ^1.81

### NDBC NOAA Buoy Data
## Payload Formats
* https://www.ndbc.noaa.gov/faq/rsa.shtml
* https://www.ndbc.noaa.gov/data/historical

### Endpoints
* /station
   * metadata for all active stations
* /station/{id}
    * metadata for a specific station
    * historic stdmet data available for download
* /station/{id}/stdmet/{year}
    * historic Standard Meteorological sensor data for the specified station and year
* /station/{id}/stdmet/realtime
    * realtime (last 45 days) stdmet sensor data for the specified station
* /station/{id}/stdmetdrift/realtime
    * realtime (last 45 days) stdmet drift sensor data for the specified station
    * drifting buoys do not provide stdmet data in the same format
    * other realtime formats will be supported in the future eg. ocean
* /station/{id}/cwind/{year}
* /station/{id}/cwind/realtime

### Future Considerations
* future data will be considered in order: ocean current, spectral wave, oceanographic,  
* filters (future consideration)
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
