# Build Stage
FROM ghcr.io/upryzing/base:latest AS builder

# Bundle Stage
FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean
COPY --from=builder /home/rust/src/target/release/upryzing-bonfire ./
EXPOSE 9000
CMD ["./upryzing-bonfire"]
