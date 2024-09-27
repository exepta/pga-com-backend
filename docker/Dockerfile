FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY ../Cargo.toml ../Cargo.lock ./
RUN cargo fetch
COPY ../ ./

RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/pga-com-backend .
RUN chmod +x /usr/src/app/pga-com-backend

CMD ["./pga-com-backend"]