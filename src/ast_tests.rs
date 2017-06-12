use super::ast::*;

fn parse_primitive_type(ty: Type, ty_str: &str) {
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

fn parse_type_error(type_str: &str) {
    assert_eq!(Err(AstError::IllegalTypeString(type_str.to_string())),
               Type::parse(type_str));
}

#[test]
fn parse_unknown_primitive() {
    parse_type_error("N");
}

#[test]
fn parse_too_long_type_string() {
    parse_type_error("II");
}

#[test]
fn parse_array_without_type() {
    parse_type_error("[[");
}

#[test]
fn parse_incomplete_class() {
    parse_type_error("Ljava.lang.Obj");
}

#[test]
fn parse_incomplete_array_class() {
    parse_type_error("[[Ljava.lang.Object");
}

#[test]
fn parse_1dim_array_int() {
    assert_eq!(Ok(Type::Array(1, Box::new(Type::Integer))),
               Type::parse("[I"));
}

#[test]
fn parse_2dim_array_int() {
    assert_eq!(Ok(Type::Array(2, Box::new(Type::Integer))),
               Type::parse("[[I"));
}

#[test]
fn parse_3dim_array_int() {
    assert_eq!(Ok(Type::Array(3, Box::new(Type::Integer))),
               Type::parse("[[[I"));
}

#[test]
fn parse_3dim_array_bool() {
    assert_eq!(Ok(Type::Array(3, Box::new(Type::Boolean))),
               Type::parse("[[[Z"));
}

#[test]
fn parse_class() {
    assert_eq!(Ok(Type::Class(String::from("java.lang.Object"))),
               Type::parse("Ljava.lang.Object;"));
}

#[test]
fn parse_2dim_array_class() {
    assert_eq!(Ok(Type::Array(2, Box::new(Type::Class(String::from("java.lang.Object"))))),
               Type::parse("[[Ljava.lang.Object;"));
}

#[test]
#[allow(non_snake_case)]
fn parse_method_desc_IC_Z() {
    assert_eq!(Ok(FunctionType {
                      ret: Some(Type::Boolean),
                      args: vec![Type::Integer, Type::Char],
                  }),
               FunctionType::parse("(IC)Z"));
}

#[test]
#[allow(non_snake_case)]
fn parse_method_desc_IC_V() {
    assert_eq!(Ok(FunctionType {
                      ret: None,
                      args: vec![Type::Integer, Type::Char],
                  }),
               FunctionType::parse("(IC)V"));
}

#[test]
#[allow(non_snake_case)]
fn parse_method_desc_array_classes_() {
    assert_eq!(Ok(FunctionType {
                      ret: Some(Type::Array(3, Box::new(Type::Boolean))),
                      args: vec![Type::Array(2,
                                             Box::new(Type::Class(String::from("java.lang.String")))),
                                 Type::Class(String::from("com.pack.MyClass"))],
                  }),
               FunctionType::parse("([[Ljava.lang.String;Lcom.pack.MyClass;)[[[Z"));
}

#[test]
#[allow(non_snake_case)]
fn parse_method_desc_void_void() {
    assert_eq!(Ok(FunctionType {
                      ret: None,
                      args: Vec::new(),
                  }),
               FunctionType::parse("()V"));
}

fn parse_method_desc_error(type_str: &str) {
    assert_eq!(Err(AstError::IllegalTypeString(type_str.to_string())),
               FunctionType::parse(type_str));
}

#[test]
fn parse_missing_open_parenthesis() {
    parse_method_desc_error("II)I");
}

#[test]
fn parse_missing_close_parenthesis() {
    parse_method_desc_error("(III");
}

#[test]
fn parse_close_open_parenthesis() {
    parse_method_desc_error(")II(I");
}

#[test]
fn parse_double_open_parenthesis() {
    parse_method_desc_error("((II)I");
}

#[test]
fn parse_double_close_parenthesis() {
    parse_method_desc_error("(II))I");
}

#[test]
fn parse_with_space() {
    parse_method_desc_error("(I I)I");
}

#[test]
fn parse_missing_return_type() {
    parse_method_desc_error("(II)");
}
