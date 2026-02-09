#!/usr/bin/env bash

# fail asap
set -e

# Check if an argument was provided
if [ $# -eq 0 ]; then
    echo "No arguments provided"
    echo "Usage: scripts/publish-debug-image.sh 20230826-1 true"
    echo ""
    echo "Last argument specifies whether we should have a debug build as opposed to release build."
    exit 1
fi

DEBUG=$2
if [ "$DEBUG" = "true" ]; then
  echo "[profile.release]" >> Cargo.toml
  echo "debug = true" >> Cargo.toml
fi

TAG=$1-debug
echo "Building images, will tag for ghcr.io with $TAG!"
docker build -t ghcr.io/upryzing/base:latest -f Dockerfile.useCurrentArch .
docker build -t ghcr.io/upryzing/server:$TAG - < crates/delta/Dockerfile
docker build -t ghcr.io/upryzing/bonfire:$TAG - < crates/bonfire/Dockerfile
docker build -t ghcr.io/upryzing/pigeon:$TAG - < crates/services/pigeon/Dockerfile
docker build -t ghcr.io/upryzing/dove:$TAG - < crates/services/dove/Dockerfile
docker build -t ghcr.io/upryzing/gifbox:$TAG - < crates/services/gifbox/Dockerfile
docker build -t ghcr.io/upryzing/crond:$TAG - < crates/daemons/crond/Dockerfile
docker build -t ghcr.io/upryzing/pushd:$TAG - < crates/daemons/pushd/Dockerfile
docker build -t ghcr.io/upryzing/voice-ingress:$TAG - < crates/daemons/voice-ingress/Dockerfile

if [ "$DEBUG" = "true" ]; then
  git restore Cargo.toml
fi

docker push ghcr.io/upryzing/server:$TAG
docker push ghcr.io/upryzing/bonfire:$TAG
docker push ghcr.io/upryzing/pigeon:$TAG
docker push ghcr.io/upryzing/dove:$TAG
docker push ghcr.io/upryzing/gifbox:$TAG
docker push ghcr.io/upryzing/crond:$TAG
docker push ghcr.io/upryzing/pushd:$TAG
docker push ghcr.io/upryzing/voice-ingress:$TAG
