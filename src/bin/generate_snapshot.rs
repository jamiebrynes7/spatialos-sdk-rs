extern crate spatialos_sdk;

use std::env;
use std::path::PathBuf;

use spatialos_sdk::worker::core::parameters::SnapshotParameters;
use spatialos_sdk::worker::core::snapshot::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = std::env::current_dir().expect("Could not find current working directory.");

    if args.len() != 2 {
        panic!(
            "Incorrect usage. Expected usage: cargo run --bin generate_snapshot <path-to-snapshot>"
        );
    }

    let mut path_buf = PathBuf::new();
    path_buf.push(current_dir);
    path_buf.push(args[1].clone());

    let snapshot_path = path_buf.to_str().unwrap();
    println!("Creating empty snapshot at: {}", snapshot_path);

    let stream = SnapshotOutputStream::new(snapshot_path, SnapshotParameters {});
}
