use crate::code::Code;
use std::str::FromStr;

pub enum Instruction {
    Address(String),
    Label(String),
    Compute(Code),
}

impl Instruction {
    fn try_to_address(s: &str) -> Option<Self> {
        todo!()
    }

    fn try_to_label(s: &str) -> Option<Self> {
        todo!()
    }

    fn try_to_compute(s: &str) -> Option<Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInstructionError {
    msg: String,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_to_address(s)
            .or_else(|| Self::try_to_label(s))
            .or_else(|| Self::try_to_compute(s))
            .ok_or(ParseInstructionError {
                msg: "Cannot parse instruction from string: {s}".to_string(),
            })
    }
}
