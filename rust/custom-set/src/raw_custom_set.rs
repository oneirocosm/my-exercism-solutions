use std::alloc;
use std::mem;
use std::ptr::NonNull;

#[derive(Debug, PartialEq, Clone)]
pub struct RawCustomSet<T> {
    pub ptr: NonNull<T>,
    pub cap: usize,
}

impl<T: Copy> RawCustomSet<T> {
    pub fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        Self {
            ptr: NonNull::dangling(),
            cap: cap,
        }
    }

    pub fn grow(&mut self) -> Result<(), alloc::LayoutErr> {
        let align = mem::align_of::<T>();
        let elem_size = mem::size_of::<T>();
        assert!(elem_size != 0, "capacity overflow");

        let (new_cap, ptr) = match self.cap {
            0 => {
                let new_cap = 1;
                let layout = alloc::Layout::from_size_align(elem_size * new_cap, align)?;
                let ptr = unsafe { alloc::alloc(layout) };
                (new_cap, ptr)
            },
            old_cap => {
                let new_cap = old_cap * 2;
                let old_layout = alloc::Layout::from_size_align(elem_size * old_cap, align)?;
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                let ptr = unsafe { alloc::realloc(old_ptr, old_layout, elem_size * new_cap) };
                (new_cap, ptr)
            },
        };

        self.ptr = NonNull::new(ptr as *mut _).unwrap();
        self.cap = new_cap;

        Ok(())
    }
}

impl<T> Drop for RawCustomSet<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            let align = mem::align_of::<T>();
            let num_bytes = self.cap * elem_size;
            let old_layout = alloc::Layout::from_size_align(num_bytes, align).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
                alloc::dealloc(old_ptr, old_layout);
            }
        }
    }
}
