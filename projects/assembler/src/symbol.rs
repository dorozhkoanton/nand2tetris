use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    value: String,
}

impl Symbol {
    pub fn new(s: &str) -> Result<Self, ParseSymbolError> {
        if !Self::is_valid(s) {
            return Err(ParseSymbolError(s.to_string()));
        }

        Ok(Self {
            value: s.to_string(),
        })
    }

    fn is_valid(s: &str) -> bool {
        let allowed_special_chars = "_.$:";
        let mut chars = s.chars();

        let first_ok = chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || allowed_special_chars.contains(c));
        let rest_ok = chars.all(|c| c.is_ascii_alphanumeric() || allowed_special_chars.contains(c));

        first_ok && rest_ok
    }
}

impl FromStr for Symbol {
    type Err = ParseSymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct ParseSymbolError(String);

impl Error for ParseSymbolError {}

impl Display for ParseSymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown symbol mnemonic: {}", self.0)
    }
}
