mod flags_parser;
mod flags_description;

use std::env;
use crate::flags_parser::{Flags, parse_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_args(args);
    println!("{:?}", parsed_args);
}