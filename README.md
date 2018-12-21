# SpatialOS SDK for Rust

[![Build Status](https://travis-ci.org/jamiebrynes7/spatialos-sdk-rs.svg?branch=master)](https://travis-ci.org/jamiebrynes7/spatialos-sdk-rs) [![dependency status](https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs/status.svg)](https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs) ![Rustc Version](https://img.shields.io/badge/rustc-1.31-blue.svg)


> This is an **unoffical**, **unsupported**, and **untested** integration of the [SpatialOS SDK C API bindings](https://docs.improbable.io/reference/13.3/capi/introduction) with Rust. Improbable does not officially support Rust as a worker language.

This is still heavily WIP and should be treated as such. Progress can be seen in the `Projects` boards which define milestones and progress toward those milestones. When this reaches feature parity with the C API and has basic code generation in which all user facing APIs are safe, a crate will be published to `crates.io`.

## Setup 

1. Clone this repository.
2. Run `cargo run --bin download_sdk -- -d dependencies -s 13.3.1` to download the C API dependencies.
3. Run `cargo build` 

If these steps complete successfully, the `spatialos-sdk` crate has been built and linked successfully and can be used in user code.

## Running the Example Project

To run the example project, you will need to:

1. Build a release version of the RustWorker - `cargo build --example project-example --release`.
2. Build the schema descriptor for Spatial - `./spatialos-sdk/examples/project-example/setup.sh`
3. In two terminals:
   - Navigate to the `spatialos` directory and start spatial: `cd spatialos-sdk/examples/project-example/spatialos/ && spatial local launch`
   - Run the example project worker - `cargo run --example project-example -- receptionist --worker_type RustWorker`

