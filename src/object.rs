use crate::ast::Node;
use crate::environment::Environment;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<Box<Node>>,
        body: Box<Node>,
        environment: Environment,
    },
}

impl Object {
    pub fn is_same(&self, other: &Object) -> bool {
        use std::mem;
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn is_integer(&self) -> bool {
        self.is_same(&INTEGER)
    }

    pub fn is_error(&self) -> bool {
        self.is_same(&Object::Error(String::new()))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        match self {
            Object::Null => self.is_same(other),
            Object::Integer(l) => match other {
                Object::Integer(r) => l == r,
                _ => false,
            },
            Object::Bool(l) => match other {
                Object::Bool(r) => l == r,
                _ => false,
            },
            Object::Error(l) => match other {
                Object::Error(r) => l == r,
                _ => false,
            },
            _ => false,
        }
    }
}

pub const TRUE: Object = Object::Bool(true);
pub const FALSE: Object = Object::Bool(false);
pub const INTEGER: Object = Object::Integer(0);
