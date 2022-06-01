use rusty_vm::{Value, VmBuilder };



fn main() {
    let mut vm_builder = VmBuilder::new();

    vm_builder
        .label(String::from("Start"))
        .movi(0, Value::F32(Some(0.6)))
        .movi(2, Value::F32(Some(2.6)))
        .movi(4, Value::F32(Some(4.6)))
        .movi(6, Value::F32(Some(6.6)))
        .movi(8, Value::F32(Some(8.6)))
        .movi(15, Value::F32(Some(15.6)))
        .movr(3,2)
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
