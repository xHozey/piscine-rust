mod messenger;
pub use messenger::*;
pub use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>
}

impl Worker {
    pub fn new(track_value: usize) -> Self {
        Self { track_value: Rc::new(track_value), mapped_messages: RefCell::new(HashMap::new()), all_messages: RefCell::new(vec![]) }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let msg_format: Vec<&str> = msg.split(":").collect();
        self.mapped_messages.borrow_mut().insert(msg_format[0].to_string(), msg_format[1].trim().to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        let msg_format: Vec<&str> = msg.split(":").collect();
        self.mapped_messages.borrow_mut().insert(msg_format[0].to_string(), msg_format[1].trim().to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        let msg_format: Vec<&str> = msg.split(":").collect();
        self.mapped_messages.borrow_mut().insert(msg_format[0].to_string(), msg_format[1].trim().to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}