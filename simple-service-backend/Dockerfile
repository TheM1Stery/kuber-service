FROM lukemathwalker/cargo-chef:latest-rust-1.90.0-slim-trixie AS toolchain
WORKDIR /app

FROM toolchain AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM toolchain AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV APP__SERVER__PORT=8081
ENV APP__SERVER__ADDRESS=0.0.0.0
COPY --from=builder /app/target/release/simple-service /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/simple-service" ]

