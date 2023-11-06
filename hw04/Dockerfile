FROM rust:1.73 as builder
RUN apt update && apt install -y musl-dev sqlite3 libssl-dev

WORKDIR /usr/src/brutus
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
COPY migrations migrations
RUN cargo fetch
RUN cargo install --path .


# Must match rust deebian package for glib compatibility
FROM debian:12-slim
RUN apt update && apt install -y sqlite3
COPY --from=builder /usr/local/cargo/bin/brutus /usr/local/bin/brutus
COPY templates templates
COPY static static
EXPOSE 8000
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -fs "http://localhost:8000/" || exit 42
CMD ["brutus"]
