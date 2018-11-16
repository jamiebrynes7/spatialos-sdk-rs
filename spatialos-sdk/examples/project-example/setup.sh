#!/usr/bin/env bash

set -e -x

cd "$(dirname "$0")"

DEPENDENCIES="../../../dependencies"
SCHEMA_COMPILER="${DEPENDENCIES}/schema-compiler/schema_compiler"

mkdir -p "spatialos/schema/bin"

"${SCHEMA_COMPILER}" \
    --schema_path="${DEPENDENCIES}/std-lib" \
    --schema_path="spatialos/schema" \
    --descriptor_set_out="spatialos/schema/bin/schema.descriptor" \
    --bundle_json_out="spatialos/schema/bin/bundle.json" \
    --load_all_schema_on_schema_path \
    ${DEPENDENCIES}/std-lib/improbable/*.schema \
    spatialos/schema/*.schema
