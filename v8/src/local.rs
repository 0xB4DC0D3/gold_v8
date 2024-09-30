use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[repr(C)]
pub struct Local<T>(NonNull<T>);

impl<T> Local<T> {
    #[inline(always)]
    pub fn empty() -> Self {
        Self(NonNull::dangling())
    }
}

impl<T> Deref for Local<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Drop for Local<T> {
    fn drop(&mut self) {
        unsafe { self.0.drop_in_place() };
    }
}
