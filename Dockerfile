FROM rust:1.77.2 as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y musl-tools musl-dev

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --target x86_64-unknown-linux-musl --release
FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/crypto-ltp-example /usr/local/bin/crypto-ltp-example

# expose port
EXPOSE 8080

CMD ["crypto-ltp-example"]