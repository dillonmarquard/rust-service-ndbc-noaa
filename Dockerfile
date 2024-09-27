FROM rust:1.81 AS builder
WORKDIR /usr/src/rust-service-ndbc-noaa
COPY . .
RUN cargo install --path .

FROM debian:latest
RUN apt-get update && apt-get upgrade && apt-get install -y openssl && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-service-ndbc-noaa /usr/local/bin/rust-service-ndbc-noaa
EXPOSE 80
CMD ["rust-service-ndbc-noaa"]