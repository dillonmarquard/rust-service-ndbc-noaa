#!/bin/sh
k3d cluster delete demo-cluster
docker build -t rust-service-ndbc-noaa .
k3d cluster create --api-port 6550 -p "8081:80@loadbalancer" --agents 2 --wait demo-cluster
k3d image import rust-service-ndbc-noaa -c demo-cluster
kubectl apply -f deployment.yaml