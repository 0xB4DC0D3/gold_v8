use std::ffi::c_void;

use crate::{
    data::traits::Data,
    isolate::Isolate,
    local::Local,
    object::traits::Object,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__array_buffer_byte_length(this: *const ArrayBuffer) -> usize;
    fn v8cxx__array_buffer_max_byte_length(this: *const ArrayBuffer) -> usize;
    fn v8cxx__array_buffer_new(
        local_buf: *mut Local<ArrayBuffer>,
        isolate: *mut Isolate,
        byte_length: usize,
        init_mode: BackingStoreInitializationMode,
    );
    fn v8cxx__array_buffer_is_detachable(this: *const ArrayBuffer) -> bool;
    fn v8cxx__array_buffer_was_detached(this: *const ArrayBuffer) -> bool;
    fn v8cxx__array_buffer_detach(this: *mut ArrayBuffer, key: *const Local<value::Value>) -> bool;
    fn v8cxx__array_buffer_is_resizable_by_user_javascript(this: *const ArrayBuffer) -> bool;
    fn v8cxx__array_buffer_data(this: *const ArrayBuffer) -> *mut c_void;
}

#[repr(C)]
pub struct ArrayBuffer([u8; 0]);

impl ArrayBuffer {
    #[inline(always)]
    pub fn new(
        isolate: &mut Isolate,
        byte_length: usize,
        init_mode: BackingStoreInitializationMode,
    ) -> Local<Self> {
        let mut local_array_buffer = Local::empty();

        unsafe {
            v8cxx__array_buffer_new(&mut local_array_buffer, isolate, byte_length, init_mode)
        };

        local_array_buffer
    }
}

impl Data for ArrayBuffer {}
impl Value for ArrayBuffer {}
impl Object for ArrayBuffer {}

pub mod traits {
    use crate::object::traits::Object;

    use super::*;

    pub trait ArrayBuffer: Object {
        fn byte_length(&self) -> usize {
            unsafe { v8cxx__array_buffer_byte_length(self as *const _ as *const _) }
        }

        fn max_byte_length(&self) -> usize {
            unsafe { v8cxx__array_buffer_max_byte_length(self as *const _ as *const _) }
        }

        fn is_detachable(&self) -> bool {
            unsafe { v8cxx__array_buffer_is_detachable(self as *const _ as *const _) }
        }

        fn was_detached(&self) -> bool {
            unsafe { v8cxx__array_buffer_was_detached(self as *const _ as *const _) }
        }

        fn detach(&mut self, key: &Local<value::Value>) -> bool {
            unsafe { v8cxx__array_buffer_detach(self as *mut _ as *mut _, key) }
        }

        fn is_resizable_by_user_javascript(&self) -> bool {
            unsafe {
                v8cxx__array_buffer_is_resizable_by_user_javascript(self as *const _ as *const _)
            }
        }

        fn data(&self) -> *mut c_void {
            unsafe { v8cxx__array_buffer_data(self as *const _ as *const _) }
        }
    }
}

#[repr(C)]
pub enum BackingStoreInitializationMode {
    ZeroInitialized,
    Uninitialized,
}

extern "C" {
    fn v8cxx__array_buffer__allocator_new_default_allocator() -> *mut ArrayBufferAllocator;
}

#[repr(C)]
pub struct ArrayBufferAllocator([u8; 0]);

impl ArrayBufferAllocator {
    #[inline(always)]
    pub fn new<'a>() -> Option<&'a mut Self> {
        unsafe { v8cxx__array_buffer__allocator_new_default_allocator().as_mut() }
    }
}
