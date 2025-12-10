# rust-service-ndbc-noaa
A Rust Service for the National Data Bouy Center's API and Web-interface.

### Useful Resources
Any questions regarding the meaning of an attribute or measurement can be found on the NDBC website.  
* https://www.ndbc.noaa.gov/faq/
* https://www.ndbc.noaa.gov/docs/ndbc_web_data_guide.pdf
* Useful Buoy https://www.ndbc.noaa.gov/station_history.php?station=46014
### Dependencies
* Rust ^1.91.1

### NDBC NOAA Buoy Data
## Payload Formats
* https://www.ndbc.noaa.gov/faq/rsa.shtml
* https://www.ndbc.noaa.gov/data/historical

### Endpoints
* /station
   * metadata for all active stations
   * includes availability of historic and realtime data
* /station/{id}
    * metadata for a specific station
    * historic stdmet data available for download
* /station/{id}/stdmet/{year}
    * historic Standard Meteorological sensor data for the specified station and year
* /station/{id}/cwind/{year}
    * historic Continuous Winds sensor data for the specified station and year
* /station/{id}/stdmet/realtime
    * realtime (last 45 days) stdmet sensor data for the specified station
* /station/{id}/stdmetdrift/realtime
    * realtime (last 45 days) stdmetdrift sensor data for the specified station
    * drifting buoys do not provide stdmet data in the same format
* /station/{id}/cwind/realtime
    * realtime (last 45 days) cwind sensor data for the specified station
* /station/{id}/spec/realtime
    * realtime (last 45 days) spec sensor data for the specified station

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
