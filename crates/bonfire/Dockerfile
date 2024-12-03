# Build Stage
FROM ghcr.io/upryzing/base:latest AS builder

# Bundle Stage
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /home/rust/src/target/release/upryzing-bonfire ./

EXPOSE 14703
USER nonroot
CMD ["./upryzing-bonfire"]
