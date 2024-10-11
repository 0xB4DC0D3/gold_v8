use std::mem::MaybeUninit;

use crate::{
    bindings,
    context::Context,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    value::Value,
};

extern "C" {
    fn v8cxx__try_catch_new(buf: *mut TryCatch, isolate: *mut Isolate);
    fn v8cxx__try_catch_drop(this: *mut TryCatch);
    fn v8cxx__try_catch_has_caught(this: *const TryCatch) -> bool;
    fn v8cxx__try_catch_can_continue(this: *const TryCatch) -> bool;
    fn v8cxx__try_catch_has_terminated(this: *const TryCatch) -> bool;
    fn v8cxx__try_catch_rethrow(local_buf: *mut Local<Value>, this: *mut TryCatch);
    fn v8cxx__try_catch_exception(local_buf: *mut Local<Value>, this: *const TryCatch);
    fn v8cxx__try_catch_stack_trace(
        maybe_local_buf: *mut MaybeLocal<Value>,
        context: *const Local<Context>,
        exception: *const Local<Value>,
    );
    // TODO: TryCatch::message
    fn v8cxx__try_catch_reset(this: *mut TryCatch);
    fn v8cxx__try_catch_set_verbose(this: *mut TryCatch, value: bool);
    fn v8cxx__try_catch_is_verbose(this: *const TryCatch) -> bool;
    fn v8cxx__try_catch_set_capture_message(this: *mut TryCatch, value: bool);
}

#[repr(C)]
pub struct TryCatch([u8; bindings::v8cxx__sizeof_trycatch]);

impl TryCatch {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate) -> Self {
        unsafe {
            let mut try_catch = MaybeUninit::uninit();

            v8cxx__try_catch_new(try_catch.as_mut_ptr(), isolate);

            try_catch.assume_init()
        }
    }

    #[inline(always)]
    pub fn has_caught(&self) -> bool {
        unsafe { v8cxx__try_catch_has_caught(self) }
    }

    #[inline(always)]
    pub fn can_continue(&self) -> bool {
        unsafe { v8cxx__try_catch_can_continue(self) }
    }

    #[inline(always)]
    pub fn has_terminated(&self) -> bool {
        unsafe { v8cxx__try_catch_has_terminated(self) }
    }

    #[inline(always)]
    pub fn rethrow(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__try_catch_rethrow(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn exception(&self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__try_catch_exception(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn stack_trace(context: &Local<Context>, exception: &Local<Value>) -> MaybeLocal<Value> {
        let mut maybe_local_value = MaybeLocal::empty();

        unsafe { v8cxx__try_catch_stack_trace(&mut maybe_local_value, context, exception) };

        maybe_local_value
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        unsafe { v8cxx__try_catch_reset(self) };
    }

    #[inline(always)]
    pub fn set_verbose(&mut self, value: bool) {
        unsafe { v8cxx__try_catch_set_verbose(self, value) };
    }

    #[inline(always)]
    pub fn is_verbose(&self) -> bool {
        unsafe { v8cxx__try_catch_is_verbose(self) }
    }

    #[inline(always)]
    pub fn set_capture_message(&mut self, value: bool) {
        unsafe { v8cxx__try_catch_set_capture_message(self, value) };
    }
}

impl Drop for TryCatch {
    fn drop(&mut self) {
        unsafe { v8cxx__try_catch_drop(self) };
    }
}
