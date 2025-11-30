FROM rust:1.91.1-trixie as builder

WORKDIR /usr/src/

RUN USER=root cargo new anorak 

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/anorak/

WORKDIR /usr/src/anorak

RUN cargo build --release

COPY src /usr/src/anorak/src/
COPY assets /usr/src/anorak/assets/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/anorak/src/main.rs

RUN cargo build --release

### deploy

# bullseye is required for libssl 1.1.1
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y openssl

COPY --from=builder /usr/src/anorak/target/release/anorak /usr/local/bin/anorak
COPY assets /root/assets/

WORKDIR /root

EXPOSE 9341 

ENV RUST_LOG=info

CMD ["anorak"]
