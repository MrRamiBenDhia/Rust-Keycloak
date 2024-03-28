FROM rust:alpine as builder

WORKDIR /app/src
RUN USER=root

RUN apk add pkgconfig openssl-dev libc-dev
COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN apk update \
    && apk add openssl ca-certificates

EXPOSE 8000

COPY --from=builder /app/src/target/release/my-rust-app /app/my-rust-app 

CMD ["/app/my-rust-app "]