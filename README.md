<div align="center">
    <h1>SpatialOS SDK for Rust</h1>
    <p>
        <strong>An integration of the SpatialOS C API and the Rust programming language.</strong>
    </p>
    <p>
        <a href="https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs"><img src="https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs/status.svg"/></a>
        <img src="https://img.shields.io/badge/rustc-1.39-blue.svg"/>
    </p>
</div>


> **Note**: This is an **unofficial** integration of the [SpatialOS C API ](https://docs.improbable.io/reference/latest/capi/introduction) with Rust. Improbable does not officially support Rust as a worker language.

# Quick start

## Requirements

To develop in this repository you'll need:

1. Rust v1.39
2. A [SpatialOS account](https://www.improbable.io/get-spatialos) 

## Setup

1. Clone this repository.
2. Install `cargo-spatial`.
   
    ```
    $ cargo install --path ./cargo-spatial --force
    ```
3. Set the `SPATIAL_LIB_DIR` environment variable to the location of the SpatialOS dependencies.
   
   ```
   $ export SPATIAL_LIB_DIR=$(pwd)/dependencies
   ```
4. Download the C API dependencies.
   
   ```
   $ cargo spatial download sdk --sdk-version 14.0.0
   ```
5. Build the `spatialos-sdk` crate.
   
   ```
   $ cd spatialos-sdk && cargo build
   ```

At this point, the `spatialos-sdk` crate has been built and linked successfully and can be used in user code.

## Running the Example Project

To run the example project, you will need to:

1. Navigate to the example project:

  ```
  $ cd project-example
  ```
2. Generate Rust code from the project's [SpatialOS schema](https://docs.improbable.io/reference/14.2/shared/schema/introduction).
  ```
  $ cargo spatial codegen
  ```
3. Launch a local deployment:
  ```
  $ cargo spatial local launch
  ```

Its ready to go when you see a message like the following in your console.

```
Access the Inspector at http://localhost:21000/inspector
```

Navigate to http://localhost:21000/inspector in your browser to view your deployment. You should see 4 workers already connected,
these were started by the SpatialOS Runtime. 

If you want to manually launch another instance of the worker, run the following command from the `project-example` directory:

```
$ cargo run -- --worker-id RustWorker999 --worker-type RustWorker receptionist
```

This will allow you to see the log output of the worker as it runs.

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

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
