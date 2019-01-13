use bindgen;
use clap::{App, Arg};
use glob::glob;

use std::path::PathBuf;

const OUTPUT_DIR_ARG: &str = "output_dir";
const INPUT_DIR_ARG: &str = "input_dir";

struct Args {
    pub input_dir: String,
    pub output_dir: String,
}

fn main() {
    let args = get_arguments();

    let headers =
        glob(&format!("{}/*.h", args.input_dir)).expect("Could not glob input directory.");

    let mut bindings = bindgen::Builder::default().layout_tests(false);

    for path in headers {
        bindings = bindings.header(path.unwrap().as_path().to_str().unwrap().to_owned())
    }

    let mut out_path = PathBuf::from(args.output_dir);
    out_path.push("worker.rs");

    bindings
        .generate()
        .expect("Could not generate bindings.")
        .write_to_file(out_path)
        .expect("Could not write bindings");
}

fn get_arguments() -> Args {
    let matches = App::new("Spatial OS SDK Bindings Generator")
        .author("Jamie Brynes <jamiebrynes7@gmail.com>")
        .about("Generate Rust bindings for the SpatialOS C API.")
        .arg(
            Arg::with_name(OUTPUT_DIR_ARG)
                .short("o")
                .long(OUTPUT_DIR_ARG)
                .takes_value(true)
                .value_name(&OUTPUT_DIR_ARG.to_uppercase())
                .required(true)
                .help("Output directory for the Rust bindings. Relative to the current working directory.")
        )
        .arg(
            Arg::with_name(INPUT_DIR_ARG)
                .short("i")
                .long(INPUT_DIR_ARG)
                .takes_value(true)
                .value_name(&INPUT_DIR_ARG.to_uppercase())
                .required(true)
                .help("Input directory for C header files. Relative to the current working directory.")
        )
        .get_matches();

    Args {
        input_dir: matches.value_of(INPUT_DIR_ARG).unwrap().to_owned(),
        output_dir: matches.value_of(OUTPUT_DIR_ARG).unwrap().to_owned(),
    }
}
