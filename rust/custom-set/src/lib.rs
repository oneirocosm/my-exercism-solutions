use std::marker::PhantomData;
use std::mem;

mod raw_custom_set;
mod raw_val_iter;
use raw_custom_set::RawCustomSet;
use raw_val_iter::RawValIter;

/// My own implementation of a Set (with set theory operations)
///
/// Based off of the vector implementation in the Rustonomicon
/// The set does not allow repeat elements
/// Members of the set must implement Copy and PartialEq
/// The set is ordered in insertion order
#[derive(Debug, Clone)]
pub struct CustomSet<T: Copy + PartialEq> {
    buf: RawCustomSet<T>,
    len: usize,
}

impl<T> CustomSet<T>
where
    T: Copy + PartialEq,
{
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new(input: &[T]) -> Self {
        let mut set = Self {
            buf: RawCustomSet::new(),
            len: 0,
        };

        input.into_iter().for_each(|element| set.add(*element));
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|member| member == element)
    }

    pub fn add(&mut self, element: T) {
        if self.len == self.cap() {
            self.buf.grow().unwrap();
        }

        if !self.contains(&element) {
            unsafe { std::ptr::write(self.ptr().offset(self.len as isize), element) };
            self.len += 1;
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|member| other.contains(member))
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.iter().any(|member| other.contains(member))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.iter()
            .filter(|member| other.contains(member))
            .copied()
            .collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.iter()
            .filter(|member| !other.contains(member))
            .copied()
            .collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        self.iter().chain(other.iter()).copied().collect()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        let iter = unsafe { RawValIter::new(&self) };

        let buf = unsafe { std::ptr::read(&self.buf) };
        mem::forget(self);

        IntoIter { iter, _buf: buf }
    }

    pub fn drain(&mut self) -> Drain<T> {
        let iter = unsafe { RawValIter::new(&self) };
        self.len = 0;

        Drain {
            iter,
            set: PhantomData,
        }
    }

    /// Do not expose this since it's not a normal set operation
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(std::ptr::read(self.ptr().offset(self.len as isize))) }
        }
    }
}

impl<T> PartialEq for CustomSet<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T> Drop for CustomSet<T>
where
    T: Copy + PartialEq,
{
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: Copy> std::ops::Deref for CustomSet<T>
where
    T: Copy + PartialEq,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T: Copy> std::ops::DerefMut for CustomSet<T>
where
    T: Copy + PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { ::std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> std::iter::FromIterator<T> for CustomSet<T>
where
    T: Copy + PartialEq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new(&[]);

        for elem in iter {
            set.add(elem);
        }
        set
    }
}

pub struct IntoIter<T> {
    _buf: RawCustomSet<T>,
    iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    set: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
