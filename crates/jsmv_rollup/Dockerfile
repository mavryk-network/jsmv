ARG MAVKIT_TAG
FROM mavrykdynamics/mavryk-bare:${MAVKIT_TAG} AS mavkit

FROM rust:1.73.0-alpine AS builder
# Install build deps
RUN rustup target add wasm32-unknown-unknown
RUN apk --no-cache add musl-dev libcrypto3 openssl-dev clang
ENV OPENSSL_DIR=/usr
# Copy srcs
WORKDIR /
COPY . .
# Build jsmv-rollup
# RUSTFLAGS is needed to avoid linking to the static version of the C runtime (provided by musl)
# https://github.com/rust-lang/rust/pull/58575#issuecomment-496026747
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --package jsmv_rollup --release
# Build jsmv kernel
ARG JSTZ_ROLLUP_BRIDGE_ADDRESS
RUN CC=clang cargo build --package jsmv_kernel --release --target wasm32-unknown-unknown
RUN /target/release/jsmv-rollup make-installer \
    --kernel /target/wasm32-unknown-unknown/release/jsmv_kernel.wasm \
    --bridge "$JSTZ_ROLLUP_BRIDGE_ADDRESS" \
    --output /root/installer

FROM alpine AS node
# libcrypto3, openssl, and musl are needed for jsmv-rollup binary
# hidapi, gmp, and libev are needed for mavkit-client binary
# libffi, libgmpxx are needed for mavkit-smart-rollup-node binary
RUN apk --no-cache add binutils bash libcrypto3 openssl musl hidapi gmp libev libffi libgmpxx
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug
ARG MAVKIT_PROTO
COPY --from=mavkit /usr/local/bin/mavkit-smart-rollup-node-${MAVKIT_PROTO} /usr/bin/mavkit-smart-rollup-node
COPY --from=mavkit /usr/local/bin/mavkit-client /usr/bin/mavkit-client
COPY --from=builder /target/release/jsmv-rollup /usr/bin/jsmv-rollup
COPY --from=builder /root/installer /root/installer
COPY ./crates/jsmv_rollup/entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh
ENTRYPOINT [ "./entrypoint.sh" ]