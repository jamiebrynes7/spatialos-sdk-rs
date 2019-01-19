#!/usr/bin/env bash

set -e -x

cd "$(dirname "$0")"

SCHEMA_COMPILER="${SPATIAL_LIB_DIR}/schema-compiler/schema_compiler"
PROTOC="${SPATIAL_LIB_DIR}/schema-compiler/protoc"

mkdir -p "spatialos/schema/bin"
mkdir -p "tmp"

cp -r ${SPATIAL_LIB_DIR}/schema-compiler/proto/* "./tmp"
"${SCHEMA_COMPILER}" --schema_path="${SPATIAL_LIB_DIR}/std-lib" --proto_out="./tmp" --load_all_schema_on_schema_path ${SPATIAL_LIB_DIR}/std-lib/improbable/*.schema
"${PROTOC}" --proto_path="./tmp" --descriptor_set_out="./spatialos/schema/bin/schema.descriptor" --include_imports  tmp/**/*.proto

rm -rf "tmp"
