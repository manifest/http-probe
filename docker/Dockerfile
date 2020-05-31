## -----------------------------------------------------------------------------
## Build
## -----------------------------------------------------------------------------
FROM rust:slim-buster as build-stage

WORKDIR "/build"
COPY . .
RUN cargo build --release

## -----------------------------------------------------------------------------
## Package
## -----------------------------------------------------------------------------
FROM debian:buster

COPY --from=build-stage "/build/target/x86_64-unknown-linux-musl/release/http_101" "/app/http_101"

WORKDIR "/app"
ENTRYPOINT ["/app/http_101"]
