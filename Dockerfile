FROM rust:1.92-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
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
