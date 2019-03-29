use crate::object::Object;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        let val = self.store.get(key);
        if val.is_some() {
            return self.clone_object(val.unwrap());
        }

        if self.outer.is_some() {
            self.outer.clone().unwrap().get(key)
        } else {
            None
        }
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
            Object::Function {
                parameters: p,
                body: b,
                environment: e,
            } => Some(Object::Function {
                parameters: p.clone(),
                body: b.clone(),
                environment: e.clone(),
            }),
            _ => None,
        }
    }
}
