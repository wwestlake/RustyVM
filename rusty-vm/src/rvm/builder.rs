


use std::{
    collections::{
        HashMap
    },
};

use super::vm::{RustyVM, MemoryCell, Instruction, Value, Message};


pub struct VMBuilder {
    vm: RustyVM, 
    pc: usize,
    symbol_table: HashMap<String, Value>,
    unresolved_label_refs: HashMap<String, usize>,    
    built: bool
}

impl VMBuilder {
    pub fn new() -> VMBuilder {
        VMBuilder {
            vm: RustyVM::new(),
            pc: 0,
            symbol_table: HashMap::new(),
            unresolved_label_refs: HashMap::new(),
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
        for (label, address) in &self.unresolved_label_refs {
            if let Some(Value::Address(Some(actual_address))) = self.symbol_table.get(label) {
                match self.vm.get_instruction(*address) {
                    MemoryCell::Instruction(Instruction::Jmp(_)) => {
                        self.vm.set_instruction(MemoryCell::Instruction(Instruction::Jmp(Value::Address(Some(*actual_address))) ), *address)
                    },

                    _ => println!("Invalid instruction at {}", &address)

                }
            } else {
                println!("Invalid instruction at {}", &address);
            }
        }

        self.built = true;
        self
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.symbol_table.insert(String::from(label), Value::Address(Some(self.pc)));
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

    pub fn sub(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Sub));
        self.pc += 1;
        self
    }

    pub fn mul(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Mul));
        self.pc += 1;
        self
    }

    pub fn div(&mut self) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Div));
        self.pc += 1;
        self
    }

    pub fn jump(&mut self, label: &str) -> &mut Self {
        match self.symbol_table.get(label) {
            Some(Value::Address(Some(v))) => {
                self.pc += 1;
                self.vm.push(MemoryCell::Instruction(Instruction::Jmp(Value::Address(Some(*v)))));
            },
            _ => { 
                self.vm.push(MemoryCell::Instruction(Instruction::Jmp(Value::Address(None))));
                self.unresolved_label_refs.insert(label.to_string(), self.pc);
                self.pc += 1;
             },
        }
        self
    }

    pub fn out(&mut self, port: usize, message: Message) -> &mut Self {
        self.vm.push(MemoryCell::Instruction(Instruction::Out(port, message)));
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

    pub fn results(&self) -> Vec<MemoryCell> {
        self.vm.get_stack()
    }

}


#[cfg(test)]
mod tests {
    use crate::rvm::{builder, vm::*};

    #[test]
    fn builder_creates_vm() {
        let mut builder = builder::VMBuilder::new();
        builder
            .push(Value::I32(10))
            .halt()
            .build()
            .start();
        
        let stack = builder.results();
        for mem in stack {
            match mem {
                MemoryCell::Value(Value::I32(n)) => {
                    assert_eq!(10, n);
                },
                _ => panic!("Stack does not contain pushed value")
            }            
        }
    }

}
