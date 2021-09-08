mod atlas_maker;
mod flags_parser;

use crate::atlas_maker::*;
use crate::flags_parser::{parse_args, Flags};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_args(args);
    println!("{:?}", parsed_args);

    // TODO: test usage, remove
    // let mut file_paths = Vec::new();
    // file_paths.push(String::from("./test_assets/RustyAtlas_R.png"));
    // file_paths.push(String::from("./test_assets/RustyAtlas_A.png"));
    //
    // make_atlas(file_paths);
}
