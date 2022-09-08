FROM rust:latest as builder

RUN cargo new --bin learnactix
WORKDIR ./learnactix
COPY . .
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /learnactix/target/release/learnactix ./learnactix
RUN apt update && apt install libpq-dev -y
CMD ["./learnactix"]
