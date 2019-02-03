#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

(cd project-example && cargo spatial codegen)
cargo build --release
cargo build --examples --release

cargo test

exit 0
