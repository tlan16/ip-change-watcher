#!/usr/bin/env bash
set -uro pipefail
cd "$(dirname "$0")/.." || exit 1

cargo run --manifest-path migration/Cargo.toml -- up
