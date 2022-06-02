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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum MetaData {
    Tag(String),

}

/// Describes what can be stored in a memory location
#[derive(Debug, Clone)]
pub enum MemoryCell {
    Instruction(Instruction),
    Value(Value),
    MetaData(MetaData),
    Empty
}

/// The virtual machine
#[derive(Debug, Clone)]
pub struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<MemoryCell>,
    stack: Vec<MemoryCell>,
    registers: Vec<MemoryCell>,

    // special registers
    cur_instruction: Option<Instruction>

}

impl RustyVM {
    pub fn new() -> Self {

        let mut vm = RustyVM {
            pc: 0,
            memory: vec![],
            stack: vec![],
            registers: vec![],
            running: false,
            cur_instruction: None
        };
        for i in 0..16 {
            vm.registers.push(MemoryCell::Empty)
        }
        vm
    }

    fn handle_exception(&mut self, msg: &str) {
        println!("Exception({}): {}\nCurrent Instruction\n{:#?}", self.pc, msg, self.cur_instruction);
        self.ex_dump();
        self.running = false;
    }

    pub fn push(&mut self, mem: MemoryCell) {
        self.memory.push(mem);
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.running = true;
    }

    fn fetch(&mut self) {
        let cell = &self.memory[self.pc];

        match cell {
            MemoryCell::Instruction(i) => self.cur_instruction = Some(i.clone()),
            _ => self.handle_exception("Invalid Instruction in MemoryCell")
        };
    }

    fn decode(&mut self) {
        match self.cur_instruction.clone() {
            Some(inst) => self.execute(inst),
            None => self.handle_exception("Invalid Instruction in MemoryCell")
        };
    }

    fn execute(&mut self, inst: Instruction) {
        match inst {
            Instruction::Nop => self.pc += 1,
            Instruction::Add => self.ex_add(),
            Instruction::Push(x) => self.ex_push(x),
            Instruction::Pop => self.ex_pop(),
            Instruction::Dump => self.ex_dump(),
            Instruction::Halt => self.ex_halt(),
            _ => {}
        };
    }

    fn ex_add(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        println!("ex_add: {:#?} + {:#?}", left, right);
        match (left, right) {
            (Some(MemoryCell::Value(Value::I32(l))), 
                Some(MemoryCell::Value(Value::I32(r)))) =>
                self.stack.push(MemoryCell::Value( Value::I32(l + r)) ),
            (Some(MemoryCell::Value(Value::I64(l))), 
                Some(MemoryCell::Value(Value::I64(r)))) =>
                self.stack.push(MemoryCell::Value( Value::I64(l + r)) ),

            (Some(MemoryCell::Value(Value::F32(l))), 
                Some(MemoryCell::Value(Value::F32(r)))) =>
                self.stack.push(MemoryCell::Value( Value::F32(l + r)) ),
            (Some(MemoryCell::Value(Value::F64(l))), 
                Some(MemoryCell::Value(Value::F64(r)))) =>
                self.stack.push(MemoryCell::Value( Value::F64(l + r)) ),

            (l,r) => {
                self.handle_exception(format!("Add: left and right operands must be of the same type: {:#?} + {:#?}", l, r).as_str())
            }
        }
        self.pc += 1;
    }

    fn ex_push(&mut self, value: Value) {
        self.stack.push(MemoryCell::Value( value ));
        self.pc += 1;
    }

    fn ex_pop(&mut self) {
        self.stack.pop();
        self.pc += 1;
    }


    fn ex_halt(&mut self) {
        self.running = false;
    }

    fn ex_dump(&mut self) {
        println!("\n----------- Start Processor Dump -------------");
        println!("{:#?}", self);
        println!("\n----------- End Processor Dump -------------");

        self.pc += 1;
    }

    pub fn run(&mut self) {
        while self.running {
            self.fetch();
            self.decode();
        }
    }
}




