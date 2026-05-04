FROM rust:1.92-slim AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends gcc libc6-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY docker-compose.yml ./
COPY src ./src
COPY tests ./tests
RUN cargo test --locked || cargo test
RUN cargo build --release

FROM debian:trixie-slim AS runtime
RUN useradd --create-home --shell /usr/sbin/nologin cust
WORKDIR /workspace
COPY --from=build /app/target/release/cust /usr/local/bin/cust
USER cust
ENTRYPOINT ["cust"]
