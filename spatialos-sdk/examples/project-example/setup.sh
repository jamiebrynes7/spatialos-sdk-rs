#!/usr/bin/env bash

set -e -x

cd "$(dirname "$0")"

DEPENDENCIES="../../../dependencies"
SCHEMA_COMPILER="${DEPENDENCIES}/schema-compiler/schema_compiler"

mkdir -p "spatial-os/schema/bin"

"${SCHEMA_COMPILER}" --schema_path="${DEPENDENCIES}/std-lib" --schema_path="spatial-os/schema" --descriptor_set_out="spatial-os/schema/bin/schema.descriptor" --bundle_json_out="spatial-os/schema/bin/bundle.json" --load_all_schema_on_schema_path ../../dependencies/std-lib/improbable/*.schema spatial-os/schema/*.schema
