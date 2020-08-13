# Getting Started

> **Note:** The instructions here are for using the `spatialos-sdk-rs` repository. They may differ if you are attempting to use this crate in your own project. Documentation for this workflow will be available in the future.

## Get the prerequisites

You'll need a few things before you can use the `spatialos-sdk-rs` crates.

1. At least Rust v1.40.
2. A [SpatialOS account](https://www.improbable.io/get-spatialos) 
3. The [Spatial CLI](https://documentation.improbable.io/spatialos-tools/docs/cli-introduction) installed and available on your `PATH`.

## Setup

1. Clone the `spatialos-sdk-rs` repository.

    ```
    $ git clone git@github.com:jamiebrynes7/spatialos-sdk-rs.git
    ```
2. Install `cargo-spatial`. This is a wrapper around the Spatial CLI and the Rust code generator.

    ```
    $ cargo install --path ./cargo-spatial --force
    ```

## Download native libraries

The `spatialos-sdk-sys` crate has some native C dependencies. The `cargo-spatial` tool offers a convenient way of downloading these. 

1. Set the `SPATIAL_LIB_DIR` environment variable to where you'd like these to be downloaded.
   
   ```
   $ export SPATIAL_LIB_DIR=$(pwd)/dependencies
   ```

2. Download the libraries.

    ```
    $ cargo spatial download sdk --sdk-version 14.5.0 --with-test-schema
    ```

## Run the example project

Now that you've got all the dependencies, its time to run the example project!

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

Navigate to [localhost:21000/inspector](http://localhost:21000/inspector) in your browser to view your deployment. You should see 4 workers already connected,
these were started by the SpatialOS Runtime. 

If you want to manually launch another instance of the worker, run the following command from the `project-example` directory:

```
$ cargo run -- --worker-id RustWorker999 --worker-type RustWorker receptionist
```

This will allow you to see the log output of the worker as it runs.