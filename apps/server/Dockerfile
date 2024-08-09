# Build stage
FROM rust:1.79 as builder

WORKDIR /app

COPY . . 

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust_crud .

EXPOSE 8080

CMD ["./rust_crud"]
