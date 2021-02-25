use std::borrow::Borrow;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> where T: Clone {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> where T: Clone {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        let mut next_node = self.head.borrow();
        loop {
            match next_node {
                Some(h) => {
                    len += 1;
                    next_node = &h.next;
                }
                None => return len,
            }
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(h) => {
                self.head = h.next;
                Some(h.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = SimpleLinkedList::new();
        let mut next_node = self.head.borrow();
        while let Some(node) = next_node {
            reversed_list.push(node.data.clone());
            next_node = &node.next
        }
        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> where T: Clone {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> where T: Clone {
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();

        let mut next_node = self.head.borrow();
        loop {
            match next_node {
                Some(h) => {
                    vector.push(h.data.clone());
                    next_node = &h.next;
                }
                None => break,
            }
        }
        // Meh...
        vector.reverse();
        vector
    }
}

struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}
