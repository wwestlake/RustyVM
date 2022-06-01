//! Virtual Machine

use serde_derive::{
    Serialize
};
use std::{
    collections::{
        HashMap
    }, rc::Rc,
};


/// Values that the system is able to process
#[derive(Clone)]
#[derive(Debug)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bool(bool),
    Symbol(Rc<Value>),
    Address(Option<usize>),
}


/// Instructions that the virtual machne will execute
#[derive(Clone)]
#[derive(Debug)]
pub enum Instruction {
    Nop,                        
    Push(Value),           
    Pop,                        
    Add,                        
    Sub,                        
    Mul,                        
    Div,                        
    Jmp(Value),            
    Halt,                       
    Out(Value),            
    Dump,                       
}

#[derive(Clone)]
#[derive(Debug)]
pub enum MetaData {
    Tag(String),

}

/// Describes what can be stored in a memory location
#[derive(Clone)]
#[derive(Debug)]
pub enum MemoryCell {
    Instruction(Instruction),
    Value(Value),
    MetaData(MetaData),
    Empty
}

/// The virtual machine
#[derive(Clone)]
#[derive(Debug)]
pub struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<MemoryCell>,
    stack: Vec<MemoryCell>,
    registers: Vec<MemoryCell>,
}

impl RustyVM {
    pub fn new() -> Self {

        let mut vm = RustyVM {
            pc: 0,
            memory: vec![],
            stack: vec![],
            registers: vec![],
            running: false,
        };
        for i in 0..16 {
            vm.registers.push(MemoryCell::Empty)
        }
        vm
    }

    pub fn run(&mut self) {
    }
}




