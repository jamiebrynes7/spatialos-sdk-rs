#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

function isLinux() {
  [[ "$(uname -s)" == "Linux" ]];
}

function isMacOS() {
  [[ "$(uname -s)" == "Darwin" ]];
}

function isWindows() {
  ! ( isLinux || isMacOS );
}


if isWindows; then
  SPATIAL_URL="https://console.improbable.io/toolbelt/download/latest/win"
elif isMacOS; then
  SPATIAL_URL="https://console.improbable.io/toolbelt/download/latest/mac"
elif isLinux; then
  SPATIAL_URL="https://console.improbable.io/toolbelt/download/latest/linux"
else 
  echo "Unsupported platform"
  exit 1
fi

rm -rf "./tmp"
mkdir -p "./tmp"

curl -sSL $SPATIAL_URL --output ./tmp/spatial
chmod +x ./tmp/spatial
PATH=$PATH:$(pwd)/tmp/

mkdir -p ~/.improbable/oauth2
echo $SPATIAL_OAUTH > ~/.improbable/oauth2/oauth2_refresh_token

cargo run --bin download_sdk -- -d dependencies -s 13.5.1

rm -rf "./tmp"

rustup component add rustfmt-preview
rustup component add clippy-preview
