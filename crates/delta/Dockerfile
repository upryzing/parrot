# Build Stage
FROM ghcr.io/upryzing/base:latest AS builder

# Bundle Stage
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /home/rust/src/target/release/upryzing-delta ./

EXPOSE 14702
ENV ROCKET_ADDRESS 0.0.0.0
USER nonroot
CMD ["./upryzing-delta"]
