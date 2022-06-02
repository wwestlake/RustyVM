


use std::{
    collections::{
        HashMap
    },
};

use super::vm::{RustyVM, MemoryCell, Instruction, Value};


pub struct VMBuilder {
    vm: RustyVM, 
    pc: usize,
    symbol_table: HashMap<String, usize>,
    built: bool
}

impl VMBuilder {
    pub fn new() -> VMBuilder {
        VMBuilder {
            vm: RustyVM::new(),
            pc: 0,
            symbol_table: HashMap::new(),
            built: false
        }
    }

    pub fn start(&mut self) {
        if ! self.built {
            println!("You must call build before start");
            return
        }
        self.vm.reset();
        self.vm.run();
    }

    pub fn build(&mut self) -> &mut Self {
        // reconciles all labels here

        self.built = true;
        self
    }

    pub fn push(&mut self, val: Value) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Push(val)));
        self.pc += 1;
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Add));
        self.pc += 1;
        self
    }

    pub fn halt(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Halt));
        self.pc += 1;
        self
    }

    pub fn dump(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Dump));
        self.pc += 1;
        self
    }

}
