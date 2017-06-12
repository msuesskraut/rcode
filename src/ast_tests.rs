use super::ast::*;

fn parse_primitive_type(ty: Type, ty_str : &str) {
    assert_eq!(Ok(ty), Type::parse(ty_str));
}

#[test]
fn parse_integer() {
    parse_primitive_type(Type::Integer, "I");
}

#[test]
fn parse_byte() {
    parse_primitive_type(Type::Byte, "B");
}

#[test]
fn parse_short() {
    parse_primitive_type(Type::Short, "S");
}

#[test]
fn parse_char() {
    parse_primitive_type(Type::Char, "C");
}

#[test]
fn parse_long() {
    parse_primitive_type(Type::Long, "J");
}

#[test]
fn parse_float() {
    parse_primitive_type(Type::Float, "F");
}

#[test]
fn parse_double() {
    parse_primitive_type(Type::Double, "D");
}

#[test]
fn parse_bool() {
    parse_primitive_type(Type::Boolean, "Z");
}

#[test]
fn parse_1dim_array_int() {
    assert_eq!(Ok(Type::Array(1, Box::new(Type::Integer))), Type::parse("[I"));
}

#[test]
fn parse_2dim_array_int() {
    assert_eq!(Ok(Type::Array(2, Box::new(Type::Integer))), Type::parse("[[I"));
}

#[test]
fn parse_3dim_array_int() {
    assert_eq!(Ok(Type::Array(3, Box::new(Type::Integer))), Type::parse("[[[I"));
}

#[test]
fn parse_3dim_array_bool() {
    assert_eq!(Ok(Type::Array(3, Box::new(Type::Boolean))), Type::parse("[[[Z"));
}

#[test]
fn parse_class() {
    assert_eq!(Ok(Type::Class(String::from("java.lang.Object"))), Type::parse("Ljava.lang.Object;"));
}

#[test]
fn parse_2dim_array_class() {
    assert_eq!(Ok(Type::Array(2, Box::new(Type::Class(String::from("java.lang.Object"))))), Type::parse("[[Ljava.lang.Object;"));
}

#[test]
#[allow(non_snake_case)]
fn parse_method_desc_IC_Z() {
    assert_eq!(Ok(FunctionType{ret : Some(Type::Boolean), args: vec![Type::Integer, Type::Char]}), FunctionType::parse("(IC)Z"));
}
