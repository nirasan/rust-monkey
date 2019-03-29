#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Bool(bool),
}