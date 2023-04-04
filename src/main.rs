use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() == 0 || args.len() == 1 {
        Path::new(".")
    } else if args.len() == 2 {
        Path::new(args.get(1).unwrap())
    } else {
        panic!("riptree received more than one argument: {:?}.\nriptree only accepts one argument for the Path to tree which defaults to current working directory.", args)
    };

    riptree::run(path)
}
