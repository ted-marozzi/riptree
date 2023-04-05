use std::{path::Path};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "riptree", about = "Tree but in Rust", version)]
struct Args {
    path: Option<std::path::PathBuf>,
}
fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or(Path::new(".").to_path_buf());

    riptree::run(&path)
}
