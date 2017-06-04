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
    Double
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

pub fn is_valid_id(id : &String) -> bool {
    id.len() > 0
}

#[derive(Debug)]
pub struct ConstantPool (
    Vec<ConstValue>,
);

#[derive(Debug)]
pub struct Method {
    name : String,
    statik : bool,
    locals : usize,
    code : Vec<OpCode>
}

impl Method {
    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.statik
    }

    #[inline]
    pub fn get_locals(&self) -> usize {
        self.locals
    }

    #[inline]
    pub fn get_code(&self) -> &Vec<OpCode> {
        &self.code
    }
}

#[derive(Default, Debug)]
pub struct MethodBuilder {
    name : String,
    locals : usize,
    statik : bool,
    code : Vec<OpCode>,
}

impl MethodBuilder {
    pub fn set_name(mut self, name : String) -> MethodBuilder {
        self.name = name;
        self
    }

    pub fn set_static(mut self, statik : bool) -> MethodBuilder {
        self.statik = statik;
        self
    }

    pub fn set_locals(mut self, locals : usize) -> MethodBuilder {
        self.locals = locals;
        self
    }

    pub fn append_op(mut self, op : OpCode) -> MethodBuilder {
        self.code.push(op);
        self
    }

    pub fn create_method(self) -> Result<Method, AstError> {
        use self::AstError::*;
    
        if !is_valid_id(&self.name) {
            return Err(IllegalMethodName(self.name))
        }
        Ok(Method {
            name : self.name,
            locals : self.locals,
            statik : self.statik,
            code : self.code,
        })
    }
}

#[derive(Debug)]
pub struct Class {
    name : String,
    constant_pool : ConstantPool,
    methods : HashMap<String, Method>,
}

impl Class {
    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn get_constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    #[inline]
    pub fn get_method(&self, name : &String) -> Option<&Method> {
        self.methods.get(name)
    }
}

#[derive(Debug)]
pub struct ClassBuilder {
    name : String,
    constant_pool : ConstantPool,
    methods : HashMap<String, Method>,
}

impl ClassBuilder {
    pub fn new(name : String) -> ClassBuilder {
        ClassBuilder {
            name,
            constant_pool : ConstantPool(Vec::new()),
            methods : HashMap::new(),
        }
    }

    pub fn new_method(&mut self, method_builder : MethodBuilder) -> Result<(), AstError> {
        let method = method_builder.create_method()?;
        self.methods.insert(method.get_name().clone(), method);
        Ok(())
    }

    pub fn create_class(self) -> Result<Class, AstError> {
        use self::AstError::*;

        if !is_valid_id(&self.name) {
            Err(IllegalClassName(self.name))
        }
        else {
            Ok(Class {
                name : self.name,
                constant_pool : self.constant_pool,
                methods : self.methods,
            })
        }
    }
}
