use std::iter::FromIterator;

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
            self.length -= 1;
            Some(old_head.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(head) => Some(&head.data),
            None => None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut next_node = self.head;

        while let Some(node) = next_node {
            list.push(node.data);
            next_node = node.next;
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(SimpleLinkedList::new(), |mut list, elem| {
                list.push(elem);
                list
            })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut next_node = self.head;

        while let Some(node) = next_node {
            vec.push(node.data);
            next_node = node.next;
        }
        vec.into_iter().rev().collect()
    }
}
