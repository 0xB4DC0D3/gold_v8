use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::data::traits::Data;

#[repr(C)]
pub struct Local<T: Data>(NonNull<T>);

impl<T: Data> Local<T> {
    #[inline(always)]
    pub fn empty() -> Self {
        Self(NonNull::dangling())
    }

    #[inline(always)]
    pub fn cast<U: Data>(self) -> Local<U> {
        Local(self.0.cast())
    }
}

impl<T: Data> Deref for Local<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Data> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Data> Drop for Local<T> {
    fn drop(&mut self) {
        unsafe { self.0.drop_in_place() };
    }
}
