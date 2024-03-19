FROM rust:latest
LABEL authors="rami.bendhia"

WORKDIR /app
COPY . /app

# ENV SQLX_OFFLINE true
# RUN cargo build
# RUN cargo sqlx prepare > output.txt
# CMD ["./target/debug/service"]
CMD ["cargo", "run"]