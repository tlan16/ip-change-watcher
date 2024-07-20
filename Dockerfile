FROM rust:alpine3.20 as builder

# OS dependencies
RUN apk add --no-cache musl-dev

# App dependencies
WORKDIR /app
ADD Cargo.toml .
ADD Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build
RUN rm -rf src

# build app
ADD . .
RUN cargo build --release
RUN ls -alh target/release/ip-change-notifier


FROM alpine:3.20.1 as packer
# OS dependencies
RUN apk add --no-cache upx

# pack app
WORKDIR /app
COPY --from=builder /app/target/release/ip-change-notifier ip-change-notifier
RUN upx -9 --ultra-brute ip-change-notifier

FROM scratch
COPY --from=packer /app/ip-change-notifier /ip-change-notifier
CMD ["/ip-change-notifier"]
