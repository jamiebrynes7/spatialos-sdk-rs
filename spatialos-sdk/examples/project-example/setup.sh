#!/usr/bin/env bash

set -e -x

cd "$(dirname "$0")"

DEPENDENCIES="../../../dependencies"
SCHEMA_COMPILER="${DEPENDENCIES}/schema-compiler/schema_compiler"
PROTOC="${DEPENDENCIES}/schema-compiler/protoc"

mkdir -p "spatial-os/schema/bin"
mkdir -p "tmp"

cp -r ${DEPENDENCIES}/schema-compiler/proto/* "./tmp"
"${SCHEMA_COMPILER}" --schema_path="${DEPENDENCIES}/std-lib" --proto_out="./tmp" --load_all_schema_on_schema_path ${DEPENDENCIES}/std-lib/improbable/*.schema
"${PROTOC}" --proto_path="./tmp" --descriptor_set_out="./spatial-os/schema/bin/schema.descriptor" --include_imports  tmp/**/*.proto

rm -rf "tmp"