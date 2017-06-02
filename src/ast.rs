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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct ConstantPool (
    Vec<ConstValue>,
);

#[derive(Debug)]
pub struct Method<'a> {
    name : String,
    statik : bool,
    locals : usize,
    constant_pool : &'a ConstantPool,
    code : Vec<OpCode>
}

impl<'a> Method<'a> {
    fn new(name : String, constant_pool : &'a ConstantPool) -> Method<'a> {
        Method {
            name,
            statik : true,
            locals : 0,
            constant_pool,
            code : Vec::new(),
        }
    }

    pub fn append_inst(&mut self, op: OpCode) {
        self.code.push(op);
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    name : String,
    constant_pool : ConstantPool,
    methods : Vec<Method<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(name : String) -> Class<'a> {
        Class {
            name,
            constant_pool : ConstantPool(Vec::new()),
            methods : Vec::new(),
        }
    }

    pub fn append_method(&'a mut self, name : String) -> &mut Method<'a> {
        self.methods.push(Method::new(name, &self.constant_pool));
        self.methods.last_mut().unwrap()
    }
}
