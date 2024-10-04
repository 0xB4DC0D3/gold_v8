use crate::{
    data::traits::Data, isolate::Isolate, local::Local, primitive::traits::Primitive,
    value::traits::Value,
};

extern "C" {
    fn v8cxx__bigint_new(local_buf: *mut Local<BigInt>, isolate: *mut Isolate, value: i64);
}

#[repr(C)]
pub struct BigInt([u8; 0]);

impl BigInt {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, value: i64) -> Local<Self> {
        let mut local_bigint = Local::<Self>::empty();

        unsafe {
            v8cxx__bigint_new(&mut local_bigint, isolate, value);
        }

        local_bigint
    }
}

impl Data for BigInt {}
impl Primitive for BigInt {}
impl Value for BigInt {}
