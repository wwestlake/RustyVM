
use rusty_vm::rvm:: {
    self, 
    vm::{self,
        Value, 
        message_handler, 
        MessageHandler,
        Message,

    }, 
    builder};



fn main() {
    let mut builder = builder::VMBuilder::new();

    unsafe {
        message_handler = Some(
            MessageHandler {
                sender: Box::new(|port: usize, msg: Message| print!("{}", msg.get_message() )),
                receiver: Box::new(|port: usize| Some(Message { from: 1, to: 1, value: Value::F32(42.0) }))
            }
        );
    }

    builder
        .label("Start")
        .jump("End")
        .push(Value::I32(21))
        .push(Value::I32(21))
        //.dump()
        .add()
        .push(Value::I32(2))
        .mul()
        .push(Value::I32(2))
        .div()
        .push(Value::I32(4))
        .sub()
        .out(0, Message { from: 0, to: 0, value: Value::String(String::from("This is a message ")) })
        .out(0, Message {from: 0, to: 0, value: Value::F64(27.56)})
        .out(0, Message { from: 0, to: 0, value: Value::String(String::from("\n")) })
        .label("End")
        .dump()
        .halt()
        .build()
        .start();

}
