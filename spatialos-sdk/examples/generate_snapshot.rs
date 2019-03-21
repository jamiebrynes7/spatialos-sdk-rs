use std::path::PathBuf;
use structopt::StructOpt;

use spatialos_sdk::worker::snapshot::*;

fn main() {
    let opt = Opt::from_args();
    let current_dir = std::env::current_dir().expect("Could not find current working directory.");

    let mut path_buf = PathBuf::new();
    path_buf.push(current_dir);
    path_buf.push(opt.snapshot_path);

    let snapshot_path = path_buf.to_str().unwrap();
    println!("Creating empty snapshot at: {}", snapshot_path);

    let _stream = SnapshotOutputStream::new(snapshot_path);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "generate_snapshot")]
struct Opt {
    /// Relative path for the snapshot to be written to.
    #[structopt(short = "p", long = "snapshot-path")]
    snapshot_path: PathBuf,
}
