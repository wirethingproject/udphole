FROM rust:alpine as builder

WORKDIR /usr/src/udphole
COPY . .

RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/udphole /usr/local/bin/udphole

ENTRYPOINT ["udphole"]
