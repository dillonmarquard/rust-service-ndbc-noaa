FROM rust:1.81 as builder
WORKDIR /usr/src/rust-ndbc-wrapper
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-ndbc-wrapper /usr/local/bin/rust-ndbc-wrapper
EXPOSE 80
CMD ["rust-ndbc-wrapper"]