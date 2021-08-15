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

    fn value(&self) -> String {
        match self {
            ShortForm(_, value) => value.to_string(),
            LongForm(_, value) => value.to_string(),
            BooleanShortForm(_) => String::from("true")
        }
    }
}

type Parser<T> = fn(String) -> Result<T, Box<dyn Error>>;

pub fn parse_args(args_raw: Vec<String>) -> Result<Flags, Box<dyn Error>> {
    let raw_flags = parse_string_to_raw_flags(args_raw)?;

    let verbosity: bool = extract_flag_and_parse(&raw_flags, String::from("v"), String::from("verbosity"), boolean_parser)?;

    Ok(Flags {
        verbosity
    })
}

fn extract_flag_and_parse<T>(raw_flags: &Vec<RawFlags>, short_form: String, long_form: String, parser: Parser<T>) -> Result<T, Box<dyn Error>> {
    let result = raw_flags.iter().find(|&elem| {
        match elem {
            ShortForm(flag, _) if short_form.eq(flag) => true,
            LongForm(flag, _) if long_form.eq(flag) => true,
            BooleanShortForm(flag) if short_form.eq(flag) => true,
            _ => false
        }
    }).ok_or(ParseError)?; // TODO: more specialized error

    parser(result.value())
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

        return Err(ParseError)?; // TODO: more specialized error
    }

    return Ok(acc);
}

fn boolean_parser(str: String) -> Result<bool, Box<dyn Error>> {
    match str.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ParseError)? // TODO: more specialized error
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // parse_string_to_raw_flags tests
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

    //extract_flag_and_parse tests
    #[test]
    fn return_error_if_cannot_extract_flag() {
        let raw_flags = vec![LongForm(String::from("verbosity"), String::from("true"))];

        let result = extract_flag_and_parse(&raw_flags, String::from("t"), String::from("test"), boolean_parser);
        assert!(result.is_err(), "should not find flag")
    }

    fn return_error_if_parser_error() {
        fn bad_parser(str: String) -> Result<bool, Box<dyn Error>> {
            Err(ParseError)?
        }

        let raw_flags = vec![LongForm(String::from("test1"), String::from("true"))];
        let result = extract_flag_and_parse(&raw_flags, String::from("t"), String::from("test1"), boolean_parser);
        assert!(result.is_err(), "should not find flag")
    }

    #[test]
    fn boolean_parser_test() {
        let result = boolean_parser(String::from("true")).unwrap();
        assert_eq!(result, true);
        let result = boolean_parser(String::from("false")).unwrap();
        assert_eq!(result, false);
        let error = boolean_parser(String::from("True"));
        assert!(error.is_err())
    }
}