// Copyright 2025, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/licenses/COPYRIGHT.md

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
    error Unauthorized();
}

sol_storage! {
    #[entrypoint]
    pub struct {{ cookiecutter.contract_name }} {
        address owner;
        uint256 number;
    }
}

#[derive(SolidityError)]
pub enum {{ cookiecutter.contract_name }}Errors {
    Unauthorized(Unauthorized),
}

#[public]
impl {{ cookiecutter.contract_name }} {
    #[constructor]
    pub fn constructor(&mut self, initial_number: U256) {
        let owner = self.vm().tx_origin();
        self.owner.set(owner);
        self.number.set(initial_number);
    }

    pub fn set_number(&mut self, number: U256) -> Result<(), {{ cookiecutter.contract_name }}Errors> {
        if self.owner.get() != self.vm().msg_sender() {
            return Err({{ cookiecutter.contract_name }}Errors::Unauthorized(Unauthorized {}));
        }
        self.number.set(number);
        Ok(())
    }

    pub fn number(&self) -> U256 {
        self.number.get()
    }

    pub fn owner(&self) -> Address {
        self.owner.get()
    }
}
