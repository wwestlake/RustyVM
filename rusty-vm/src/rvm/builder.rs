


use std::{
    collections::{
        HashMap
    },
};

use super::vm::RustyVM;


pub struct VMBuilder {
    vm: RustyVM, 
    pc: usize,
    symbol_table: HashMap<String, usize>,
}

impl VMBuilder {
    pub fn new() -> VMBuilder {
        VMBuilder {
            vm: RustyVM::new(),
            pc: 0,
            symbol_table: HashMap::new()
        }
    }
}
