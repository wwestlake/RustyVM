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
    Dump,                       
}


#[derive(Debug)]
struct RustyVM {
    pc: usize, // program counter
    running: bool,
    memory: Vec<Value>,
    stack: Vec<Value>,
    symbol_table: HashMap<String, Value>,
    
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
        while self.running {
            let instruction = &self.memory[self.pc];
            match instruction {
                Value::Instruction(Instruction::Nop) => self.do_nop(),  
                Value::Instruction(Instruction::Push(_)) => self.do_push(),
                Value::Instruction(Instruction::Pop) => self.do_pop(),                        
                Value::Instruction(Instruction::Add) => self.do_add(),                        
                Value::Instruction(Instruction::Sub) => self.do_sub(),                        
                Value::Instruction(Instruction::Mul) => self.do_mul(),                        
                Value::Instruction(Instruction::Div) => self.do_div(),                        
                Value::Instruction(Instruction::Jmp(address)) => self.do_jump(address.clone()),

                Value::Instruction(Instruction::Halt) => self.do_halt(),                       
                Value::Instruction(Instruction::Out(_)) => self.do_out(),
                Value::Instruction(Instruction::Dump) => self.do_dump(),
                _ => { self.exception(String::from("Invalid instruction"))},
            }
        }
    }

    fn exception(&mut self, msg: String) {
        println!("Exception: {}", msg);
        self.do_dump();
        self.do_halt();
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
        println!("{:#?} + {:#?}", left, right);
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        self.stack.push(Value::I32(Some(l + r)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        self.stack.push(Value::I64(Some(l + r)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        self.stack.push(Value::F32(Some(l + r)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        self.stack.push(Value::F64(Some(l + r)));
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
        println!("{:#?} + {:#?}", left, right);
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        self.stack.push(Value::I32(Some(l - r)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        self.stack.push(Value::I64(Some(l - r)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        self.stack.push(Value::F32(Some(l - r)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        self.stack.push(Value::F64(Some(l - r)));
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
        println!("{:#?} + {:#?}", left, right);
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        self.stack.push(Value::I32(Some(l * r)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        self.stack.push(Value::I64(Some(l * r)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        self.stack.push(Value::F32(Some(l * r)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        self.stack.push(Value::F64(Some(l * r)));
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
        println!("{:#?} + {:#?}", left, right);
        match (left, right) {
            (Some(l), Some(r)) => { 
                match (l, r) {
                    (Value::I32(Some(l)), Value::I32(Some(r))) => {  
                        self.stack.push(Value::I32(Some(l / r)));
                    }
                    (Value::I64(Some(l)), Value::I64(Some(r))) => {
                        self.stack.push(Value::I64(Some(l / r)));
                    }
                    (Value::F32(Some(l)), Value::F32(Some(r))) => {
                        self.stack.push(Value::F32(Some(l / r)));
                    }
                    (Value::F64(Some(l)), Value::F64(Some(r))) => {
                        self.stack.push(Value::F64(Some(l / r)));
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
        self.do_dump();
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




