# Contributing

## Running the test-suite

There are some integration tests that live in the `test-suite` crate. These utilize and test generated code. To run these tests: 

```
$ cd test-suite && cargo spatial codegen && cargo test
```

## Testing the code generator

To regenerate the schema bundle, run the following:

```
$ ./dependencies/schema-compiler/schema_compiler --schema_path=project-example/schema --schema_path=dependencies/std-lib project-example/schema/example.schema --bundle_json_out=spatialos-sdk-code-generator/data/test.sb.json
```

To run the code generator tests, run the following:

```
$ cargo test -p spatialos-sdk-code-generator
```

To display Rusts auto-generated debug representation of the schema bundle, run the following:

```
$ cargo test -p spatialos-sdk-code-generator -- --nocapture
```

## Updating Rust bindings

To update the Rust bindings found in `spatialos-sdk-sys` run the following command from the root of the repository:

```bash
$ cargo run --bin generate_bindings -- -i ./dependencies/headers/include/improbable/ -o ./spatialos-sdk-sys/src/
```

> **Note:** this depends on `bindgen` which has `clang` as a dependency. See [bindgen's documentation](https://rust-lang.github.io/rust-bindgen/requirements.html) for more info.