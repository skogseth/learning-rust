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
        // Initialize the layout with the given type, and try to allocate memory
        // If an error occured, the pointer is null, call std::alloc::handle_alloc_error
        // If all is good, dereference the pointer and give it a value
        // Finally return the struct with the initialized values
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
        // This method cannot be called with uninitialized data
        unsafe {
            *(self.ptr as *mut T) = val;
        }
    }
}

impl<T: Copy> UniquePtr<T> {
    pub fn get(&self) -> T {
        // This method cannot be called with uninitialized data
        unsafe {
            *(self.ptr as *mut T)
        }
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        println!("dropping value");
        // We cannot end up here without having initialized memory
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

impl<T: Copy + std::fmt::Display> std::fmt::Display for UniquePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // This cannot be called with uninitialized data
        let val = unsafe { *(self.ptr as *mut T) };
        write!(f, "{}", val)
    }
}

impl<T> std::ops::Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.ptr as *mut T)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set() {
        let mut ptr = UniquePtr::new(7);
        ptr.set(5);
        assert_eq!(ptr.get(), 5);
    }

    #[test]
    fn deref() {
        let ptr = UniquePtr::new("hello");
        assert_eq!(*ptr, "hello");
    }

    #[test]
    fn formatting() {
        let ptr = UniquePtr::new(3.1415);
        let string = format!("{}", ptr);
        assert_eq!(string.as_str(), "3.1415");
    }
}

