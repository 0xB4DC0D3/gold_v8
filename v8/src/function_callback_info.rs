use std::{marker::PhantomData, mem::{transmute, MaybeUninit}};

use crate::{
    bindings,
    isolate::Isolate,
    local::Local,
    object::Object,
    return_value::ReturnValue,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__function_callback_info_length(this: *const FunctionCallbackInfo<value::Value>)
        -> i32;
    fn v8cxx__function_callback_info_at(
        local_buf: *mut Local<value::Value>,
        this: *const FunctionCallbackInfo<value::Value>,
        index: i32,
    );
    fn v8cxx__function_callback_info_this(
        local_buf: *mut Local<Object>,
        this: *const FunctionCallbackInfo<value::Value>,
    );
    fn v8cxx__function_callback_info_new_target(
        local_buf: *mut Local<value::Value>,
        this: *const FunctionCallbackInfo<value::Value>,
    );
    fn v8cxx__function_callback_info_is_construct_call(
        this: *const FunctionCallbackInfo<value::Value>,
    ) -> bool;
    fn v8cxx__function_callback_info_data(
        local_buf: *mut Local<value::Value>,
        this: *const FunctionCallbackInfo<value::Value>,
    );
    fn v8cxx__function_callback_info_get_isolate(
        this: *const FunctionCallbackInfo<value::Value>,
    ) -> *mut Isolate;
    fn v8cxx__function_callback_info_get_return_value(
        buf: *mut ReturnValue<value::Value>,
        this: *const FunctionCallbackInfo<value::Value>,
    );
}

#[repr(C)]
pub struct FunctionCallbackInfo<T: Value>(
    [u8; bindings::v8cxx__sizeof_functioncallbackinfo],
    PhantomData<T>,
);

impl<T: Value> FunctionCallbackInfo<T> {
    #[inline(always)]
    pub fn cast<U: Value>(self) -> FunctionCallbackInfo<U> {
        FunctionCallbackInfo(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn cast_ref<U: Value>(&self) -> &FunctionCallbackInfo<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn cast_mut<U: Value>(&mut self) -> &mut FunctionCallbackInfo<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn length(&self) -> i32 {
        unsafe { v8cxx__function_callback_info_length(self.cast_ref()) }
    }

    #[inline(always)]
    pub fn at(&self, index: i32) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__function_callback_info_at(&mut local_value, self.cast_ref(), index) };

        local_value
    }

    #[inline(always)]
    pub fn this(&self) -> Local<Object> {
        let mut local_object = Local::empty();

        unsafe { v8cxx__function_callback_info_this(&mut local_object, self.cast_ref()) };

        local_object
    }

    #[inline(always)]
    pub fn new_target(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__function_callback_info_new_target(&mut local_value, self.cast_ref()) };

        local_value
    }

    #[inline(always)]
    pub fn is_construct_call(&self) -> bool {
        unsafe { v8cxx__function_callback_info_is_construct_call(self.cast_ref()) }
    }

    #[inline(always)]
    pub fn data(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__function_callback_info_data(&mut local_value, self.cast_ref()) };

        local_value
    }

    #[inline(always)]
    pub fn get_isolate(&self) -> Option<&mut Isolate> {
        unsafe { v8cxx__function_callback_info_get_isolate(self.cast_ref()).as_mut() }
    }

    #[inline(always)]
    pub fn get_return_value(&self) -> ReturnValue<value::Value> {
        unsafe {
            let mut return_value = MaybeUninit::zeroed();

            v8cxx__function_callback_info_get_return_value(
                return_value.as_mut_ptr(),
                self.cast_ref(),
            );

            return_value.assume_init()
        }
    }
}
