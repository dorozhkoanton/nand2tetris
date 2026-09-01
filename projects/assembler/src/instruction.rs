use crate::address::{Address, AddressRangeError};
use crate::code::{Code, ParseCodeError};
use crate::symbol::{ParseSymbolError, Symbol};
use std::{error::Error, fmt::Display, str::FromStr};

//
// Instruction
//

pub enum Instruction {
    A(AddressRef), // @variable or @16
    C(Code),       // dest=comp;jump
    L(Symbol),     // (LABEL)
}

impl Instruction {
    pub fn new(s: &str) -> Result<Self, ParseInstructionError> {
        let instruction_str = s.trim();

        if instruction_str.is_empty() || instruction_str.starts_with("//") {
            todo!()
        } else if let Some(address_ref_str) = instruction_str.strip_prefix('@') {
            let address_ref = AddressRef::new(address_ref_str)?;
            Ok(Self::A(address_ref))
        } else if let Some(label_str) = instruction_str
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
        {
            let label = Symbol::new(label_str)?;
            Ok(Self::L(label))
        } else {
            let (dest_comp_str, jump_str) = instruction_str
                .rsplit_once(';')
                .unwrap_or((instruction_str, "null"));

            let (dest_str, comp_str) = dest_comp_str
                .split_once('=')
                .unwrap_or(("null", dest_comp_str));

            let code = Code::new(dest_str, comp_str, jump_str)?;
            Ok(Self::C(code))
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Instruction::new(s)
    }
}

#[derive(Debug)]
pub enum ParseInstructionError {
    A(ParseAddressRefError),
    C(ParseCodeError),
    L(ParseSymbolError),
}

impl Error for ParseInstructionError {}

impl From<ParseAddressRefError> for ParseInstructionError {
    fn from(e: ParseAddressRefError) -> Self {
        Self::A(e)
    }
}

impl From<ParseCodeError> for ParseInstructionError {
    fn from(e: ParseCodeError) -> Self {
        Self::C(e)
    }
}

impl From<ParseSymbolError> for ParseInstructionError {
    fn from(e: ParseSymbolError) -> Self {
        Self::L(e)
    }
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(e) => write!(f, "A Instruction: {e}"),
            Self::C(e) => write!(f, "C Instruction: {e}"),
            Self::L(e) => write!(f, "L Instruction: {e}"),
        }
    }
}

//
// AddressRef
//

pub enum AddressRef {
    Numeric(Address),
    Symbolic(Symbol),
}

impl AddressRef {
    fn new(s: &str) -> Result<Self, ParseAddressRefError> {
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self::Numeric(s.parse::<Address>()?))
        } else {
            Ok(Self::Symbolic(s.parse::<Symbol>()?))
        }
    }
}

impl FromStr for AddressRef {
    type Err = ParseAddressRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AddressRef::new(s)
    }
}

#[derive(Debug)]
pub enum ParseAddressRefError {
    Numeric(AddressRangeError),
    Symbolic(ParseSymbolError),
}

impl Error for ParseAddressRefError {}

impl From<AddressRangeError> for ParseAddressRefError {
    fn from(e: AddressRangeError) -> Self {
        Self::Numeric(e)
    }
}

impl From<ParseSymbolError> for ParseAddressRefError {
    fn from(e: ParseSymbolError) -> Self {
        Self::Symbolic(e)
    }
}

impl Display for ParseAddressRefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric(e) => write!(f, "numeric address: {e}"),
            Self::Symbolic(e) => write!(f, "symbolic address: {e}"),
        }
    }
}
