# Build Stage
FROM ghcr.io/upryzing/base:latest AS builder

# Bundle Stage
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /home/rust/src/target/release/upryzing-pigeon ./
COPY --from=mwader/static-ffmpeg:7.0.2 /ffmpeg /usr/local/bin/
COPY --from=mwader/static-ffmpeg:7.0.2 /ffprobe /usr/local/bin/

EXPOSE 14704
USER nonroot
CMD ["./upryzing-pigeon"]
