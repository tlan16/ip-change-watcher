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

# App build
ADD . .
RUN cargo build --release
RUN ls -alh target/release/ip-change-notifier

FROM scratch
COPY --from=builder /app/target/release/ip-change-notifier /ip-change-notifier
CMD ["/ip-change-notifier"]
