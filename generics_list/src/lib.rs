#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node { value, next: None })
        } else {
            self.head = Some(Node {
                value,
                next: Some(Box::new(self.head.take().unwrap())),
            })
        }
    }

    pub fn pop(&mut self) {
        match &self.head {
            Some(node) => {
                if node.next.is_some() {
                    self.head = Some(*self.head.take().unwrap().next.unwrap())
                } else {
                    self.head = None
                }
            },
            None => {}
        }
       
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_ref();
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = if let Some(next) = node.next.as_ref() {
                Some(&*next)
            } else {
                None
            }
        }
        count
    }
}
