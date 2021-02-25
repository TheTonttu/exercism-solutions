use std::borrow::Borrow;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T>
where
    T: Clone,
{
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;

        let mut next_node = self.head.borrow();
        while let Some(node) = next_node {
            len += 1;
            next_node = &node.next;
        }

        len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.data)
    }

    /// Returns a new instance of `SimpleLinkedList<T>` with elements in reverse order.
    ///
    /// # Remarks
    /// The unit tests do not unambiguous indicate should the original linked list be modified or new instance returned because the original list is not asserted in the unit tests.
    /// Assuming for now that new instance is wanted because the method return type is a linked list.
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

impl<T> FromIterator<T> for SimpleLinkedList<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();

        // A new list instance is created so we can just pop the contents without affecting the original linked list.
        let mut new_reversed = self.rev();

        while let Some(value) = new_reversed.pop() {
            vector.push(value);
        }

        vector
    }
}

struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}
