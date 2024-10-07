use crate::{
    data::traits::Data, isolate::Isolate, local::Local, numeric::traits::Numeric,
    primitive::traits::Primitive, value::traits::Value,
};

extern "C" {
    fn v8cxx__number_new(local_buf: *mut Local<Number>, isolate: *mut Isolate, value: f64);
    fn v8cxx__number_value(this: *const Number) -> f64;
}

#[repr(C)]
pub struct Number([u8; 0]);

impl Number {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, value: f64) -> Local<Self> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__number_new(&mut local_value, isolate, value) };

        local_value
    }
}

impl Data for Number {}
impl Value for Number {}
impl Primitive for Number {}
impl Numeric for Number {}

pub mod traits {
    use crate::numeric::traits::Numeric;

    use super::v8cxx__number_value;

    pub trait Number: Numeric {
        fn value(&self) -> f64 {
            unsafe { v8cxx__number_value(self as *const _ as *const _) }
        }
    }
}
