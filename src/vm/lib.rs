//! Virtual Machine


use serde_derive::{
    Serialize
};
use std::{
    collections::{
        HashMap
    },
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
    Symbol(Option<Value>),
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

pub enum MetaData {

}

/// Describes what can be stored in a memory location
#[derive(Clone)]
#[derive(Debug)]
pub enum MemoryCell {
    Instruction(Instruction),
    Value(Value),
    MetaData(MetaData)
}

/// The virtual machine
#[derive(Clone)]
#[derive(Debug)]
struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<MemoryCell>,
    stack: Vec<MemoryCell>,
    symbol_table: HashMap<String, MemoryCell>,
    
}

impl RustyVM {
    fn new() -> Self {  
        Self {
            pc: 0,
            memory: vec![],
            stack: vec![],
            symbol_table: HashMap::new(),
            running: false
        }
    }

    pub fn run(&mut self) {
    }
}




