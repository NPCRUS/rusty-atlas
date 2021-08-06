use regex::Regex;
use Result::{Err, Ok};
use crate::args_parser::Argument::{BooleanShortForm, LongForm, ShortForm};
use crate::args_parser::ParseError::InvalidArgumentOrder;

#[derive(Debug)]
pub struct Arguments {
    folder: String
}

#[derive(Debug)]
pub enum ParseError {
    InvalidArgumentOrder(String)
}

#[derive(Debug)]
enum Argument {
    ShortForm(String, String),
    LongForm(String, String),
    BooleanShortForm(String)
}

impl Argument {
    fn flag(&self) -> &String {
        match self {
            ShortForm(flag, _) => flag,
            LongForm(flag, _) => flag,
            BooleanShortForm(flag) => flag
        }
    }
}

pub fn parse_arguments(args_raw: Vec<String>) -> Arguments {
    Arguments {
        folder: String::from("./whatever")
    }
}

fn parse_rec(args_raw: Vec<String>) -> Result<Vec<Argument>, ParseError> {
    let short_form_reg_exp = Regex::new(r"-(\w*)").unwrap();
    let long_form_reg_exp = Regex::new(r"--(\w*)=(\S*)").unwrap();
    let mut acc: Vec<Argument> = Vec::new();

    'outer: for str in args_raw.iter() {
        for cap in long_form_reg_exp.captures(&str) {
            acc.push(LongForm(cap[1].parse().unwrap(), cap[2].parse().unwrap()));
            continue 'outer;
        }
        for cap in short_form_reg_exp.captures(&str) {
            acc.push(BooleanShortForm(cap[1].parse().unwrap()));
            continue 'outer;
        }
        if let Some(Argument::BooleanShortForm(flag)) = acc.pop() {
            acc.push(ShortForm(flag, str.parse().unwrap()));
            continue 'outer;
        }
        let fallback_arg = BooleanShortForm(String::from("exec"));
        let prev_arg = acc.last().unwrap_or(&fallback_arg);
        return Err(InvalidArgumentOrder(
            String::from(String::from("Invalid token `") + str + &*String::from("` after `") + prev_arg.flag() + &*String::from("`"))
        ));
    }

    return Ok(acc);
}

#[test]
fn parse_long_form() {
    let result = parse_rec(vec![String::from("--test=lol.,j")]).unwrap();

    assert_eq!(result.len(), 1);
    if let Argument::LongForm(flag, value) = result.first().unwrap() {
        assert_eq!(flag, "test");
        assert_eq!(value, "lol.,j");
    }
}

#[test]
fn parse_boolean_short_form() {
    let result = parse_rec(vec![String::from("-f")]).unwrap();

    assert_eq!(result.len(), 1);
    if let Argument::BooleanShortForm(flag) = result.first().unwrap() {
        assert_eq!(flag, "f");
    }
}

#[test]
fn parse_short_form() {
    let result = parse_rec(vec![String::from("-f"), String::from("./test.txt")]).unwrap();

    assert_eq!(result.len(), 1);
    if let Argument::ShortForm(flag, value) = result.first().unwrap() {
        assert_eq!(flag, "f");
        assert_eq!(value, "./test.txt")
    }
}

#[test]
fn return_invalid_argument_order() {
    let result = parse_rec(vec![String::from("--file=lol"), String::from("./test.txt")]).err().unwrap();

    if let InvalidArgumentOrder(err) = result {

    } else {
        panic!("shouldBe InvalidArgumentError")
    }
}