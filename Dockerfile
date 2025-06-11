FROM rust:1.85 AS build-env
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY Cargo.toml /app
COPY Cargo.lock /app
RUN mkdir -p src
COPY Cargo.toml /app/Cargo.toml

# dummy build to cache dependencies
RUN echo "fn main() {}" > /app/src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl --package claptrap

# copy the actual application code and build
COPY src /app/src
COPY README.md /app
RUN cargo clean --release --target=x86_64-unknown-linux-musl -p claptrap
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine
RUN apk update && apk add ncurses
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/claptrap /
ENTRYPOINT ["./claptrap"]
