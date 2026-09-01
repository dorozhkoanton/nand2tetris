mod address;
mod assembler;
mod code;
mod instruction;
mod parser;
mod symbol;
mod symbol_table;

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::assembler::{AssemblerError, assemble};

fn main() -> Result<(), AssemblerError> {
    let args: Vec<String> = env::args().collect();

    let source_file_path_str = match args.len() {
        2 => &args[1],
        _ => {
            return Err(AssemblerError::IO(io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: <program> <file_name>",
            )));
        }
    };

    let source_file_path = Path::new(source_file_path_str);

    if let Some(extension) = source_file_path.extension()
        && extension != "asm"
    {
        return Err(AssemblerError::IO(io::Error::new(
            io::ErrorKind::InvalidInput,
            "file extension should be '.asm'",
        )));
    }

    let source_file = File::open(source_file_path_str)?;

    let output_file_path = source_file_path.with_extension("hack");
    let output_file = File::create(output_file_path)?;

    assemble(source_file, output_file)?;

    Ok(())
}
