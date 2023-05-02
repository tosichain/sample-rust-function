FROM rust:alpine3.17 AS build

WORKDIR /ipfs-example

COPY ./ ./

RUN apk add --no-cache musl-dev

RUN cargo build --release

FROM alpine:3.17

COPY --from=build /ipfs-example/target/release/ipfs-example /init