extern crate rcode;

use rcode::ast::{ClassBuilder, MethodBuilder, OpCode};

#[test]
fn create_adder_method() {
    use OpCode::*;

    let mut c = ClassBuilder::new("MyClass".to_string());

    let mut m: MethodBuilder = Default::default();
    let m = m.set_name("adder".to_string())
        .set_static(true)
        .set_locals(2)
        .append_op(iload_1)
        .append_op(iload_2)
        .append_op(iadd)
        .append_op(ireturn);
    c.new_method(m);
    let class = c.create_class().unwrap();

    let method = class.get_method(&"adder".to_string()).unwrap();
}
