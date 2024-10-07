use crate::{
    data::traits::Data, isolate::Isolate, local::Local, number::traits::Number,
    numeric::traits::Numeric, primitive::traits::Primitive, value::traits::Value,
};

extern "C" {
    fn v8cxx__integer_new(local_buf: *mut Local<Integer>, isolate: *mut Isolate, value: i32);
    fn v8cxx__integer_new_from_unsigned(
        local_buf: *mut Local<Integer>,
        isolate: *mut Isolate,
        value: u32,
    );
    fn v8cxx__integer_value(this: *const Integer) -> i64;
}

#[repr(C)]
pub struct Integer([u8; 0]);

impl Integer {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, value: i32) -> Local<Self> {
        let mut local_integer = Local::empty();

        unsafe { v8cxx__integer_new(&mut local_integer, isolate, value) };

        local_integer
    }

    #[inline(always)]
    pub fn new_from_unsigned(isolate: &mut Isolate, value: u32) -> Local<Self> {
        let mut local_integer = Local::empty();

        unsafe { v8cxx__integer_new_from_unsigned(&mut local_integer, isolate, value) };

        local_integer
    }
}

impl Data for Integer {}
impl Value for Integer {}
impl Primitive for Integer {}
impl Numeric for Integer {}
impl Number for Integer {}

pub mod traits {
    use crate::number::traits::Number;

    use super::*;

    pub trait Integer: Number {
        fn value(&self) -> i64 {
            unsafe { v8cxx__integer_value(self as *const _ as *const _) }
        }
    }
}
