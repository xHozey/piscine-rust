use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);    
}

pub struct Tracker {
    logger: Rc<dyn Logger>,
    value: u32,
    max: u32
}

impl Tracker {
    pub fn new(logger: Rc<dyn Logger>,max: u32 ) -> Self {
        Self { logger, value: 0, max }
    }
    pub fn set_value() -> {
        
    }
}