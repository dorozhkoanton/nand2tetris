use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::address::{Address, AddressRangeError};
use crate::symbol::Symbol;

pub struct SymbolTable {
    bindings: HashMap<Symbol, Address>,
    next_variable_address: Address,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            bindings: Self::get_predefined_bindings(),
            next_variable_address: Address::new(16)
                .expect("predefined variable address always valid"),
        }
    }

    fn get_predefined_bindings() -> HashMap<Symbol, Address> {
        [
            ("R0", 0),
            ("R1", 1),
            ("R2", 2),
            ("R3", 3),
            ("R4", 4),
            ("R5", 5),
            ("R6", 6),
            ("R7", 7),
            ("R8", 8),
            ("R9", 9),
            ("R10", 10),
            ("R11", 11),
            ("R12", 12),
            ("R13", 13),
            ("R14", 14),
            ("R15", 15),
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("SCREEN", 16384),
            ("KBD", 24576),
        ]
        .map(|(s, n)| {
            (
                Symbol::new(s).expect("predefined symbol always valid"),
                Address::new(n).expect("predefined address always valid"),
            )
        })
        .into_iter()
        .collect::<HashMap<_, _>>()
    }

    fn allocate_address(&mut self) -> Result<Address, AddressRangeError> {
        let address = self.next_variable_address;
        self.next_variable_address = self.next_variable_address.next()?;

        Ok(address)
    }

    pub fn get_or_insert_variable(
        &mut self,
        variable: Symbol,
    ) -> Result<Address, AddressRangeError> {
        if let Some(&address) = self.bindings.get(&variable) {
            return Ok(address);
        }

        let address = self.allocate_address()?;
        self.bindings.insert(variable, address);

        Ok(address)
    }

    pub fn bind_label(
        &mut self,
        label: Symbol,
        address: Address,
    ) -> Result<(), DuplicateLabelError> {
        if let Some(&old_address) = self.bindings.get(&label) {
            return Err(DuplicateLabelError {
                label,
                existing_address: old_address,
                attempted_address: address,
            });
        }

        self.bindings.insert(label, address);

        Ok(())
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct DuplicateLabelError {
    label: Symbol,
    existing_address: Address,
    attempted_address: Address,
}

impl Error for DuplicateLabelError {}

impl Display for DuplicateLabelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "label {} already defined at {} (attempted redefinition at {})",
            self.label, self.existing_address, self.attempted_address,
        )
    }
}
