use crate::object::Object;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<Environment>) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<Object>> {
        let val = self.store.get(key);
        if val.is_some() {
            return Some(val.unwrap().clone());
        }

        if let Some(outer) = &self.outer {
            outer.get(key)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: String, val: Rc<Object>) -> Option<Rc<Object>> {
        self.store.insert(key, val.clone());
        return Some(val);
    }
}
