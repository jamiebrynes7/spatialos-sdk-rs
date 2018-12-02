#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

cargo build --release
cargo build --examples --release

cargo test

exit 0