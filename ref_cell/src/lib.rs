pub use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub use messenger::*;

pub mod messenger;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(
            "Warning".to_string(),
            msg.to_string().replace("Warning: ", ""),
        );
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string().replace("Info: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string().replace("Error: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}
