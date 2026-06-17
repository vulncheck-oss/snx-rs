FROM rust:1.96-alpine as builder

RUN apk add --no-cache openssl-dev musl-dev git openssl-libs-static sqlite-dev sqlite-static

WORKDIR /app
RUN git clone https://github.com/vulncheck-oss/snx-rs
 
WORKDIR /app/snx-rs
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl -p snx-rs
RUN cp /app/snx-rs/target /output/

CMD ["echo", "Build finished"]
