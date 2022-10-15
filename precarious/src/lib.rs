use std::alloc::{alloc, dealloc, handle_alloc_error};
use std::alloc::Layout;
use std::marker::PhantomData;

pub struct UniquePtr<T> {
    ptr: *mut u8,
    layout: Layout,
    phantom: PhantomData<T>,
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
            let phantom = PhantomData::default();
            UniquePtr { ptr, layout, phantom }
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