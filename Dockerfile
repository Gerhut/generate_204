FROM rust as builder

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release --features nodotenv

FROM debian

COPY --from=builder /usr/src/app/target/release/generate_204 /usr/local/bin/
ENTRYPOINT generate_204
