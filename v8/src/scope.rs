use crate::{bindings, isolate::Isolate};

extern "C" {
    fn v8cxx__handlescope_new(isolate: *mut Isolate) -> *mut HandleScope;
    fn v8cxx__handlescope_drop(this: *mut HandleScope);
    fn v8cxx__handlescope_get_isolate(this: *mut HandleScope) -> *mut Isolate;
}

#[repr(C)]
pub struct HandleScope([u8; bindings::v8cxx__sizeof_handlescope]);

impl HandleScope {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate) -> Option<&mut Self> {
        unsafe { v8cxx__handlescope_new(isolate).as_mut() }
    }

    #[inline(always)]
    pub fn get_isolate(&mut self) -> Option<&mut Isolate> {
        unsafe { v8cxx__handlescope_get_isolate(self).as_mut() }
    }
}

impl Drop for HandleScope {
    fn drop(&mut self) {
        println!("HandleScope::drop");
        unsafe { v8cxx__handlescope_drop(self) };
    }
}
