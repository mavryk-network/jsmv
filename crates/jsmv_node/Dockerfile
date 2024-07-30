FROM rust:1.73.0 AS builder
WORKDIR /
COPY . .
RUN cargo build --package jsmv_node --release

FROM debian:bookworm-slim AS node
RUN apt-get update && apt-get install -y openssl
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug
COPY --from=builder /target/release/jsmv-node /usr/bin/jsmv-node
ENTRYPOINT [ "/usr/bin/jsmv-node" ]