use crate::{data::traits::Data, primitive::traits::Primitive, value::traits::Value};

#[repr(C)]
pub struct Numeric([u8; 0]);

impl Data for Numeric {}
impl Value for Numeric {}
impl Primitive for Numeric {}

pub mod traits {
    use crate::primitive::traits::Primitive;

    pub trait Numeric: Primitive {}
}
