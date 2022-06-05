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

#[derive(Debug, Clone)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    pub value: Value
}

impl Message {
    pub fn get_message(&self) -> String {
        match &self.value {
            Value::Address(Some(addr)) => addr.to_string().clone(),
            Value::Bool(b) => b.to_string().clone(),
            Value::Char(c) => c.to_string().clone(),
            Value::F32(f) => f.to_string().clone(),
            Value::F64(f) => f.to_string().clone(),
            Value::I32(i) => i.to_string().clone(),
            Value::I64(i) => i.to_string().clone(),
            Value::String(s) => s.clone(),
            _ => String::from("unknown value")

        }
    }
}

pub static mut message_handler: Option<MessageHandler> = None;

pub struct MessageHandler {
    pub sender: Box<dyn Fn(usize, Message)>,
    pub receiver: Box<dyn Fn(usize) -> Option<Message>>,     
}

impl MessageHandler {
    pub fn send(&mut self, port: usize, message: Message) {
        (*self.sender)(port, message);
    }

    pub fn receive(&mut self, port: usize) -> Option<Message> {
        (*self.receiver)(port)
    }
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
    Out(usize, Message),            
    Halt,                       
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

#[derive(Debug, Clone)]
struct Flags {
    pub zero: bool,         // set by arithmentic operations
    pub neg: bool,          // set by arithmentic operations
    pub pos: bool,          // set by arithmentic operations
    pub equal: bool,        // set by compare
    pub less_than: bool,    // set by compare
    pub great_than: bool    // set by compare
}

impl Flags {
    fn new() -> Flags {
        Flags {
            zero: false,
            neg: false,
            pos: false,
            equal: false,
            less_than: false,
            great_than: false
        }
    }

    fn reset(&mut self) {
        self.zero = false;
        self.neg = false;
        self.pos = false;
        self.equal = false;
        self.less_than = false;
        self.great_than = false;
    }

}

/// The virtual machine
#[derive(Debug, Clone)]
pub struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<MemoryCell>,
    stack: Vec<MemoryCell>,
    registers: Vec<MemoryCell>,
    heap: Vec<Value>,
    flags: Flags,
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
            cur_instruction: None,
            heap: vec![],
            flags: Flags::new()
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

    pub fn get_instruction(&mut self, address: usize) -> MemoryCell {
        self.memory[address].clone()
    }

    pub fn set_instruction(&mut self, mem: MemoryCell, address: usize) {
        self.memory[address] = mem;
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
            Instruction::Sub => self.ex_sub(),
            Instruction::Div => self.ex_div(),
            Instruction::Mul => self.ex_mul(),

            Instruction::Jmp(Value::Address(Some(addr))) => self.ex_jump(addr),

            Instruction::Push(x) => self.ex_push(x),
            Instruction::Pop => self.ex_pop(),
            Instruction::Dump => self.ex_dump(),
            Instruction::Halt => self.ex_halt(),
            Instruction::Out(port, message) => self.ex_out(port, message),
            _ => {}
        };
    }

    fn ex_add(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (Some(MemoryCell::Value(Value::I32(l))), 
                Some(MemoryCell::Value(Value::I32(r)))) =>
                {
                    let res = l + r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I32(res)) )
                },
            (Some(MemoryCell::Value(Value::I64(l))), 
                Some(MemoryCell::Value(Value::I64(r)))) => {
                    let res = l + r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I64(res)) )
                },

            (Some(MemoryCell::Value(Value::F32(l))), 
                Some(MemoryCell::Value(Value::F32(r)))) => {
                    let res = l + r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F32(res)) )
            },
            (Some(MemoryCell::Value(Value::F64(l))), 
                Some(MemoryCell::Value(Value::F64(r)))) => {
                    let res = l + r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F64(res)) )
            },

            (l,r) => {
                self.handle_exception(format!("Add: left and right operands must be of the same type: {:#?} + {:#?}", l, r).as_str())
            }
        }
        self.pc += 1;
    }

    fn ex_sub(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (Some(MemoryCell::Value(Value::I32(l))), 
                Some(MemoryCell::Value(Value::I32(r)))) =>
                {
                    let res = l - r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I32(res)) )
                },
            (Some(MemoryCell::Value(Value::I64(l))), 
                Some(MemoryCell::Value(Value::I64(r)))) => {
                    let res = l - r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I64(res)) )
                },

            (Some(MemoryCell::Value(Value::F32(l))), 
                Some(MemoryCell::Value(Value::F32(r)))) => {
                    let res = l - r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F32(res)) )
            },
            (Some(MemoryCell::Value(Value::F64(l))), 
                Some(MemoryCell::Value(Value::F64(r)))) => {
                    let res = l - r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F64(res)) )
            },

            (l,r) => {
                self.handle_exception(format!("Add: left and right operands must be of the same type: {:#?} + {:#?}", l, r).as_str())
            }
        }
        self.pc += 1;
    }

    fn ex_mul(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (Some(MemoryCell::Value(Value::I32(l))), 
                Some(MemoryCell::Value(Value::I32(r)))) =>
                {
                    let res = l * r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I32(res)) )
                },
            (Some(MemoryCell::Value(Value::I64(l))), 
                Some(MemoryCell::Value(Value::I64(r)))) => {
                    let res = l * r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I64(res)) )
                },

            (Some(MemoryCell::Value(Value::F32(l))), 
                Some(MemoryCell::Value(Value::F32(r)))) => {
                    let res = l * r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F32(res)) )
            },
            (Some(MemoryCell::Value(Value::F64(l))), 
                Some(MemoryCell::Value(Value::F64(r)))) => {
                    let res = l * r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F64(res)) )
            },

            (l,r) => {
                self.handle_exception(format!("Add: left and right operands must be of the same type: {:#?} + {:#?}", l, r).as_str())
            }
        }
        self.pc += 1;
    }

    fn ex_div(&mut self) {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (Some(MemoryCell::Value(Value::I32(l))), 
                Some(MemoryCell::Value(Value::I32(r)))) =>
                {
                    let res = l / r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I32(res)) )
                },
            (Some(MemoryCell::Value(Value::I64(l))), 
                Some(MemoryCell::Value(Value::I64(r)))) => {
                    let res = l / r;
                    self.flags.zero = res == 0;
                    self.flags.pos = res > 0;
                    self.flags.neg = res < 0;
                    self.stack.push(MemoryCell::Value( Value::I64(res)) )
                },

            (Some(MemoryCell::Value(Value::F32(l))), 
                Some(MemoryCell::Value(Value::F32(r)))) => {
                    let res = l / r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F32(res)) )
            },
            (Some(MemoryCell::Value(Value::F64(l))), 
                Some(MemoryCell::Value(Value::F64(r)))) => {
                    let res = l / r;
                    self.flags.zero = res == 0.0;
                    self.flags.pos = res > 0.0;
                    self.flags.neg = res < 0.0;
                    self.stack.push(MemoryCell::Value( Value::F64(res)) )
            },

            (l,r) => {
                self.handle_exception(format!("Add: left and right operands must be of the same type: {:#?} + {:#?}", l, r).as_str())
            }
        }
        self.pc += 1;
    }

    fn ex_jump(&mut self, address: usize) {
        self.pc = address;
    }


    fn ex_push(&mut self, value: Value) {
        self.stack.push(MemoryCell::Value( value ));
        self.pc += 1;
    }

    fn ex_pop(&mut self) {
        self.stack.pop();
        self.pc += 1;
    }


    fn ex_out(&mut self, port: usize, message: Message) {
        unsafe {
            match &mut message_handler {
                Some(handler) => handler.send(port, message),
                None => self.handle_exception("No communication channel available")
            }
        }
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




