# syntax=docker/dockerfile:1

FROM rust:latest AS build

WORKDIR /app

RUN \
  DEBIAN_FRONTEND=noninteractive \
  apt-get update &&\
  apt-get -y install ca-certificates tzdata

ARG TARGETARCH
ARG TARGETVARIANT

COPY . .

RUN \
  --mount=type=cache,id=microbin-cargo,target=/var/cache/cargo \
  --mount=type=cache,id=microbin-target-${TARGETARCH}-${TARGETVARIANT},target=/app/target \
  CARGO_HOME=/var/cache/cargo \
  CARGO_NET_GIT_FETCH_WITH_CLI=true \
  cargo build --release &&\
  cp /app/target/release/microbin /tmp/microbin

# https://hub.docker.com/r/bitnami/minideb
FROM debian:bookworm-slim

# microbin will be in /app
WORKDIR /app

# Keep runtime certificates and timezone data available on every target
RUN apt-get update \
  && apt-get install --no-install-recommends -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

# copy built executable
COPY --from=build \
  /tmp/microbin \
  /usr/bin/microbin

# Expose webport used for the webserver to the docker runtime
EXPOSE 8080

ENTRYPOINT ["microbin"]
