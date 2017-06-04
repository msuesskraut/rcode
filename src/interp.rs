use std::cell::RefCell;
use std::rc::Rc;

use ast::*;

#[derive(Debug, Copy, Clone)]
pub enum InterpError {
    InsufficientLocals,
    StackUnderflow,
    WrongType,
    Exception,
}

#[derive(Debug, Clone)]
pub enum StackValue {
    Void,
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

enum OpResult {
    Next,
    Jump(usize),
    Return(StackValue),
    Exception,
    Err(InterpError),
}
use self::OpResult::*;

#[derive(Debug)]
pub struct Frame {
    locals : Vec<StackValue>,
    stack : Vec<StackValue>,
    pc : usize,
}

impl Frame {
    pub fn new(method: &Method, parent_stack : &mut Vec<StackValue>) -> Result<Frame, InterpError> {
        let num_locals = method.get_locals();
        if parent_stack.len() < num_locals {
            Result::Err(InterpError::InsufficientLocals)
        }
        else {
            let split_point = parent_stack.len() - num_locals;
            Ok(Frame {
                locals : parent_stack.split_off(split_point),
                stack : Vec::new(),
                pc : 0,
            })
        }
    }

    fn push(&mut self, v : StackValue) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    fn pop_int(&mut self) -> Result<i32, InterpError> {
        let v = self.pop();
        if let Some(v) = v {
            if let StackValue::Int(v) = v {
                Ok(v)
            }
            else {
                Result::Err(InterpError::WrongType)
            }
        }
        else {
            Result::Err(InterpError::StackUnderflow)
        }
    }

    fn iload(&mut self, addr : usize) -> OpResult {
        if addr < self.locals.len() {
            let v = self.locals[addr].clone();
            self.push(v);
            Next
        }
        else {
            Err(InterpError::InsufficientLocals)
        }
    }

    fn iadd(&mut self) -> OpResult {
        let lhs = self.pop_int();
        if let Ok(lhs) = lhs {
            let rhs = self.pop_int();
            if let Ok(rhs) = rhs {
                self.push(StackValue::Int(lhs.wrapping_add(rhs)));
                Next
            }
            else {
                Err(rhs.unwrap_err())
            }
        }
        else {
            Err(lhs.unwrap_err())
        }
    }

    fn ireturn(&mut self) -> OpResult {
        let ret_val = self.pop_int();
        if let Ok(ret_val) = ret_val {
            Return(StackValue::Int(ret_val))
        }
        else {
            Err(ret_val.unwrap_err())
        }
    }

    fn exec_op(&mut self, op : OpCode) -> OpResult {
        use ast::OpCode ::*;
        match op {
            iload_1 => self.iload(1),
            iload_2 => self.iload(2),
            iadd => self.iadd(),
            ireturn => self.ireturn(),
        }
    }

    pub fn exec(&mut self, method : &Method) -> Result<StackValue, InterpError> {
        {
            let mut pc = 0usize;
            let code = method.get_code();

            loop {
                let op : OpCode = code[pc];
                match self.exec_op(op) {
                    Next => pc += 1,
                    Jump(next_pc) => pc = next_pc,
                    Return(ret_val) => return Result::Ok(ret_val),
                    Exception => return Result::Err(InterpError::Exception),
                    Err(err) => return Result::Err(err),
                }
            }
        }
    }
}