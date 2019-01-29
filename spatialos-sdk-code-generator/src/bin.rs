extern crate spatialos_sdk_code_generator;

use spatialos_sdk_code_generator::generator;
use spatialos_sdk_code_generator::schema_bundle;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn main() {
    let input_filename = args().nth(1).unwrap();
    let output_filename = args().nth(2).unwrap();

    let mut input_file =
        File::open(input_filename).expect("Unable to open the test schema bundle.");
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .expect("Unable to read the test schema bundle");
    let generated_file = generator::generate_code(schema_bundle::load_bundle(&contents).unwrap());
    let mut output_file = File::create(output_filename).unwrap();
    output_file.write_all(generated_file.as_bytes()).unwrap();
}
