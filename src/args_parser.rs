use std::fmt::Error;
use regex::Regex;
use Result::{Err, Ok};

#[derive(Debug)]
pub struct Arguments {
    folder: String
}

enum Argument {
    ShortForm(String, String),
    LongForm(String, String)
}

pub fn parse_arguments(args_raw: Vec<String>) -> Arguments {
    let iter = args_raw.iter();
    Arguments {
        folder: String::from("./whatever")
    }
}

fn parse_rec(args_raw: Vec<String>, acc: Vec<Argument>) -> Result<Vec<Argument>, Error> {
    let short_form_reg_exp = Regex::new(r"-\w*").unwrap();
    let long_form_reg_exp = Regex::new(r"--\w*=\S*").unwrap();

    unimplemented!("recursive call with parsing tokens");
}