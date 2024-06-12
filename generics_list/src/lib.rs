// Définition des structures et implémentations

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
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed_node| *boxed_node);
        }
    }
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref().map(|n| n);
        while let Some(node) = current_node {
            count += 1;
            current_node = node.next.as_ref().map(|boxed_node| &**boxed_node);
        }
        count
    }
}
