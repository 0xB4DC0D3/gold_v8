use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{array_buffer::ArrayBufferAllocator, bindings, context::Context, local::Local};

extern "C" {
    fn v8cxx__isolate__createparams(
        array_buffer_allocator: *mut ArrayBufferAllocator,
    ) -> IsolateCreateParams;

    fn v8cxx__isolate_new(create_params: *const IsolateCreateParams) -> *mut Isolate;
    fn v8cxx__isolate_enter(this: *mut Isolate);
    fn v8cxx__isolate_exit(this: *mut Isolate);
    fn v8cxx__isolate_get_current() -> *mut Isolate;
    fn v8cxx__isolate_get_current_context(local_buf: *mut Local<Context>, this: *mut Isolate);
}

#[repr(C)]
pub struct IsolateCreateParams([u8; bindings::v8cxx__sizeof_isolate__createparams]);

impl IsolateCreateParams {
    #[inline(always)]
    pub fn new(array_buffer_allocator: &mut ArrayBufferAllocator) -> Self {
        unsafe { v8cxx__isolate__createparams(array_buffer_allocator) }
    }
}

#[repr(C)]
pub struct Isolate([u8; 0]);

impl Isolate {
    #[inline(always)]
    pub fn new(create_params: &IsolateCreateParams) -> OwnedIsolate {
        unsafe { OwnedIsolate::new(v8cxx__isolate_new(create_params)) }
    }

    #[inline(always)]
    pub(super) fn enter(&mut self) {
        unsafe { v8cxx__isolate_enter(self) };
    }

    #[inline(always)]
    pub(super) fn exit(&mut self) {
        unsafe { v8cxx__isolate_exit(self) };
    }

    #[inline(always)]
    pub fn get_current<'a>() -> Option<&'a mut Self> {
        unsafe { v8cxx__isolate_get_current().as_mut() }
    }

    #[inline(always)]
    pub fn get_current_context(&mut self) -> Local<Context> {
        let mut local_context = Local::<Context>::empty();

        unsafe { v8cxx__isolate_get_current_context(&mut local_context, self) };

        local_context
    }
}

pub struct OwnedIsolate(NonNull<Isolate>);

impl OwnedIsolate {
    pub fn new(isolate: *mut Isolate) -> Self {
        let mut owned_isolate = Self(NonNull::new(isolate).unwrap());

        owned_isolate.enter();
        owned_isolate
    }
}

impl Deref for OwnedIsolate {
    type Target = Isolate;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnedIsolate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl AsMut<Isolate> for OwnedIsolate {
    fn as_mut(&mut self) -> &mut Isolate {
        self
    }
}

impl Drop for OwnedIsolate {
    fn drop(&mut self) {
        self.exit();
    }
}
