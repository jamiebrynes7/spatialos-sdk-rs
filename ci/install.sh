#!/usr/bin/env bash

set -euo pipefail
if [[ -n "${DEBUG-}" ]]; then
  set -x
fi

ARCHIVER_RELEASE="3.1.0"

function isLinux() {
  [[ "$(uname -s)" == "Linux" ]];
}

function isMacOS() {
  [[ "$(uname -s)" == "Darwin" ]];
}

function isWindows() {
  ! ( isLinux || isMacOS );
}

rm -rf "./tmp"
mkdir -p "./tmp"

echo "Downloading archiver."
if isLinux; then
    ARCHIVER_PLATFORM="arc_linux_amd64"
elif isMacOS; then
    ARCHIVER_PLATFORM="arc_mac_amd64"
elif isWindows; then
    ARCHIVER_PLATFORM="arc_windows_amd64.exe"
else 
    echo "Unsupported platform"
    exit 1
fi

URL="https://github.com/mholt/archiver/releases/download/v${ARCHIVER_RELEASE}/${ARCHIVER_PLATFORM}"

curl -sSLf -o ./tmp/archiver "${URL}"
chmod +x ./tmp/archiver

echo "Unpacking SpatialOS dependencies"

curl -c ./tmp/cookie -s -L "https://drive.google.com/uc?export=download&id=${FILE_ID}" > /dev/null
curl -Lb ./tmp/cookie "https://drive.google.com/uc?export=download&confirm=`awk '/download/ {print $NF}' ./tmp/cookie`&id=${FILE_ID}" -o "./tmp/dependencies.tar"

DEPENDENCIES_TARGET_DIR="dependencies"
rm -rf "${DEPENDENCIES_TARGET_DIR}"
mkdir -p "${DEPENDENCIES_TARGET_DIR}"

./tmp/archiver unarchive "./tmp/dependencies.tar" "${DEPENDENCIES_TARGET_DIR}"

rm -rf "./tmp"

rustup component add rustfmt-preview
rustup component add clippy-preview
