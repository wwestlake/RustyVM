
use rusty_vm::rvm:: {
    self, 
    vm::{self,
        Value
    }, 
    builder};



fn main() {
    let mut builder = builder::VMBuilder::new();

    builder
        .push(Value::I32(21))
        .push(Value::I32(21))
        .dump()
        .add()
        .dump()
        .halt()
        .build()
        .start();

}
