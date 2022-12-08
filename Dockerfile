# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:1.64.0 AS build

WORKDIR /usr/src

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new store-api-rs
WORKDIR /usr/src/store-api-rs
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY ./src ./src
RUN cargo clean
RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM debian:11.5-slim
RUN apt-get update && apt-get install -y build-essential
WORKDIR /data
COPY ./resources ./resources
COPY --from=build /usr/src/store-api-rs/target/release/store-api-rs ./store-api-rs
USER root
CMD /data/store-api-rs