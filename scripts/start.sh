#!/usr/bin/env bash
set -e
cargo build \
    --bin upryzing-delta \
    --bin upryzing-bonfire \
    --bin upryzing-pigeon \
    --bin upryzing-dove

trap 'pkill -f upryzing-' SIGINT
cargo run --bin upryzing-delta &
cargo run --bin upryzing-bonfire &
cargo run --bin upryzing-pigeon &
cargo run --bin upryzing-dove
