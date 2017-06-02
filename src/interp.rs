use std::cell::RefCell;
use std::rc::Rc;

use ast::*;

#[derive(Debug)]
pub enum StackValue {
    Empty,
    Byte(i8),
    Short(i16),
    Char(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    CRef(Option<Rc<RefCell<Object>>>),
    IRef(Option<Rc<RefCell<Object>>>),
    ARef(Option<Rc<RefCell<Array>>>),
}

#[derive(Debug)]
pub struct Array {
    ty : Type,
}

#[derive(Debug)]
pub struct Object {
}

#[derive(Debug)]
pub struct Frame<'a> {
    method : &'a Method<'a>,
    locals : Vec<StackValue>,
    stack : Vec<StackValue>,
}