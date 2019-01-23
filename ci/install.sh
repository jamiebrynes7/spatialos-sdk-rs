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

function waitOnExit() {
  sleep 5
}

# Force full output on Travis instead of truncating output.
trap waitOnExit EXIT


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

rm -rf "./temp"
mkdir -p "./temp"

curl -sSL $SPATIAL_URL --output ./temp/spatial
chmod +x ./temp/spatial
PATH=$PATH:$(pwd)/temp/

export RUST_BACKTRACE=1

# TODO: Windows support

mkdir -p ~/.improbable/oauth2
echo $SPATIAL_OAUTH > ~/.improbable/oauth2/oauth2_refresh_token

cargo run --bin download_sdk -- -d dependencies -s 13.5.1

rm -rf "./temp"

rustup component add rustfmt-preview
rustup component add clippy-preview
