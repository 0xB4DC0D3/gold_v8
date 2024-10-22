use crate::{data::traits::Data, isolate::Isolate, local::Local, string::String, value::Value};

extern "C" {
    fn v8cxx__private_name(local_buf: *mut Local<Value>, private: *const Private);
    fn v8cxx__private_new(
        local_buf: *mut Local<Private>,
        isolate: *mut Isolate,
        name: *const Local<String>,
    );
    fn v8cxx__private_for_api(
        local_buf: *mut Local<Private>,
        isolate: *mut Isolate,
        name: *const Local<String>,
    );
}

#[repr(C)]
pub struct Private([u8; 0]);

impl Private {
    #[inline(always)]
    pub fn name(&self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__private_name(&mut local_value, self) };

        local_value.cast()
    }

    #[inline(always)]
    pub fn new(isolate: &mut Isolate, name: &Local<String>) -> Local<Self> {
        let mut local_private = Local::empty();

        unsafe { v8cxx__private_new(&mut local_private, isolate, name) };

        local_private
    }

    #[inline(always)]
    pub fn for_api(isolate: &mut Isolate, name: &Local<String>) -> Local<Self> {
        let mut local_private = Local::empty();

        unsafe { v8cxx__private_for_api(&mut local_private, isolate, name) };

        local_private
    }
}

impl Data for Private {}
