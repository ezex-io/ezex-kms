FROM rust:1.86-slim AS builder

WORKDIR /app
COPY . .

RUN apt update
RUN apt install -y protobuf-compiler libssl-dev pkg-config

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update
RUN apt install -y libssl-dev

COPY --from=builder /app/target/release/ezex-kms /usr/bin/ezex-kms

EXPOSE 39577

ENTRYPOINT ["/usr/bin/ezex-kms", "start"]
