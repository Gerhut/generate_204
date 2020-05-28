FROM rustlang/rust:nightly-alpine as builder

RUN apk add --no-cache musl-dev

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release --features nodotenv

FROM alpine

COPY --from=builder /usr/src/app/target/release/generate_204 /usr/local/bin/

ENV HOST 0.0.0.0
ENV PORT 80
EXPOSE 80

ENTRYPOINT generate_204
