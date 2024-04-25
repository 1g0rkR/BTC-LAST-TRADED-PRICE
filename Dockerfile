# Build btcapp
FROM rust:1.77-bookworm as builder

WORKDIR /root

COPY ./ ./

RUN cargo build --release --bin btc_app

# Running environment
FROM debian:bookworm

WORKDIR /usr/local/bin

COPY --from=builder /root/target/release/btc_app .

CMD ["./btc_app"]