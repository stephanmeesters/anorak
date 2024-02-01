FROM rust:1.75-buster 
# FROM rust:1.75-buster as builder

WORKDIR /usr/src/

# RUN apt-get update && apt-get install -y musl-dev musl-tools libssl-dev pkg-config

# RUN export OPENSSL_DIR=/usr/local/opt/openssl/
 
# Create blank project
RUN USER=root cargo new anorak 

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/anorak/

# Set the working directory
WORKDIR /usr/src/anorak

## Install target platform (Cross-Compilation) --> Needed for Alpine
# RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --release
# RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/anorak/src/
COPY assets /usr/src/anorak/assets/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/anorak/src/main.rs

# This is the actual application build.
RUN cargo build --release

# RUN echo $(ls -1 /usr/src/anorak/target/release)

### deploy

# FROM debian:bookworm-slim

# RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

RUN cp /usr/src/anorak/target/release/anorak /usr/local/bin/anorak
# COPY --from=builder /usr/src/anorak/target/release/anorak /usr/local/bin/anorak
# COPY --from=builder /usr/src/anorak/target/x86_64-unknown-linux-musl/release/anorak /usr/local/bin/anorak
COPY assets /usr/local/bin/assets/

EXPOSE 9341 

ENV RUST_LOG=info

CMD ["anorak"]
