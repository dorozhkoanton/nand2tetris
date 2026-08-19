use std::str::FromStr;

pub struct Code {
    dest: String,
    comp: String,
    jump: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCodeError {
    msg: String,
}

impl FromStr for Code {
    type Err = ParseCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Dest {
    text: String,
    binary: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDestError {
    msg: String,
}

impl FromStr for Dest {
    type Err = ParseDestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
