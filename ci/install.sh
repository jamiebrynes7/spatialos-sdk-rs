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

echo "Downloading Spatial CLI"

if isWindows; then
    DOWNLOAD_URL="https://console.improbable.io/toolbelt/download/latest/win"
elif isMacOS; then
    DOWNLOAD_URL="https://console.improbable.io/toolbelt/download/latest/mac"
elif isLinux; then
    DOWNLOAD_URL="https://console.improbable.io/toolbelt/download/latest/linux"
else
    echo "Unknown CI platform."
    exit 1
fi

mkdir -p ~/.spatial
curl -sSLf -o ~/.spatial/spatial "${DOWNLOAD_URL}"
chmod +x ~/.spatial/spatial

export PATH=${PATH}:~/.spatial

echo "Install Spatial OAuth"

if isWindows; then
    OAUTH_LOCATION="${APPDATA}/Local/.improbable/oauth2"
else
    OAUTH_LOCATION="~/.improbable/oauth2"
fi

mkdir -p "${OAUTH_LOCATION}"
echo "$SPATIAL_OAUTH" > "${OAUTH_LOCATION}/oauth2_refresh_token"

spatial auth login

spatial version

echo "Installing cargo fmt"

rustup component add rustfmt-preview
