use std::{error::Error, fmt::Display, str::FromStr};

//
// Code
//

pub struct Code {
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

impl Code {
    pub fn new(dest_str: &str, comp_str: &str, jump_str: &str) -> Result<Self, ParseCodeError> {
        let dest = Dest::from_str(dest_str)?;
        let comp = Comp::from_str(comp_str)?;
        let jump = Jump::from_str(jump_str)?;

        Ok(Code { dest, comp, jump })
    }

    pub fn to_bits(&self) -> u16 {
        let code = 0b111 << 13;
        let comp = self.comp.to_bits() << 6;
        let dest = self.dest.to_bits() << 3;
        let jump = self.jump.to_bits();

        code | comp | dest | jump
    }
}

#[derive(Debug)]
pub enum ParseCodeError {
    Dest(ParseDestError),
    Comp(ParseCompError),
    Jump(ParseJumpError),
}

impl Error for ParseCodeError {}

impl From<ParseDestError> for ParseCodeError {
    fn from(e: ParseDestError) -> Self {
        Self::Dest(e)
    }
}
impl From<ParseCompError> for ParseCodeError {
    fn from(e: ParseCompError) -> Self {
        Self::Comp(e)
    }
}
impl From<ParseJumpError> for ParseCodeError {
    fn from(e: ParseJumpError) -> Self {
        Self::Jump(e)
    }
}

impl Display for ParseCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dest(e) => write!(f, "dest field: {e}"),
            Self::Comp(e) => write!(f, "comp field: {e}"),
            Self::Jump(e) => write!(f, "jump field: {e}"),
        }
    }
}

//
// Dest
//

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dest {
    NULL,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

impl Dest {
    const fn to_bits(self) -> u16 {
        match self {
            Self::NULL => 0b000,
            Self::M => 0b001,
            Self::D => 0b010,
            Self::DM => 0b011,
            Self::A => 0b100,
            Self::AM => 0b101,
            Self::AD => 0b110,
            Self::ADM => 0b111,
        }
    }
}

impl FromStr for Dest {
    type Err = ParseDestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "null" => Ok(Self::NULL),
            "M" => Ok(Self::M),
            "D" => Ok(Self::D),
            "DM" | "MD" => Ok(Self::DM),
            "A" => Ok(Self::A),
            "AM" | "MA" => Ok(Self::AM),
            "AD" | "DA" => Ok(Self::AD),
            "ADM" | "AMD" | "DAM" | "DMA" | "MAD" | "MDA" => Ok(Self::ADM),
            other => Err(ParseDestError(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ParseDestError(String);

impl Error for ParseDestError {}

impl Display for ParseDestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown dest mnemonic: {}", self.0)
    }
}

//
// Comp
//

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comp {
    Zero,
    PositiveOne,
    NegativeOne,
    D,
    A,
    M,
    NotD,
    NotA,
    NotM,
    NegativeD,
    NegativeA,
    NegativeM,
    DPlusOne,
    APlusOne,
    MPlusOne,
    DMinusOne,
    AMinusOne,
    MMinusOne,
    DPlusA,
    DPlusM,
    DMinusA,
    DMinusM,
    AMinusD,
    MMinusD,
    DAndA,
    DAndM,
    DOrA,
    DOrM,
}

impl Comp {
    const fn to_bits(self) -> u16 {
        match self {
            Self::Zero => 0b0101010,
            Self::PositiveOne => 0b0111111,
            Self::NegativeOne => 0b0111010,
            Self::D => 0b0001100,
            Self::A => 0b0110000,
            Self::M => 0b1110000,
            Self::NotD => 0b0001101,
            Self::NotA => 0b0110001,
            Self::NotM => 0b1110001,
            Self::NegativeD => 0b0001111,
            Self::NegativeA => 0b0110011,
            Self::NegativeM => 0b1110011,
            Self::DPlusOne => 0b0011111,
            Self::APlusOne => 0b0110111,
            Self::MPlusOne => 0b1110111,
            Self::DMinusOne => 0b0001110,
            Self::AMinusOne => 0b0110010,
            Self::MMinusOne => 0b1110010,
            Self::DPlusA => 0b0000010,
            Self::DPlusM => 0b1000010,
            Self::DMinusA => 0b0010011,
            Self::DMinusM => 0b1010011,
            Self::AMinusD => 0b0000111,
            Self::MMinusD => 0b1000111,
            Self::DAndA => 0b0000000,
            Self::DAndM => 0b1000000,
            Self::DOrA => 0b0010101,
            Self::DOrM => 0b1010101,
        }
    }
}

impl FromStr for Comp {
    type Err = ParseCompError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::PositiveOne),
            "-1" => Ok(Self::NegativeOne),
            "D" => Ok(Self::D),
            "A" => Ok(Self::A),
            "M" => Ok(Self::M),
            "!D" => Ok(Self::NotD),
            "!A" => Ok(Self::NotA),
            "!M" => Ok(Self::NotM),
            "-D" => Ok(Self::NegativeD),
            "-A" => Ok(Self::NegativeA),
            "-M" => Ok(Self::NegativeM),
            "D+1" => Ok(Self::DPlusOne),
            "A+1" => Ok(Self::APlusOne),
            "M+1" => Ok(Self::MPlusOne),
            "D-1" => Ok(Self::DMinusOne),
            "A-1" => Ok(Self::AMinusOne),
            "M-1" => Ok(Self::MMinusOne),
            "D+A" => Ok(Self::DPlusA),
            "D+M" => Ok(Self::DPlusM),
            "D-A" => Ok(Self::DMinusA),
            "D-M" => Ok(Self::DMinusM),
            "A-D" => Ok(Self::AMinusD),
            "M-D" => Ok(Self::MMinusD),
            "D&A" => Ok(Self::DAndA),
            "D&M" => Ok(Self::DAndM),
            "D|A" => Ok(Self::DOrA),
            "D|M" => Ok(Self::DOrM),
            other => Err(ParseCompError(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ParseCompError(String);

impl Error for ParseCompError {}

impl Display for ParseCompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown comp mnemonic: {}", self.0)
    }
}

//
// Jump
//

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jump {
    NULL,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Jump {
    const fn to_bits(self) -> u16 {
        match self {
            Self::NULL => 0b000,
            Self::JGT => 0b001,
            Self::JEQ => 0b010,
            Self::JGE => 0b011,
            Self::JLT => 0b100,
            Self::JNE => 0b101,
            Self::JLE => 0b110,
            Self::JMP => 0b111,
        }
    }
}

impl FromStr for Jump {
    type Err = ParseJumpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "null" => Ok(Self::NULL),
            "JGT" => Ok(Self::JGT),
            "JEQ" => Ok(Self::JEQ),
            "JGE" => Ok(Self::JGE),
            "JLT" => Ok(Self::JLT),
            "JNE" => Ok(Self::JNE),
            "JLE" => Ok(Self::JLE),
            "JMP" => Ok(Self::JMP),
            other => Err(ParseJumpError(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ParseJumpError(String);

impl Error for ParseJumpError {}

impl Display for ParseJumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown jump mnemonic: {}", self.0)
    }
}
