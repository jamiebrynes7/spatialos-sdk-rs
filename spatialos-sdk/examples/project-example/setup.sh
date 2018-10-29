#!/usr/bin/env bash

set -e -x

cd "$(dirname "$0")"

SCHEMA_COMPILER="../../dependencies/schema-compiler/schema_compiler"
PROTOC="../../dependencies/schema-compiler/protoc"

mkdir -p "spatial-os/schema/bin"
mkdir -p "tmp"

cp -r ../../dependencies/schema-compiler/proto/* "./tmp"
"${SCHEMA_COMPILER}" --schema_path="../../dependencies/std-lib" --proto_out="./tmp" --load_all_schema_on_schema_path ../../dependencies/std-lib/improbable/*.schema
"${PROTOC}" --proto_path="./tmp" --descriptor_set_out="./spatial-os/schema/bin/schema.descriptor" --include_imports  tmp/**/*.proto

rm -rf "tmp"