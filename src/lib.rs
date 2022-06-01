use serde_derive::{
    Serialize
};
use std::{
    collections::{
        HashMap
    },
};




#[derive(Clone)]
#[derive(Debug)]
pub enum Value {
    I32(Option<i32>),
    I64(Option<i64>),
    F32(Option<f32>),
    F64(Option<f64>),
    Char(Option<char>),
    String(Option<String>),
    Bool(Option<bool>),
    Symbol(Option<Box<Value>>),
    Instruction(Instruction),
    Address(Option<usize>),
}



#[derive(Clone)]
#[derive(Debug)]
pub enum Instruction {
    Nop,                        
    Push(Box<Value>),           
    Pop,                        
    Add,                        
    Sub,                        
    Mul,                        
    Div,                        
    Jmp(Box<Value>),            
    Halt,                       
    Out(Box<Value>),
    Movi(usize, Box<Value>),    // move imediate to register 
                                // (usize reg 0-15 inclusive)            
    Movr(usize, usize),         // move from reg a to reg b
    Dump,                       
}

#[derive(Clone)]
#[derive(Debug)]
struct Flags {
    zero: bool,
    pos: bool,
    neg: bool
}

impl Flags {
    fn new() -> Flags {
        Flags {
            zero: false,
            pos: false,
            neg: false
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum Register {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bool(bool),
    Empty,
}

#[derive(Clone)]
#[derive(Debug)]
struct GeneralRegisters {
    registers: Vec<Register>,
}

impl GeneralRegisters {
    fn new() -> GeneralRegisters {
        let mut result = GeneralRegisters { registers: vec![] };
        for i in 0..16 {
            result.registers.push(Register::Empty);
        }
        result
    }

    fn set(&mut self, reg: usize, value: Value) {
        if reg > 15 {
            panic!("Invalid register: {}", reg);
        }

        let val = match value {
            Value::I32(v) => match v {
                Some(x) => Register::I32(x),
                None => Register::Empty
            },
            Value::I64(v) => match v {
                Some(x) => Register::I64(x),
                None => Register::Empty
            },
            Value::F32(v) => match v {
                Some(x) => Register::F32(x),
                None => Register::Empty
            },
            Value::F64(v) => match v {
                Some(x) => Register::F64(x),
                None => Register::Empty
            },
            Value::Char(v) => match v {
                Some(x) => Register::Char(x),
                None => Register::Empty
            },
            Value::String(v) => match v {
                Some(x) => Register::String(x),
                None => Register::Empty
            },
            Value::Bool(v) => match v {
                Some(x) => Register::Bool(x),
                None => Register::Empty
            },
            
            _ => Register::Empty // may gen an error?
        };

        self.registers[reg] = val;
    }

    fn get(&mut self, reg: usize) -> Option<Value> {
        if reg > 15 {
            panic!("Invalid register: {}", reg);
        }

        let val = self.registers[reg].clone();
        let result = match val {
            Register::I32(v) => Some(Value::I32(Some(v))),
            Register::I64(v) => Some(Value::I64(Some(v))),
            Register::F32(v) => Some(Value::F32(Some(v))),
            Register::F64(v) => Some(Value::F64(Some(v))),
            Register::Char(ch) => Some(Value::Char(Some(ch))),
            Register::String(s) => Some(Value::String(Some(s))),
            Register::Bool(b) => Some(Value::Bool(Some(b))),
            Register::Empty => None,
        };

        result
    }     

}

#[derive(Debug)]
struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<Value>,
    stack: Vec<Value>,
    symbol_table: HashMap<String, Value>,
    heap: Vec<Value>,
    flags: Flags,
    registers: GeneralRegisters,
}

impl RustyVM {
    fn new() -> Self {  
        Self {
            pc: 0,
            memory: vec![],
            stack: vec![],
            symbol_table: HashMap::new(),
            running: false,
            heap: vec![],
            flags: Flags::new(),
            registers: GeneralRegisters::new(),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            let instruction = &self.memory[self.pc];
            match instruction {
                // stack instructions
                Value::Instruction(Instruction::Nop) => self.do_nop(),  
                Value::Instruction(Instruction::Push(_)) => self.do_push(),
                Value::Instruction(Instruction::Pop) => self.do_pop(),                        
                Value::Instruction(Instruction::Add) => self.do_add(),                        
                Value::Instruction(Instruction::Sub) => self.do_sub(),                        
                Value::Instruction(Instruction::Mul) => self.do_mul(),                        
                Value::Instruction(Instruction::Div) => self.do_div(),                        
                Value::Instruction(Instruction::Jmp(address)) => self.do_jump(address.clone()),

                // register instructions
                Value::Instruction(Instruction::Movi(reg, v)) => self.do_movi(reg.clone(), *v.clone()),
                Value::Instruction(Instruction::Movr(r1, r2)) => self.do_movr(r1.clone(), r2.clone()),
                // system instructions
                Value::Instruction(Instruction::Halt) => self.do_halt(),                       
                Value::Instruction(Instruction::Out(_)) => self.do_out(),
                Value::Instruction(Instruction::Dump) => self.do_dump(),
                _ => self.exception(String::from(format!("Invalid instruction: {:#?} at {}", instruction, self.pc))),
            }
        }
    }

    fn exception(&mut self, msg: String) {
        self.do_dump();
        self.do_halt();
        println!("Exception: {}", msg);
    } 

    fn do_nop(&mut self) {
        self.pc += 1;
    }

    fn do_push(&mut self) {
        match &self.memory[self.pc] {
            Value::Instruction(Instruction::Push(v)) => self.stack.push(v.as_ref().clone()),
            _ => {self.exception(String::from("Invalid push value"))}
        };
        self.pc += 1;
    }

    fn do_pop(&mut self) {
        self.pc += 1;
        self.stack.pop();
    }

    fn do_add(&mut self) {
        self.pc += 1;
        let right = self.stack.pop().clone();
        let left = self.stack.pop().clone();
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {
                        let val = l + r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I32(Some(val)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        let val = l + r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I64(Some(val)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        let val = l + r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F32(Some(val)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        let val = l + r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F64(Some(val)));
                    }
                    (_,_) => { self.exception(String::from("Add: Cannot perform add on types supplied"))},
                
                }
            },
            (_, _) => self.exception(String::from("Add: Invalid Value")),
        }
    }

    fn do_sub(&mut self) {
        self.pc += 1;
        let right = self.stack.pop().clone();
        let left = self.stack.pop().clone();
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        let val = l - r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I32(Some(val)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        let val = l - r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I64(Some(val)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        let val = l - r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F32(Some(val)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        let val = l - r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F64(Some(val)));
                    }
                    (_,_) => { self.exception(String::from("Sub: Cannot perform Sub on types supplied"))},
                
                }
            },
            (_, _) => self.exception(String::from("Sub: Invalid Value")),
        }
    }

    fn do_mul(&mut self) {
        self.pc += 1;
        let right = self.stack.pop().clone();
        let left = self.stack.pop().clone();
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        let val = l * r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I32(Some(val)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        let val = l * r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I64(Some(val)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        let val = l * r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F32(Some(val)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        let val = l * r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F64(Some(val)));
                    }
                    (_,_) => { self.exception(String::from("Mul: Cannot perform Mul on types supplied"))},
                
                }
            },
            (_, _) => self.exception(String::from("Mul: Invalid Value")),
        }
    }

    fn do_div(&mut self) {
        self.pc += 1;
        let right = self.stack.pop().clone();
        let left = self.stack.pop().clone();
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        let val = l / r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I32(Some(val)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        let val = l / r;
                        self.flags.zero = val == 0;
                        self.flags.pos = val > 0;
                        self.flags.neg = val < 0;
                        self.stack.push(Value::I64(Some(val)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        let val = l / r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.0;
                        self.stack.push(Value::F32(Some(val)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        let val = l / r;
                        self.flags.zero = val == 0.0;
                        self.flags.pos = val > 0.0;
                        self.flags.neg = val < 0.;
                        self.stack.push(Value::F64(Some(val)));
                    }
                    (_,_) => { self.exception(String::from("Div: Cannot perform div on types supplied"))},
                
                }
            },
            (_, _) => self.exception(String::from("Div: Invalid Value")),
        }
    }

    fn do_jump(&mut self, address: Box<Value>)  {
        match *address {
            Value::Address(Some(addr)) => { self.pc = addr },
            _ => {}
        }
    }

    fn do_movi(&mut self, reg: usize, val: Value) {
        println!("{:#?}, {:#?}", reg, val);
        self.registers.set(reg, val);
        self.pc += 1;
    }

    fn do_movr(&mut self, reg1: usize, reg2: usize) {
        match self.registers.get(reg2) {
            Some(v) => self.registers.set(reg1, v),
            None => self.exception(format!("cannot move empty register: {} is empty at this point", reg2)),
        };
        self.pc += 1;
    }

    fn do_out(&mut self) {

    } 

    fn do_dump(&mut self) {
        self.pc += 1;
        println!("---------- Dump ---------");
        println!("{:#?}", self);
        println!("---------- End Dump ---------");
    }

    fn do_halt(&mut self) {
        self.running = false;
    }


}

pub struct VmBuilder {
    vm: RustyVM,
    pc: usize,
    unresolved_label_refs: HashMap<String, usize>,
    is_built: bool,
}

impl VmBuilder {
    pub fn new() -> Self {
        Self {
            vm: RustyVM::new(),
            pc: 0,
            unresolved_label_refs: HashMap::new(),
            is_built: false
        }
    }

    

    pub fn nop(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Nop));
        self
    }

    pub fn push(&mut self, val: Value) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Push(Box::new(val))));
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Pop));
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Add));
        self
    }

    pub fn sub(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Sub));
        self
    }

    pub fn mul(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Mul));
        self
    }

    pub fn div(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Div));
        self
    }

    pub fn halt(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Halt));
        self
    }

    pub fn jump(&mut self, label: String) -> &mut Self {
        match self.vm.symbol_table.get(&label) {
            Some(Value::Address(Some(v))) => {
                self.pc += 1;
                self.vm.memory.push(Value::Instruction(Instruction::Jmp(Box::new(Value::Address(Some(*v))))));
            },
            _ => { 
                self.vm.memory.push(Value::Instruction(Instruction::Jmp(Box::new(Value::Address(None)))));
                self.unresolved_label_refs.insert(label, self.pc);
                self.pc += 1;
             },
        }
        self
    }

    pub fn movi(&mut self, reg: usize, val: Value) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Movi(reg, Box::new(val))));
        self
    }

    pub fn movr(&mut self, reg1: usize, reg2: usize) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Movr(reg1, reg2)));
        self
    }

    pub fn out(&mut self, val: Value) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Out(Box::new(val))));
        self
    }

    pub fn dump(&mut self) -> &mut Self {
        self.pc += 1;
        self.vm.memory.push(Value::Instruction(Instruction::Dump));
        self
    }

    pub fn label(&mut self, label: String) -> &mut Self {
        self.vm.symbol_table.insert(label, Value::Address( Some(self.pc) ));
        self
    }

    pub fn build(&mut self) -> &mut Self {
        self.is_built = true;

        for (label, address) in &self.unresolved_label_refs {
            if let Some(Value::Address(Some(actual_address))) = self.vm.symbol_table.get(label) {
                match self.vm.memory[*address] {
                    Value::Instruction(Instruction::Jmp(_)) => {
                        self.vm.memory[*address] = Value::Instruction(Instruction::Jmp(Box::new(Value::Address(Some(*actual_address)))));
                    },
                    _ => println!("Invalid instruction at {}", address)
                }
            } else {
                println!("Invalid instruction at {}", address);
            }
        }
        self
    }

    pub fn run(&mut self) {
        if ! self.is_built {
            println!("You must call Build before calling run!");
            return;
        }
        self.vm.running = true;
        self.vm.run();
    }


}




