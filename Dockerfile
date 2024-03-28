# Build 
FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/Jenkins .

CMD [ "./Jenkins" ]