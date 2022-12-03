FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /half-rs
WORKDIR /half-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /half-rs/fuzz/target/x86_64-unknown-linux-gnu/release/half-fuzz /