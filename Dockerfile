FROM rust:1.82 AS build-env
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY Cargo.toml /app
COPY Cargo.lock /app
RUN mkdir -p /app/crates/claptrap/src
RUN mkdir -p /app/crates/claptrap-core/src
RUN mkdir -p /app/crates/gen-api-doc/src
COPY crates/claptrap/Cargo.toml /app/crates/claptrap/Cargo.toml
COPY crates/claptrap-core/Cargo.toml /app/crates/claptrap-core/Cargo.toml
COPY crates/gen-api-doc/Cargo.toml /app/crates/gen-api-doc/Cargo.toml

COPY crates/claptrap/templates /app/crates/claptrap/templates

# dummy build to cache dependencies
RUN echo "fn main() {}" > /app/crates/claptrap/src/main.rs
RUN touch /app/crates/claptrap-core/src/lib.rs
RUN touch /app/crates/gen-api-doc/src/lib.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl --package claptrap

# copy the actual application code and build
COPY crates/claptrap/src /app/crates/claptrap/src
COPY crates/claptrap-core/src /app/crates/claptrap-core/src
COPY README.md /app
COPY README.md /app/crates/claptrap
RUN cargo clean --release --target=x86_64-unknown-linux-musl -p claptrap-core
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine
RUN apk update && apk add ncurses
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/claptrap /
ENTRYPOINT ["./claptrap"]
