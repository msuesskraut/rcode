extern crate rcode;

use rcode::ast::{Class, OpCode};

#[test]
fn create_adder_method() {
    use OpCode::*;

    let mut c = Class::new("MyClass".to_string());
    let mut m = c.append_method("adder".to_string());

    m.append_inst(iload_1);
    m.append_inst(iload_2);
    m.append_inst(iadd);
    m.append_inst(ireturn);
}