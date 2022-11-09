use std::alloc::{alloc, dealloc, handle_alloc_error};
use std::alloc::Layout;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct UniquePtr<T> {
    ptr: *mut u8,
    layout: Layout,
    marker: PhantomData<T>,
}

impl<T> UniquePtr<T> {
    pub fn new(val: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr: *mut u8 = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            *(ptr as *mut T) = val;
            let marker = PhantomData::default();
            UniquePtr { ptr, layout, marker }
        }
    }
    

    pub fn set(&mut self, val: T) {
        unsafe {
            *(self.ptr as *mut T) = val;
        }
    }
}

impl<T: Copy> UniquePtr<T> {
    pub fn get(&self) -> T {
        unsafe {
            *(self.ptr as *mut T)
        }
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        println!("dropping value");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

impl<T: Copy + std::fmt::Display> std::fmt::Display for UniquePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = unsafe { *(self.ptr as *mut T) };
        write!(f, "{}", val)
    }
}


