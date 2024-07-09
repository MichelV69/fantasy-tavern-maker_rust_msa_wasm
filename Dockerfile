FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin tavern

# We do not need the Rust toolchain to run the binary!
# FROM debian:bookworm-slim AS runtime
FROM debian:bookworm AS runtime
WORKDIR /app
COPY content ./content
COPY --from=builder /app/target/release/tavern .
#CMD [ "bash" ]
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
ENTRYPOINT ["./tavern"]
