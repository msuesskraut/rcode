use std::cell::RefCell;
use std::rc::Rc;

use ast::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl PartialEq for StackValue {
    fn eq(&self, other: &StackValue) -> bool {
        use self::StackValue::*;

        match *self {
            Void => if let Void = *other { true } else { false },
            Int(lhs) => {
                if let Int(rhs) = *other {
                    lhs == rhs
                } else {
                    false
                }
            }
            Long(lhs) => {
                if let Long(rhs) = *other {
                    lhs == rhs
                } else {
                    false
                }
            }
            Float(lhs) => {
                if let Float(rhs) = *other {
                    lhs == rhs
                } else {
                    false
                }
            }
            Double(lhs) => {
                if let Double(rhs) = *other {
                    lhs == rhs
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Array {
    ty: Type,
}

#[derive(Debug)]
pub struct Object {}

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
    locals: Vec<StackValue>,
    stack: Vec<StackValue>,
}

impl Frame {
    fn new(method: &Method, caller_stack: &mut Vec<StackValue>) -> Result<Frame, InterpError> {
        let num_args = method.ty.args.len();
        if caller_stack.len() < num_args {
            Result::Err(InterpError::InsufficientLocals)
        } else {
            let split_point = caller_stack.len() - num_args;
            let mut locals = caller_stack.split_off(split_point);
            // append default values for local variables
            locals.append(&mut vec![StackValue::Void; method.locals]);
            Ok(Frame {
                   locals,
                   stack: Vec::new(),
               })
        }
    }

    fn push(&mut self, v: StackValue) {
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
            } else {
                Result::Err(InterpError::WrongType)
            }
        } else {
            Result::Err(InterpError::StackUnderflow)
        }
    }

    fn iload(&mut self, addr: usize) -> OpResult {
        if addr < self.locals.len() {
            let v = self.locals[addr].clone();
            self.push(v);
            Next
        } else {
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
            } else {
                Err(rhs.unwrap_err())
            }
        } else {
            Err(lhs.unwrap_err())
        }
    }

    fn ireturn(&mut self) -> OpResult {
        let ret_val = self.pop_int();
        if let Ok(ret_val) = ret_val {
            Return(StackValue::Int(ret_val))
        } else {
            Err(ret_val.unwrap_err())
        }
    }

    fn exec_op(&mut self, op: OpCode) -> OpResult {
        use ast::OpCode::*;
        match op {
            iload_0 => self.iload(0),
            iload_1 => self.iload(1),
            iload_2 => self.iload(2),
            iadd => self.iadd(),
            ireturn => self.ireturn(),
        }
    }

    fn exec(&mut self, method: &Method) -> Result<StackValue, InterpError> {
        let mut pc = 0usize;
        let code = &method.code;

        loop {
            let op: OpCode = code[pc];
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

pub fn exec_method(method: &Method,
                   parent_stack: &mut Vec<StackValue>)
                   -> Result<StackValue, InterpError> {
    let mut frame = Frame::new(method, parent_stack)?;
    frame.exec(method)
}
