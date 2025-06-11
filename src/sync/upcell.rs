use core::cell::{RefCell, RefMut};

pub type UPCell<T> = UniProcessorCell<T>;

pub struct UniProcessorCell<T> {
    inner: RefCell<T>
}

unsafe impl<T> Sync for UPCell<T> {}

impl<T> UPCell<T> {
    pub unsafe fn new(item: T) -> Self {
        Self { inner: RefCell::new(item) }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}