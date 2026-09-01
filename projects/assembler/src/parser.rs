use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use crate::instruction::{Instruction, ParseInstructionError};

pub struct Parser {
    lines: Lines<BufReader<File>>,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let reader = BufReader::new(file);

        Self {
            lines: reader.lines(),
        }
    }
}

impl Iterator for Parser {
    type Item = Result<Instruction, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in &mut self.lines {
            match line {
                Ok(l) => {
                    let instruction_str = l.split("//").next().unwrap_or("").trim();
                    if instruction_str.is_empty() {
                        continue;
                    }

                    match Instruction::new(instruction_str) {
                        Ok(instruction) => return Some(Ok(instruction)),
                        Err(e) => return Some(Err(e.into())),
                    }
                }
                Err(e) => {
                    return Some(Err(e.into()));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
pub enum ParserError {
    IO(std::io::Error),
    Instruction(ParseInstructionError),
}

impl Error for ParserError {}

impl From<std::io::Error> for ParserError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<ParseInstructionError> for ParserError {
    fn from(e: ParseInstructionError) -> Self {
        Self::Instruction(e)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "error while reading file: {e}"),
            Self::Instruction(e) => write!(f, "error while parsing file: {e}"),
        }
    }
}
