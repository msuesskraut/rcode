extern crate rcode;

use rcode::ast::{ClassBuilder, MethodBuilder, OpCode};
use rcode::interp::{StackValue, exec_method};

#[test]
fn create_adder_method() {
    use OpCode::*;

    let mut c = ClassBuilder::new("MyClass".to_string());

    let m: MethodBuilder = Default::default();
    let m = m.set_name("adder".to_string())
        .set_static(true)
        .set_locals(2)
        .append_op(iload_0)
        .append_op(iload_1)
        .append_op(iadd)
        .append_op(ireturn);
    c.new_method(m).unwrap();
    let class = c.create_class().unwrap();

    let method = class.get_method(&"adder".to_string()).unwrap();

    let mut caller_stack = vec![StackValue::Int(1), StackValue::Int(3)];

    let res = exec_method(method, &mut caller_stack);
    assert!(match res {
                Ok(StackValue::Int(4)) => true,
                _ => false,
            });
}
