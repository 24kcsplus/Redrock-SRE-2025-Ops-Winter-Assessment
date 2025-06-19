FROM rust:bookworm as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src target

COPY src ./src

RUN cargo build --release

FROM debian:bookworm

COPY --from=builder /usr/src/app/target/release/level1 /usr/local/bin/24k-rust-shell

CMD ["24k-rust-shell"]