use std::ptr::NonNull;

mod pre_implemented;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

pub struct Cursor<'a, T> {
    ptr: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    ptr: Option<NonNull<Node<T>>>,
    _list: &'a LinkedList<T>,
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(mut current) => {
                let mut count = 1;

                while let Some(new_node) = unsafe{ current.as_ref().next } {
                    count += 1;
                    current = new_node;
                }
                count
            }
        }
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            ptr: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            ptr: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            ptr: self.head,
            _list: self,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.ptr.map(|ptr| {
            unsafe { &mut (*ptr.as_ptr()).elem }
        })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        self.ptr.map(|old_ptr| {
            self.ptr = unsafe { (*old_ptr.as_ptr()).next };
        });
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.ptr.map(|old_ptr| {
            self.ptr = unsafe { (*old_ptr.as_ptr()).prev };
        });
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        self.ptr.map(|ptr| {
            let prev_wrap = unsafe { (*ptr.as_ptr()).prev };
            let next_wrap = unsafe { (*ptr.as_ptr()).next };
            let elem = unsafe { Box::from_raw(ptr.as_ptr()).elem };
            if let Some(mut prev) = prev_wrap {
                let prev = unsafe { prev.as_mut() };
                prev.next = next_wrap;
            } else {
                self.list.head = next_wrap;
            }
            if let Some(mut next) = next_wrap {
                let next = unsafe { next.as_mut() };
                next.prev = prev_wrap;
            } else {
                self.list.tail = prev_wrap;
            }

            self.ptr = next_wrap.or(prev_wrap);
            elem
        })
    }

    pub fn insert_after(&mut self, element: T) {
        if let Some(mut ptr) = self.ptr {
            let next_wrap = unsafe { (*ptr.as_ptr()).next };

            let new_node = Node {
                elem: element,
                next: next_wrap,
                prev: Some(ptr),
            };
            let new_ptr = unsafe {NonNull::new_unchecked(Box::into_raw(Box::new(new_node)))};

            let node = unsafe {ptr.as_mut()};
            node.next = Some(new_ptr);

            if let Some(mut next) = next_wrap {
                let next = unsafe { next.as_mut() };
                next.prev = Some(new_ptr);
            } else {
                self.list.tail = Some(new_ptr);
            }

        } else {
            let new_node = Node {
                elem: element,
                next: None,
                prev: None,
            };

            let new_ptr = unsafe {NonNull::new_unchecked(Box::into_raw(Box::new(new_node)))};
            self.list.head = Some(new_ptr);
            self.list.tail = Some(new_ptr);
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if let Some(mut ptr) = self.ptr {
            let prev_wrap = unsafe { (*ptr.as_ptr()).prev };

            let new_node = Node {
                elem: element,
                next: Some(ptr),
                prev: prev_wrap,
            };
            let new_ptr = unsafe {NonNull::new_unchecked(Box::into_raw(Box::new(new_node)))};

            let node = unsafe {ptr.as_mut()};
            node.prev = Some(new_ptr);

            if let Some(mut prev) = prev_wrap {
                let prev = unsafe { prev.as_mut() };
                prev.next = Some(new_ptr);
            } else {
                self.list.head = Some(new_ptr);
            }

        } else {
            let new_node = Node {
                elem: element,
                next: None,
                prev: None,
            };

            let new_ptr = unsafe {NonNull::new_unchecked(Box::into_raw(Box::new(new_node)))};
            self.list.head = Some(new_ptr);
            self.list.tail = Some(new_ptr);
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.ptr.map(|ptr| {
            let node = unsafe {ptr.as_ref() };

            self.ptr = node.next;
            unsafe { &(*ptr.as_ptr()).elem }
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while let Some(_) = cursor.take() {}
    }
}
