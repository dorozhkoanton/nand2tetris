use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::address::{Address, AddressRangeError};
use crate::instruction::{AddressRef, Instruction};
use crate::parser::{Parser, ParserError};
use crate::symbol_table::{DuplicateLabelError, SymbolTable};

pub fn assemble(source_file: File, output_file: File) -> Result<(), AssemblerError> {
    let mut buf_writer = BufWriter::new(output_file);

    let parser = Parser::new(source_file);

    let mut symbol_table = SymbolTable::new();
    let mut rom_address = Address::new(0).expect("predefined address always valid");

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut output_lines: Vec<String> = Vec::new();

    for instruction_or_error in parser {
        let instruction = instruction_or_error?;

        if let Instruction::L(label) = instruction {
            symbol_table.bind_label(label, rom_address)?;
        } else {
            instructions.push(instruction);
            rom_address = rom_address.next()?;
        }
    }

    for instruction in instructions {
        let bits = match instruction {
            Instruction::A(address_ref) => match address_ref {
                AddressRef::Numeric(address) => address.to_bits(),
                AddressRef::Symbolic(symbol) => {
                    symbol_table.get_or_insert_variable(symbol)?.to_bits()
                }
            },
            Instruction::C(code) => code.to_bits(),
            Instruction::L(_) => unreachable!(),
        };

        let line = format!("{:016b}", bits);
        output_lines.push(line);
    }

    for line in output_lines {
        writeln!(buf_writer, "{line}")?;
    }

    buf_writer.flush()?;

    Ok(())
}

#[derive(Debug)]
pub enum AssemblerError {
    IO(std::io::Error),
    Parser(ParserError),
    AddressRange(AddressRangeError),
    DuplicateLabel(DuplicateLabelError),
}

impl Error for AssemblerError {}

impl From<std::io::Error> for AssemblerError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<ParserError> for AssemblerError {
    fn from(e: ParserError) -> Self {
        Self::Parser(e)
    }
}

impl From<AddressRangeError> for AssemblerError {
    fn from(e: AddressRangeError) -> Self {
        Self::AddressRange(e)
    }
}

impl From<DuplicateLabelError> for AssemblerError {
    fn from(e: DuplicateLabelError) -> Self {
        Self::DuplicateLabel(e)
    }
}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "{e}"),
            Self::Parser(e) => write!(f, "{e}"),
            Self::AddressRange(e) => write!(f, "{e}"),
            Self::DuplicateLabel(e) => write!(f, "{e}"),
        }
    }
}
