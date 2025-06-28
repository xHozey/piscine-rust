use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);    
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: Rc<usize>,
    pub max: usize
}

impl <'a>Tracker<'a> {
    pub fn new(logger: &'a dyn Logger,max: usize ) -> Self {
        Self { logger, value: Rc::new(0), max }
    }
    pub fn set_value(&self, val: &Rc<usize>)  {
        let perc = (Rc::strong_count(val) * 100) / self.max;
        if perc >= 100 {
            self.logger.error("you are over your quota!");
        } else if perc >= 70 && perc < 100 {
            self.logger.warning(&format!("you have used up over {}% of your quota! Proceeds with precaution", perc));
        }
    }

    pub fn peek(&self, val: &Rc<usize>) {
        let perc = (Rc::strong_count(val) * 100) / self.max;
        self.logger.info(&format!("you are using up to {}% of your quota", perc));
    }
}