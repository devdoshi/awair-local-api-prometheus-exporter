# @REF: https://github.com/LukeMathWalker/cargo-chef#running-the-binary-in-alpine
# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM ekidd/rust-musl-builder:1.51.0 AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin app

FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/
USER myuser
ENV AWAIR_LOCAL_URL ""
EXPOSE 9185
CMD ["/usr/local/bin/app"]

