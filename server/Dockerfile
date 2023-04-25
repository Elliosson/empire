
FROM rust:1.66.0

COPY . .
WORKDIR /server
RUN cargo build --release

CMD ["./target/release/server"]