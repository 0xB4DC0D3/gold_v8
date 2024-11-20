use crate::{
    data::traits::Data, isolate::Isolate, local::Local, numeric::traits::Numeric,
    primitive::traits::Primitive, value::traits::Value,
};

extern "C" {
    fn v8cxx__bigint_new(local_buf: *mut Local<BigInt>, isolate: *mut Isolate, value: i64);
    fn v8cxx__bigint_new_from_unsigned(
        local_buf: *mut Local<BigInt>,
        isolate: *mut Isolate,
        value: u64,
    );
    fn v8cxx__bigint_uint64_value(this: *const BigInt, lossless: *mut bool) -> u64;
    fn v8cxx__bigint_int64_value(this: *const BigInt, lossless: *mut bool) -> i64;
}

#[repr(C)]
pub struct BigInt([u8; 0]);

impl BigInt {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, value: i64) -> Local<Self> {
        let mut local_bigint = Local::empty();

        unsafe {
            v8cxx__bigint_new(&mut local_bigint, isolate, value);
        }

        local_bigint
    }

    #[inline(always)]
    pub fn new_from_unsigned(isolate: &mut Isolate, value: u64) -> Local<Self> {
        let mut local_bigint = Local::empty();

        unsafe { v8cxx__bigint_new_from_unsigned(&mut local_bigint, isolate, value) };

        local_bigint
    }

    #[inline(always)]
    pub fn u64(&self, lossless: Option<&mut bool>) -> u64 {
        unsafe {
            v8cxx__bigint_uint64_value(
                self,
                match lossless {
                    Some(lossless) => lossless,
                    None => std::ptr::null_mut(),
                },
            )
        }
    }

    #[inline(always)]
    pub fn i64(&self, lossless: Option<&mut bool>) -> i64 {
        unsafe {
            v8cxx__bigint_int64_value(
                self,
                match lossless {
                    Some(lossless) => lossless,
                    None => std::ptr::null_mut(),
                },
            )
        }
    }
}

impl Data for BigInt {}
impl Value for BigInt {}
impl Primitive for BigInt {}
impl Numeric for BigInt {}
