#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
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
        self.head = Some(Box::new(Node {value, next: self.head.take()}))
    }

    pub fn pop(&mut self) {
        self.head.take().map(|node| self.head = node.next);
    }

    pub fn len(&self) -> usize {
        let mut count = 1;
        let mut current = &self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                break;
            }
            count += 1;
            current = &node.next;
        }
        count
    }

}