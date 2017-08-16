use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum AstError {
    IllegalMethodName(String),
    IllegalClassName(String),
    IllegalTypeString(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Boolean,
    Byte,
    Short,
    Char,
    Integer,
    Long,
    Float,
    Double,
    Class(String),
    Array(usize, Box<Type>),
}

#[derive(Debug)]
struct DescriptorParser<'a> {
    type_str: &'a str,
    type_iter: Peekable<Chars<'a>>,
}

fn type_error<T>(type_str: &str) -> Result<T, AstError> {
    Err(AstError::IllegalTypeString(String::from(type_str)))
}

impl<'a> DescriptorParser<'a> {
    fn new(type_str: &'a str) -> DescriptorParser<'a> {
        DescriptorParser {
            type_str,
            type_iter: type_str.chars().peekable(),
        }
    }

    #[inline]
    fn eof(&mut self) -> bool {
        self.type_iter.peek().is_none()
    }

    fn parse_field_type(&mut self) -> Result<Option<Type>, AstError> {
        match self.type_iter.next() {
            // end of string
            None => Ok(None),
            // basic types
            Some('B') => Ok(Some(Type::Byte)),
            Some('C') => Ok(Some(Type::Char)),
            Some('D') => Ok(Some(Type::Double)),
            Some('F') => Ok(Some(Type::Float)),
            Some('I') => Ok(Some(Type::Integer)),
            Some('J') => Ok(Some(Type::Long)),
            Some('S') => Ok(Some(Type::Short)),
            Some('Z') => Ok(Some(Type::Boolean)),
            // found array
            Some('[') => self.parse_array(),
            // class name
            Some('L') => self.parse_class(),
            // something else: error
            _ => type_error(self.type_str),
        }
    }

    fn parse_array(&mut self) -> Result<Option<Type>, AstError> {
        let mut dim = 1usize;
        while let Some(&'[') = self.type_iter.peek() {
            dim += 1;
            assert_eq!(Some('['), self.type_iter.next());
        }
        let ty = self.parse_field_type()?;
        if let Some(ty) = ty {
            Ok(Some(Type::Array(dim, Box::new(ty))))
        } else {
            type_error(self.type_str)
        }
    }

    fn parse_class(&mut self) -> Result<Option<Type>, AstError> {
        let mut class_name = String::new();
        while let Some(ch) = self.type_iter.next() {
            if ';' == ch {
                return Ok(Some(Type::Class(class_name)));
            }
            class_name.push(ch);
        }
        type_error(self.type_str)
    }
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        use self::Type::*;

        match *self {
            Boolean | Byte | Short | Char | Integer | Long | Float | Double => true,
            _ => false,
        }
    }

    pub fn parse(type_str: &str) -> Result<Type, AstError> {
        let mut p = DescriptorParser::new(type_str);
        let ty = p.parse_field_type()?;
        if ty.is_none() || !p.eof() {
            type_error(type_str)
        } else {
            Ok(ty.unwrap())
        }
    }
}

fn split_method_descriptor(desc: &str) -> Result<(&str, &str), AstError> {
    let start = desc.find('(');
    let end = desc.find(')');

    if let (Some(start), Some(end)) = (start, end) {
        if end < start {
            type_error(desc)
        } else {
            let args = &desc[(start + 1)..(end - start)];
            let ret = &desc[(end + 1)..];
            Ok((args, ret))
        }
    } else {
        type_error(desc)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FunctionType {
    pub ret: Option<Type>,
    pub args: Vec<Type>,
}

impl FunctionType {
    pub fn parse(type_str: &str) -> Result<FunctionType, AstError> {
        let (args, ret) = split_method_descriptor(type_str)?;
        let mut args_parser = DescriptorParser::new(args);
        let mut args = Vec::new();
        loop {
            match args_parser.parse_field_type() {
                Ok(Some(arg_ty)) => args.push(arg_ty),
                Ok(None) => break,
                Err(_) => return type_error(type_str),
            }
        }
        if ret.is_empty() {
            return type_error(type_str);
        }
        let ret = if ret == "V" {
            None
        } else {
            let ret = Type::parse(ret);
            if let Ok(ret) = ret {
                Some(ret)
            } else {
                return type_error(type_str);
            }
        };
        Ok(FunctionType { args, ret })
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    iload_0,
    iload_1,
    iload_2,
    iadd,
    ireturn,
}

#[derive(Debug)]
pub enum ConstValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
}

pub fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
}

#[derive(Debug, Default)]
pub struct ConstantPool(Vec<ConstValue>);

#[derive(Debug, Default)]
pub struct Method {
    pub name: String,
    pub ty: FunctionType,
    pub statik: bool,
    pub locals: usize,
    pub code: Vec<OpCode>,
}

#[derive(Debug, Default)]
pub struct MethodBuilder {
    method: Method,
    error: Option<AstError>,
}

impl MethodBuilder {
    pub fn set_name(mut self, name: String) -> MethodBuilder {
        if self.error.is_none() {
            self.method.name = name;
        }
        self
    }

    pub fn set_type(mut self, type_str: &str) -> MethodBuilder {
        if self.error.is_none() {
            let ty = FunctionType::parse(type_str);
            if let Ok(ty) = ty {
                self.method.ty = ty;
            } else {
                self.error = ty.err();
            }
        }
        self
    }

    pub fn set_static(mut self, statik: bool) -> MethodBuilder {
        if self.error.is_none() {
            self.method.statik = statik;
        }
        self
    }

    pub fn set_locals(mut self, locals: usize) -> MethodBuilder {
        if self.error.is_none() {
            self.method.locals = locals;
        }
        self
    }

    pub fn append_op(mut self, op: OpCode) -> MethodBuilder {
        if self.error.is_none() {
            self.method.code.push(op);
        }
        self
    }

    pub fn create_method(self) -> Result<Method, AstError> {
        use self::AstError::*;

        if let Some(err) = self.error {
            return Err(err);
        }
        if !is_valid_id(&self.method.name) {
            return Err(IllegalMethodName(self.method.name));
        }
        Ok(self.method)
    }
}

#[derive(Debug, Default)]
pub struct Class {
    pub name: String,
    constant_pool: ConstantPool,
    methods: HashMap<String, Method>,
}

impl Class {
    #[inline]
    pub fn get_method(&self, name: &str) -> Option<&Method> {
        self.methods.get(name)
    }
}

#[derive(Debug, Default)]
pub struct ClassBuilder {
    class: Class,
}

impl ClassBuilder {
    pub fn new(name: String) -> ClassBuilder {
        ClassBuilder {
            class: Class {
                name,
                ..Default::default()
            },
        }
    }

    pub fn new_method(&mut self, method_builder: MethodBuilder) -> Result<(), AstError> {
        let method = method_builder.create_method()?;
        self.class.methods.insert(method.name.clone(), method);
        Ok(())
    }

    pub fn create_class(self) -> Result<Class, AstError> {
        use self::AstError::*;

        if !is_valid_id(&self.class.name) {
            Err(IllegalClassName(self.class.name))
        } else {
            Ok(self.class)
        }
    }
}
