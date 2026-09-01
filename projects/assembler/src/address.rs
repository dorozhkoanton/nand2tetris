use std::{error::Error, fmt::Display, str::FromStr};

const MAX_ADDRESS: u16 = 32767;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address {
    value: u16,
}

impl Address {
    pub fn new(value: u16) -> Result<Self, AddressRangeError> {
        if !Self::is_valid(value) {
            return Err(AddressRangeError(value.to_string()));
        }

        Ok(Self { value })
    }

    fn is_valid(value: u16) -> bool {
        value <= MAX_ADDRESS
    }

    pub fn next(&self) -> Result<Self, AddressRangeError> {
        Self::new(self.value + 1)
    }

    pub fn to_bits(self) -> u16 {
        self.value
    }
}

impl TryFrom<u16> for Address {
    type Error = AddressRangeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Address {
    type Err = AddressRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u16 = s.parse().map_err(|_| AddressRangeError(s.to_string()))?;

        Self::new(value)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct AddressRangeError(String);

impl Error for AddressRangeError {}

impl Display for AddressRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "address out of range: {}", self.0)
    }
}
