FROM rust:1.85 AS builder

WORKDIR /app

COPY src ./src
COPY Cargo.toml ./

RUN cd src && \
        cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/event-tracker .

CMD ["./event-tracker"]
