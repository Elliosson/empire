
FROM rust:1.66.0

COPY . .
WORKDIR /server
RUN cargo build --release --features fly

CMD ["./target/release/server"]