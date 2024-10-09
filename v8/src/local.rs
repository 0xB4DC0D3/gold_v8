use std::{
    mem::{transmute, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::data::{self, traits::Data};

extern "C" {
    fn v8cxx__local_empty(local_buf: *mut Local<data::Data>);
    fn v8cxx__local_is_empty(this: *const Local<data::Data>) -> bool;
}

#[repr(C)]
pub struct Local<T: Data>(NonNull<T>);

impl<T: Data> Local<T> {
    pub fn get_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    #[inline(always)]
    pub fn empty() -> Self {
        let mut local = Self(NonNull::dangling());

        unsafe { v8cxx__local_empty(local.cast_mut()) };

        local
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        unsafe { v8cxx__local_is_empty(self.cast_ref()) }
    }

    #[inline(always)]
    pub fn cast<U: Data>(self) -> Local<U> {
        Local(self.0.cast())
    }

    #[inline(always)]
    pub fn cast_ref<U: Data>(&self) -> &Local<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn cast_mut<U: Data>(&mut self) -> &mut Local<U> {
        unsafe { transmute(self) }
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

extern "C" {
    fn v8cxx__maybe_local_empty(maybe_local_buf: *mut MaybeLocal<data::Data>);
    fn v8cxx__maybe_local_is_empty(this: *const MaybeLocal<data::Data>) -> bool;
    fn v8cxx__maybe_local_to_local(
        this: *const MaybeLocal<data::Data>,
        out: *mut Local<data::Data>,
    ) -> bool;
    fn v8cxx__maybe_local_to_local_checked(
        local_buf: *mut Local<data::Data>,
        this: *mut MaybeLocal<data::Data>,
    );
    fn v8cxx__maybe_local_from_maybe(
        local_buf: *mut Local<data::Data>,
        this: *const MaybeLocal<data::Data>,
        default_value: *const Local<data::Data>,
    );
}

#[repr(C)]
pub struct MaybeLocal<T: Data>(Local<T>);

impl<T: Data> MaybeLocal<T> {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe {
            let mut maybe_local = MaybeUninit::uninit();

            v8cxx__maybe_local_empty(maybe_local.as_mut_ptr());

            maybe_local.assume_init().cast()
        }
    }

    #[inline(always)]
    pub fn from_local(local: Local<T>) -> Self {
        Self(local)
    }

    #[inline(always)]
    pub fn cast<U: Data>(self) -> MaybeLocal<U> {
        MaybeLocal(self.0.cast())
    }

    #[inline(always)]
    pub fn cast_ref<U: Data>(&self) -> &MaybeLocal<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn cast_mut<U: Data>(&mut self) -> &mut MaybeLocal<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        unsafe { v8cxx__maybe_local_is_empty(self.cast_ref()) }
    }

    #[inline(always)]
    pub fn to_local(&self, out: &mut Local<impl Data>) -> bool {
        unsafe { v8cxx__maybe_local_to_local(self.cast_ref(), out.cast_mut()) }
    }

    #[inline(always)]
    pub fn to_local_checked(&mut self) -> Local<T> {
        let mut local_data = Local::<T>::empty();

        unsafe { v8cxx__maybe_local_to_local_checked(local_data.cast_mut(), self.cast_mut()) };

        local_data
    }

    #[inline(always)]
    pub fn from_maybe<U: Data>(&self, default_value: &Local<U>) -> Local<U> {
        let mut local_data = Local::<U>::empty();

        unsafe {
            v8cxx__maybe_local_from_maybe(
                local_data.cast_mut(),
                self.cast_ref(),
                default_value.cast_ref(),
            )
        };

        local_data
    }
}

impl<T: Data> From<Local<T>> for MaybeLocal<T> {
    fn from(value: Local<T>) -> Self {
        Self(value)
    }
}
