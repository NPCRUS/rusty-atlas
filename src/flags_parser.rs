use regex::Regex;
use Result::{Err, Ok};
use crate::flags_parser::RawFlags::{BooleanShortForm, LongForm, ShortForm};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Flags {
    pub verbosity: bool
}

#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: TBD")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
enum RawFlags {
    ShortForm(String, String),
    LongForm(String, String),
    BooleanShortForm(String)
}

impl RawFlags {
    fn flag(&self) -> &String {
        match self {
            ShortForm(flag, _) => flag,
            LongForm(flag, _) => flag,
            BooleanShortForm(flag) => flag
        }
    }
}

pub fn parse_args(args_raw: Vec<String>) -> Result<Flags, Box<dyn Error>> {
    let raw_flags = parse_string_to_raw_flags(args_raw)?;
    raw_flags_to_flags(raw_flags)
}

fn raw_flags_to_flags(raw_flags: Vec<RawFlags>) -> Result<Flags, Box<dyn Error>> {
    unimplemented!()
}

fn parse_string_to_raw_flags(args_raw: Vec<String>) -> Result<Vec<RawFlags>, Box<dyn Error>> {
    let short_form_reg_exp = Regex::new(r"-(\w*)")?;
    let long_form_reg_exp = Regex::new(r"--(\w*)=(\S*)")?;
    let mut acc: Vec<RawFlags> = Vec::new();

    'outer: for str in args_raw.iter() {
        for cap in long_form_reg_exp.captures(&str) {
            acc.push(LongForm(cap[1].parse()?, cap[2].parse()?));
            continue 'outer;
        }
        for cap in short_form_reg_exp.captures(&str) {
            acc.push(BooleanShortForm(cap[1].parse()?));
            continue 'outer;
        }
        if let Some(RawFlags::BooleanShortForm(flag)) = acc.pop() {
            acc.push(ShortForm(flag, str.parse()?));
            continue 'outer;
        }

        return Err(ParseError)?;
    }

    return Ok(acc);
}

fn boolean_parser(str: String) -> Result<bool, Box<dyn Error>> {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_long_form() {
        let result = parse_string_to_raw_flags(vec![String::from("--test=lol.,j")]).unwrap();

        assert_eq!(result.len(), 1);
        if let RawFlags::LongForm(flag, value) = result.first().unwrap() {
            assert_eq!(flag, "test");
            assert_eq!(value, "lol.,j");
        }
    }

    #[test]
    fn parse_boolean_short_form() {
        let result = parse_string_to_raw_flags(vec![String::from("-f")]).unwrap();

        assert_eq!(result.len(), 1);
        if let RawFlags::BooleanShortForm(flag) = result.first().unwrap() {
            assert_eq!(flag, "f");
        }
    }

    #[test]
    fn parse_short_form() {
        let result = parse_string_to_raw_flags(vec![String::from("-f"), String::from("./test.txt")]).unwrap();

        assert_eq!(result.len(), 1);
        if let RawFlags::ShortForm(flag, value) = result.first().unwrap() {
            assert_eq!(flag, "f");
            assert_eq!(value, "./test.txt");
        }
    }

    #[test]
    fn return_invalid_argument_order() {
        let result = parse_string_to_raw_flags(vec![String::from("--file=lol"), String::from("./test.txt")]);

        if let Ok(_) = result {
           panic!("should return Err(ParseError)");
        }
    }
}