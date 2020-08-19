FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release --no-default-features

FROM alpine

COPY --from=builder /usr/src/app/target/release/generate_204 /usr/local/bin/

ENV HOST 0.0.0.0
ENV PORT 80
EXPOSE 80

HEALTHCHECK CMD wget --quiet --tries=1 --spider http://${HOST}:${PORT}/generate_204 || exit 1

ENTRYPOINT generate_204
