use crate::object::Object;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Object>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{ store: HashMap::new() }
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        let val = self.store.get(key)?;
        return self.clone_object(val)
    }

    pub fn set(&mut self, key: String, val: Object) -> Option<Object> {
        let return_value = self.clone_object(&val);
        self.store.insert(key, val);
        return return_value;
    }

    fn clone_object(&self, val: &Object) -> Option<Object> {
        match val {
            Object::Bool(b) => Some(Object::Bool(*b)),
            Object::Integer(i) => Some(Object::Integer(*i)),
            _ => None,
        }
    }
}