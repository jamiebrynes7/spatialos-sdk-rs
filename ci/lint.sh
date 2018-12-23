#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

cd "$(dirname $0)/../"

cargo fmt -- --check

pushd spatialos-sdk
    cargo clippy --all-targets --all-features -- -D warnings -A dead-code
popd

pushd spatialos-sdk-tools
    cargo clippy --all-targets --all-features -- -D warnings -A dead-code
popd

exit 0