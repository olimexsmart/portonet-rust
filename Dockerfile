# syntax=docker/dockerfile:1

FROM rust:alpine AS builder

# Build inside a stable path so the final COPY can target one known location.
WORKDIR /build/portonet

RUN apk add --no-cache \
  musl-dev \
  pkgconf \
  sqlite-dev \
  openssl-dev \
  openssl-libs-static

# Copy the full crate so Cargo has the source, lockfile, and frontend assets.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY frontend ./frontend

RUN cargo build --release --locked

FROM alpine:latest

RUN apk add --no-cache \
  ca-certificates \
  libgcc \
  sqlite-libs \
  # Run the app as a non-root user.
  && addgroup -S portonet \
  && adduser -S -G portonet portonet \
  && mkdir -p /app/frontend /data \
  && chown -R portonet:portonet /app /data

WORKDIR /app

# Copy the compiled binary from the builder stage.
COPY --from=builder /build/portonet/target/release/portonet /app/portonet
# Copy the browser assets and favicon the app serves at runtime.
COPY --from=builder /build/portonet/frontend/ /app/frontend/
COPY --from=builder /build/portonet/favicon.ico /app/favicon.ico

# Default runtime config can be overridden with docker run -e ...
ENV DATABASE_URL=sqlite:/data/portonet.sqlite
ENV PORT=3000
ENV LICENSE_REFRESH_DAYS=30
ENV MAX_FAILED_ATTEMPTS=5

VOLUME ["/data"]
EXPOSE 3000

USER portonet
CMD ["/app/portonet"]
