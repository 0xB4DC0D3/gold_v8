use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use crate::{bindings, context::Context, isolate::Isolate, local::Local};

extern "C" {
    fn v8cxx__handlescope_new(buf: *mut HandleScope, isolate: *mut Isolate);
    fn v8cxx__handlescope_drop(this: *mut HandleScope);
    fn v8cxx__handlescope_get_isolate(this: *const HandleScope) -> *mut Isolate;
}

#[repr(C)]
pub struct HandleScope([u8; bindings::v8cxx__sizeof_handlescope]);

impl HandleScope {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate) -> Self {
        unsafe {
            let mut buf = MaybeUninit::uninit();

            v8cxx__handlescope_new(buf.as_mut_ptr(), isolate);

            buf.assume_init()
        }
    }

    #[inline(always)]
    pub fn get_isolate(&self) -> Option<&mut Isolate> {
        unsafe { v8cxx__handlescope_get_isolate(self).as_mut() }
    }
}

impl Drop for HandleScope {
    fn drop(&mut self) {
        unsafe { v8cxx__handlescope_drop(self) };
    }
}

impl Deref for HandleScope {
    type Target = Isolate;
    fn deref(&self) -> &Self::Target {
        self.get_isolate()
            .expect("Can't get `Isolate` from `HandleScope`")
    }
}

impl DerefMut for HandleScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_isolate()
            .expect("Can't get mutable `Isolate` from `HandleScope`")
    }
}

pub struct ContextScope(Local<Context>);

impl ContextScope {
    pub fn new(context: Local<Context>) -> Self {
        let mut context_scope = Self(context);

        context_scope.0.enter();
        context_scope
    }
}

impl Deref for ContextScope {
    type Target = Local<Context>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ContextScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for ContextScope {
    fn drop(&mut self) {
        self.0.exit();
    }
}
