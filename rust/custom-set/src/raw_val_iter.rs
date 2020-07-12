use std::mem;

pub struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    pub unsafe fn new(slice: &[T]) -> Self {
        let end = if mem::size_of::<T>() == 0 {
            ((slice.as_ptr() as usize) + slice.len()) as *const _
        } else if slice.len() == 0 {
            slice.as_ptr()
        } else {
            slice.as_ptr().offset(slice.len() as isize)
        };

        Self {
            start: slice.as_ptr(),
            end,
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let result = unsafe { std::ptr::read(self.start) };
            self.start = if mem::size_of::<T>() == 0 {
                (self.start as usize + 1) as *const _
            } else {
                unsafe { self.start.offset(1) }
            };
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let pseudo_size = match elem_size {
            0 => 1,
            _ => elem_size,
        };
        let len = (self.end as usize - self.start as usize) / pseudo_size;
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let elem_size = mem::size_of::<T>();
            self.end = match elem_size {
                0 => (self.end as usize - 1) as *const _,
                _ => unsafe { self.end.offset(-1) },
            };
            Some(unsafe { std::ptr::read(self.end) })
        }
    }
}
