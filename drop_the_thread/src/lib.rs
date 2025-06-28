use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Self::default()
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        (id, Thread::new_thread(id, c, &self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        for (pid, state) in self.states.borrow().iter().enumerate() {
            if pid == id {
                return *state;
            }
        }
        false
    }
    pub fn add_drop(&self, id: usize) {
        if Self::is_dropped(&self, id) {
            panic!("{} is already dropped", id)
        } else {
            self.states.borrow_mut()[id] = true
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Self { pid: p, cmd: c, parent: t }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl <'a>Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.drops.set(self.parent.drops.get()+1);
        self.parent.add_drop(self.pid);
    }
}