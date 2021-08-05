mod args_parser;

use std::env;
use crate::args_parser::{Arguments, parse_arguments};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_arguments(args);
    println!("{:?}", parsed_args);
}