FROM rust:1.73 as builder
WORKDIR /usr/src/automeme

COPY . .
RUN cargo test
RUN cargo build -r

FROM debian:bookworm-slim
WORKDIR /usr/src/automeme

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/automeme/target/release/automeme-web .
COPY templates templates

CMD ["./automeme-web"]