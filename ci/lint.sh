#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

cd "$(dirname $0)/../"

cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings -A dead-code

exit 0