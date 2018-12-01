#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

export SPATIAL_LIB_DIR="$(pwd)/dependencies"

cargo build --release
cargo build --examples --release

cargo test

exit 0