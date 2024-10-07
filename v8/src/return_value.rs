use std::{mem::transmute, ptr::NonNull};

use crate::{
    isolate::Isolate,
    local::Local,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__return_value_set(
        this: *mut ReturnValue<value::Value>,
        value: *const Local<value::Value>,
    );
    fn v8cxx__return_value_set_bool(this: *mut ReturnValue<value::Value>, value: bool);
    fn v8cxx__return_value_set_f64(this: *mut ReturnValue<value::Value>, value: f64);
    fn v8cxx__return_value_set_i16(this: *mut ReturnValue<value::Value>, value: i16);
    fn v8cxx__return_value_set_i32(this: *mut ReturnValue<value::Value>, value: i32);
    fn v8cxx__return_value_set_i64(this: *mut ReturnValue<value::Value>, value: i64);
    fn v8cxx__return_value_set_u16(this: *mut ReturnValue<value::Value>, value: u16);
    fn v8cxx__return_value_set_u32(this: *mut ReturnValue<value::Value>, value: u32);
    fn v8cxx__return_value_set_u64(this: *mut ReturnValue<value::Value>, value: u64);
    fn v8cxx__return_value_set_null(this: *mut ReturnValue<value::Value>);
    fn v8cxx__return_value_set_undefined(this: *mut ReturnValue<value::Value>);
    fn v8cxx__return_value_set_false(this: *mut ReturnValue<value::Value>);
    fn v8cxx__return_value_set_empty_string(this: *mut ReturnValue<value::Value>);
    fn v8cxx__return_value_get_isolate(this: *const ReturnValue<value::Value>) -> *mut Isolate;
    fn v8cxx__return_value_get(
        local_buf: *mut Local<value::Value>,
        this: *const ReturnValue<value::Value>,
    );
}

#[repr(C)]
pub struct ReturnValue<T: Value>(NonNull<T>);

impl<T: Value> ReturnValue<T> {
    #[inline(always)]
    pub fn cast<U: Value>(self) -> ReturnValue<U> {
        ReturnValue(self.0.cast())
    }

    #[inline(always)]
    pub fn cast_ref<U: Value>(&self) -> &ReturnValue<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn cast_mut<U: Value>(&mut self) -> &mut ReturnValue<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn set(&mut self, value: &Local<impl Value>) {
        unsafe { v8cxx__return_value_set(self.cast_mut(), value.cast_ref()) };
    }

    #[inline(always)]
    pub fn set_bool(&mut self, value: bool) {
        unsafe { v8cxx__return_value_set_bool(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_f64(&mut self, value: f64) {
        unsafe { v8cxx__return_value_set_f64(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_i16(&mut self, value: i16) {
        unsafe { v8cxx__return_value_set_i16(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_i32(&mut self, value: i32) {
        unsafe { v8cxx__return_value_set_i32(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_i64(&mut self, value: i64) {
        unsafe { v8cxx__return_value_set_i64(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_u16(&mut self, value: u16) {
        unsafe { v8cxx__return_value_set_u16(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_u32(&mut self, value: u32) {
        unsafe { v8cxx__return_value_set_u32(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_u64(&mut self, value: u64) {
        unsafe { v8cxx__return_value_set_u64(self.cast_mut(), value) };
    }

    #[inline(always)]
    pub fn set_null(&mut self) {
        unsafe { v8cxx__return_value_set_null(self.cast_mut()) };
    }

    #[inline(always)]
    pub fn set_undefined(&mut self) {
        unsafe { v8cxx__return_value_set_undefined(self.cast_mut()) };
    }

    #[inline(always)]
    pub fn set_false(&mut self) {
        unsafe { v8cxx__return_value_set_false(self.cast_mut()) };
    }

    #[inline(always)]
    pub fn set_empty_string(&mut self) {
        unsafe { v8cxx__return_value_set_empty_string(self.cast_mut()) };
    }

    #[inline(always)]
    pub fn get_isolate(&self) -> Option<&mut Isolate> {
        unsafe { v8cxx__return_value_get_isolate(self.cast_ref()).as_mut() }
    }

    #[inline(always)]
    pub fn get(&self) -> Local<impl Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__return_value_get(&mut local_value, self.cast_ref()) };

        local_value
    }
}
