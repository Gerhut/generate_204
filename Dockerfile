FROM rust:alpine as builder

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release --features nodotenv

FROM alpine

RUN mkdir -p /usr/local/bin
WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/releast/generate_204 .
ENTRYPOINT generate_204
