#!/usr/bin/env bash

set -x

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

export PATH=${PATH}:~/.spatial/spatial

spatial version