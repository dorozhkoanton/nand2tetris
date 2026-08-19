use crate::instruction::Instruction;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::str::FromStr;

pub struct Parser {
    peekable: Peekable<Lines<BufReader<File>>>,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let reader = BufReader::new(file);
        let peekable = reader.lines().peekable();

        Self { peekable }
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        for result in &mut self.peekable {
            match result {
                Ok(line) => {
                    let res = Instruction::from_str(&line);

                    match res {
                        Ok(instruction) => return Some(instruction),
                        Err(err) => {
                            eprintln!("{:?}", err);
                            continue;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Reading file error: {}", err);
                    return None;
                }
            }
        }
        None
    }
}
