#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
    ReturnValue(Box<Object>),
    Error(String),
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

pub const TRUE: Object = Object::Bool(true);
pub const FALSE: Object = Object::Bool(false);
pub const INTEGER: Object = Object::Integer(0);
