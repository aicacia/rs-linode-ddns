FROM rust:1.88-bookworm AS builder

RUN apt update && apt -yq upgrade
RUN apt -yq install musl-tools libpq-dev

RUN rustup default stable

ARG TARGET=x86_64-unknown-linux-musl
RUN rustup target add ${TARGET}

WORKDIR /
RUN cargo new app && touch /app/src/lib.rs
WORKDIR /app
RUN cargo new linode-api-client && touch /app/linode-api-client/src/lib.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --target ${TARGET} --release

COPY . .
RUN touch /app/src/lib.rs && touch /app/src/main.rs
RUN cargo build --target ${TARGET} --release

FROM scratch
LABEL org.opencontainers.image.source=https://github.com/aicacia/rs-linode-ddns

WORKDIR /app

ARG TARGET=x86_64-unknown-linux-musl
COPY --from=builder /app/target/${TARGET}/release/linode-ddns /app/linode-ddns

ENV RUN_MODE=production

CMD ["/app/linode-ddns", "-c", "/app/config.json", "watch"]
