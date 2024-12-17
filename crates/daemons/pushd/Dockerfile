# Build Stage
FROM ghcr.io/upryzing/base:latest AS builder
FROM debian:12 AS debian

# Bundle Stage
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /home/rust/src/target/release/upryzing-pushd ./
COPY --from=debian /usr/bin/uname /usr/bin/uname

USER nonroot
CMD ["./upryzing-pushd"]