use regex::Regex;
use Result::{Err, Ok};
use crate::flags_parser::RawFlag::{BooleanShortForm, LongForm, ShortForm};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Flags {
    pub verbosity: bool,
    pub images: Vec<String>,
    pub padding: i32,
    pub background_color: String,
    pub data_format: Option<DataFormat>,
    pub filename: String,
    pub image_resolution: (i32, i32)
}

#[derive(Debug)]
enum ParseError {
    Basic
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: TBD")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DataFormat {
    Json,
    Xml
}

#[derive(Debug)]
enum RawFlag {
    ShortForm(String, String),
    LongForm(String, String),
    BooleanShortForm(String)
}

impl RawFlag {
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

    let verbosity: bool = extract_flag_and_parse(&raw_flags, String::from("v"), String::from("verbose"), Some(false), boolean_parser)?;
    let images: Vec<String> = extract_flag_and_parse(&raw_flags, String::from("i"), String::from("images"), None, list_parser)?;
    let padding: i32 = extract_flag_and_parse(&raw_flags, String::from("p"), String::from("padding"), Some(1), int_parser)?;
    let background_color: String = extract_flag_and_parse(&raw_flags, String::from("bg"), String::from("background"), Some(String::from("#000000")), |e| Ok(e))?;
    let data_format: Option<DataFormat> = extract_flag_and_parse(&raw_flags, String::from("df"), String::from("data_format"), None, data_format_parser).ok();
    let filename: String = extract_flag_and_parse(&raw_flags, String::from("f"), String::from("filename"), None, |e| Ok(e))?;
    let image_resolution = extract_flag_and_parse(&raw_flags, String::from("ir"), String::from("image_resolution"), None, resolution_parser)?;

    Ok(Flags {
        verbosity,
        images,
        padding,
        background_color,
        data_format,
        filename,
        image_resolution
    })
}

fn extract_flag_and_parse<T>(raw_flags: &Vec<RawFlag>,
                             short_form: String,
                             long_form: String,
                             default_value: Option<T>,
                             parser: Parser<T>) -> Result<T, Box<dyn Error>> {
    let result = raw_flags.iter().find(|&elem| {
        match elem {
            ShortForm(flag, _) if short_form.eq(flag) => true,
            LongForm(flag, _) if long_form.eq(flag) => true,
            BooleanShortForm(flag) if short_form.eq(flag) => true,
            _ => false
        }
    });

    match (result, default_value) {
        (None, None) => Err(Box::new(ParseError::Basic)), // TODO: more specialized error
        (None, Some(value)) => Ok(value),
        (Some(res), _) => parser(res.value())
    }
}

fn parse_string_to_raw_flags(args_raw: Vec<String>) -> Result<Vec<RawFlag>, Box<dyn Error>> {
    let short_form_reg_exp = Regex::new(r"-(\w*)")?;
    let long_form_reg_exp = Regex::new(r"--(\w*)=(\S*)")?;
    let mut acc: Vec<RawFlag> = Vec::new();

    'outer: for str in args_raw.iter() {
        for cap in long_form_reg_exp.captures(&str) {
            acc.push(LongForm(cap[1].parse()?, cap[2].parse()?));
            continue 'outer;
        }
        for cap in short_form_reg_exp.captures(&str) {
            acc.push(BooleanShortForm(cap[1].parse()?));
            continue 'outer;
        }
        if let Some(RawFlag::BooleanShortForm(flag)) = acc.pop() {
            acc.push(ShortForm(flag, str.parse()?));
            continue 'outer;
        }

        return Err(Box::new(ParseError::Basic)); // TODO: more specialized error
    }

    return Ok(acc);
}

fn boolean_parser(str: String) -> Result<bool, Box<dyn Error>> {
    match str.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Box::new(ParseError::Basic)) // TODO: more specialized error
    }
}

fn int_parser(str: String) -> Result<i32, Box<dyn Error>> {
    match str.parse::<i32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(Box::new(ParseError::Basic)) // TODO: more specialized error
    }
}

fn data_format_parser(str: String) -> Result<DataFormat, Box<dyn Error>> {
    match str.as_str() {
        "json" => Ok(DataFormat::Json),
        "xml" => Ok(DataFormat::Xml),
        _ => Err(Box::new(ParseError::Basic)) // TODO: more specialized error
    }
}

fn resolution_parser(str: String) -> Result<(i32, i32), Box<dyn Error>> {
    let split: Vec<&str> = str.split(",").collect();
    match split[..] {
        [x, y] =>
            match x.parse::<i32>().and_then(|x| {
                y.parse::<i32>().map(|y| (x, y))
            }) {
                Ok(v) => Ok(v),
                Err(_) => Err(Box::new(ParseError::Basic)) // TODO: more specialized error
            }
        _ => Err(Box::new(ParseError::Basic)) // TODO: more specialized error
    }
}

fn list_parser(str: String) -> Result<Vec<String>, Box<dyn Error>> {
    let replaced: String = str.chars().filter(|c| match c {
            '[' | ']' => false,
            c if c.is_whitespace() => false,
            _ => true
         })
        .collect();

    let split: Vec<&str> = replaced.split(",").collect();

    if split.is_empty() {
        Err(Box::new(ParseError::Basic)) // TODO: more specialized error
    } else {
        Ok(split.iter().map(|&e| String::from(e)).collect())
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
        if let RawFlag::LongForm(flag, value) = result.first().unwrap() {
            assert_eq!(flag, "test");
            assert_eq!(value, "lol.,j");
        }
    }

    #[test]
    fn parse_boolean_short_form() {
        let result = parse_string_to_raw_flags(vec![String::from("-f")]).unwrap();

        assert_eq!(result.len(), 1);
        if let RawFlag::BooleanShortForm(flag) = result.first().unwrap() {
            assert_eq!(flag, "f");
        }
    }

    #[test]
    fn parse_short_form() {
        let result = parse_string_to_raw_flags(vec![String::from("-f"), String::from("./test.txt")]).unwrap();

        assert_eq!(result.len(), 1);
        if let RawFlag::ShortForm(flag, value) = result.first().unwrap() {
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
    fn return_error_if_cannot_extract_flag_and_no_default_value() {
        let raw_flags = vec![LongForm(String::from("verbosity"), String::from("true"))];

        let result = extract_flag_and_parse(&raw_flags, String::from("t"), String::from("test"), None, boolean_parser);
        assert!(result.is_err(), "should not find flag")
    }

    #[test]
    fn return_default_value_if_cannot_extract_flag_and_some_default_value() {
        let raw_flags = vec![LongForm(String::from("verbosity"), String::from("true"))];

        let result = extract_flag_and_parse(&raw_flags, String::from("t"), String::from("test"), Some(true), boolean_parser);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn return_error_if_parser_error_and_no_default_value() {
        fn bad_parser(_: String) -> Result<bool, Box<dyn Error>> {
            Err(ParseError::Basic)?
        }

        let raw_flags = vec![LongForm(String::from("test1"), String::from("true"))];
        let result = extract_flag_and_parse(&raw_flags, String::from("t"), String::from("test1"), None, bad_parser);
        assert!(result.is_err())
    }

    #[test]
    fn return_ok_if_parser_long_form_ok_and_no_default_value() {
        let raw_flags = vec![LongForm(String::from("verbosity"), String::from("true"))];

        let result = extract_flag_and_parse(&raw_flags, String::from("v"), String::from("verbosity"), None, boolean_parser);
        assert!(result.unwrap())
    }

    #[test]
    fn return_ok_if_parser_short_form_ok_and_no_default_value() {
        let raw_flags = vec![ShortForm(String::from("v"), String::from("true"))];

        let result = extract_flag_and_parse(&raw_flags, String::from("v"), String::from("verbosity"), None, boolean_parser);
        assert!(result.unwrap())
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

    #[test]
    fn list_parser_test() {
        let result = list_parser(String::from("[./dibil.com, allo.me]")).unwrap();
        assert_eq!(result.first().unwrap(), &String::from("./dibil.com"));
        assert_eq!(result.last().unwrap(), &String::from("allo.me"))
    }

    #[test]
    fn int_parser_test() {
        let result = int_parser(String::from("wes"));
        assert!(result.is_err());
        let result = int_parser(String::from("42"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42)
    }

    #[test]
    fn data_format_parser_test() {
        assert_eq!(data_format_parser(String::from("json")).unwrap(), DataFormat::Json);
        assert_eq!(data_format_parser(String::from("xml")).unwrap(), DataFormat::Xml);
        assert_eq!(data_format_parser(String::from("xmjl")).is_err(), true);
    }

    #[test]
    fn resolution_parser_test() {
        assert_eq!(resolution_parser(String::from("123,453")).unwrap(), (123, 453));
        assert_eq!(resolution_parser(String::from("rre,123")).is_err(), true);
        assert_eq!(resolution_parser(String::from("true")).is_err(), true);
    }
}