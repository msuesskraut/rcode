use std::collections::HashMap;

#[derive(Debug)]
pub enum AstError {
    IllegalMethodName(String),
    IllegalClassName(String),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Short,
    Char,
    Integer,
    Long,
    Float,
    Double,
}

#[derive(Debug)]
pub enum ReferenceType {
    Class,
    Interface,
    Array,
}

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    Reference(ReferenceType),
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        match *self {
            Type::Primitive(_) => true,
            _ => false,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
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

pub fn is_valid_id(id: &String) -> bool {
    id.len() > 0
}

#[derive(Debug, Default)]
pub struct ConstantPool(Vec<ConstValue>);

#[derive(Debug, Default)]
pub struct FunctionType {
    ret: Option<Type>,
    args: Vec<Type>,
}

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
}

impl MethodBuilder {
    pub fn set_name(mut self, name: String) -> MethodBuilder {
        self.method.name = name;
        self
    }

    pub fn set_static(mut self, statik: bool) -> MethodBuilder {
        self.method.statik = statik;
        self
    }

    pub fn set_locals(mut self, locals: usize) -> MethodBuilder {
        self.method.locals = locals;
        self
    }

    pub fn append_op(mut self, op: OpCode) -> MethodBuilder {
        self.method.code.push(op);
        self
    }

    pub fn create_method(self) -> Result<Method, AstError> {
        use self::AstError::*;

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
    pub fn get_method(&self, name: &String) -> Option<&Method> {
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
