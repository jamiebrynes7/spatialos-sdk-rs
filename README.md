# SpatialOS SDK for Rust

> This is an **unoffical**, **unsupported**, and **untested** integration of the [SpatialOS SDK C API bindings](https://docs.improbable.io/reference/13.3/capi/introduction) with Rust. Improbable does not officially support Rust as a worker language.

This is still heavily WIP and should be treated as such. Progress can be seen in the `Projects` boards which define milestones and progress toward those milestones. When this reaches feature parity with the C API and has basic code generation in which all user facing APIs are safe, a crate will be published to `crates.io`.

> Note that this has only been tested on Linux!

## Setup 

1. Clone this repository.
2. Run `cargo run --bin download_sdk -- -d ./spatialos-sdk/dependencies -s 13.3.0` to download the C API dependencies.
3. Run `cargo build` 

If these steps complete successfully, the `spatialos-sdk` crate has been built and linked successfully and can be used in user code.

For an example of this, see the `project-example` in `examples/`. Description and instructions to follow.. 

