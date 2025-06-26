#[derive(Debug, Clone)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self {
            grade: None
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker {
            role, name, next: self.grade.take()
        }))
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|curr| {
            self.grade = curr.next;
            curr.name
        })
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.clone().map(|c| (c.name, c.role))
    }
}