use std::{
    fmt::Debug,
    io::{self, Read},
    num,
};

pub fn parse_input<TRead: Read>(reader: &mut TRead) -> Result<InputData, ParseError> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    // TODO: implement
    println!("{}", content);

    todo!()
}

#[derive(Debug)]
pub struct InputData {}

pub struct ParseError {
    message: String,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error in parsing input: {}", self.message))
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
