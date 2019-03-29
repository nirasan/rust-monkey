#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
}

pub const TRUE: Object = Object::Bool(true);
pub const FALSE: Object = Object::Bool(false);
