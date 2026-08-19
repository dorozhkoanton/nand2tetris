use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;

pub enum Instruction {
    Address(String),
    Label(String),
    Compute {
        dest: String,
        comp: String,
        jump: String,
    },
}

impl Instruction {
    fn from_string(s: &String) -> Option<Self> {
        if let Some(address) = Self::try_to_address(s) {
            return Some(address);
        } else if let Some(label) = Self::try_to_label(s) {
            return Some(label);
        } else if let Some(compute) = Self::try_to_compute(s) {
            return Some(compute);
        }
        None
    }

    fn try_to_address(s: &String) -> Option<Self> {
        todo!()
    }

    fn try_to_label(s: &String) -> Option<Self> {
        todo!()
    }

    fn try_to_compute(s: &String) -> Option<Self> {
        todo!()
    }
}

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
        for res in &mut self.peekable {
            match res {
                Ok(val) => {
                    let instruction = Instruction::from_string(&val);

                    if instruction.is_some() {
                        return instruction;
                    } else {
                        println!("Non-instruction line: {:?}", val);
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
