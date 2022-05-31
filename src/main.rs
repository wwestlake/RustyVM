use rusty_vm::{Value, VmBuilder };



fn main() {
    let mut vm_builder = VmBuilder::new();

    vm_builder
        .label(String::from("Start"))
        .dump()
        .jump(String::from("End"))
        .nop()
        .push(Value::F32(Some(3.14)))
        .push(Value::F32(Some(2.6)))
        .add()
        .push(Value::F32(Some(22.7)))
        .mul()
        .label(String::from("End"))
        .dump()
        .halt()
        .build()
        .run();

}
