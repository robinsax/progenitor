# TODO release mode

# Version lock
FROM rust:1.63-slim-buster AS rust_version

RUN rustup target add x86_64-unknown-linux-musl

# Deps install and build
FROM rust_version AS build_deps

WORKDIR /api-build

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target=x86_64-unknown-linux-musl && \
    rm -r src

# Build
FROM build_deps AS build

COPY . .

# touch to reset mtime, since the dummy from previous stage is newer
RUN touch src/main.rs && \
    cargo build --target=x86_64-unknown-linux-musl

RUN mkdir /api-release && \
    cp ./target/x86_64-unknown-linux-musl/debug/api /api-release

# Runtime
FROM alpine:latest AS runtime

WORKDIR /api-runtime

COPY --from=build /api-release/api /api-runtime/api

ENV ROCKET_ADDRESS=0.0.0.0

ENTRYPOINT ["./api"]
