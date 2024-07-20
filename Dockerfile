FROM rust:alpine3.20 as builder

# OS dependencies
RUN apk add --no-cache musl-dev

# App dependencies
WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build
RUN rm -rf src

# build app
ADD . .
RUN cargo build --release --bin ip-change-notifier
RUN ls -alh target/release/ip-change-notifier
RUN cp /app/target/release/ip-change-notifier /app/ip-change-notifier

FROM rust:alpine3.20 as packer-rust-obfuscator
# OS dependencies
RUN apk add --no-cache uuidgen git musl-dev

# App dependencies
RUN git clone --depth 1 --filter=blob:none https://github.com/dronavallipranav/rust-obfuscator.git
WORKDIR /rust-obfuscator
RUN cargo build --release --bin rust-obfuscator

# obfuscate app
WORKDIR /app
COPY --from=builder /app/ip-change-notifier ip-change-notifier
RUN CRYPTIFY_KEY="$(uuidgen)" /rust-obfuscator/target/release/rust-obfuscator ip-change-notifier

FROM alpine:3.20.1 as packer-upx
# OS dependencies
RUN apk add --no-cache upx

# pack app
WORKDIR /app
COPY --from=packer-rust-obfuscator /app/ip-change-notifier ip-change-notifier
RUN upx -9 --lzma ip-change-notifier

FROM scratch
WORKDIR /app
COPY --from=packer-upx /app/ip-change-notifier /app/ip-change-notifier
ENTRYPOINT ["/app/ip-change-notifier"]
