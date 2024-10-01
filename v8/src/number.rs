use crate::{
    data::traits::Data, numeric::traits::Numeric, primitive::traits::Primitive,
    value::traits::Value,
};

#[repr(C)]
pub struct Number([u8; 0]);

impl Data for Number {}
impl Value for Number {}
impl Primitive for Number {}
impl Numeric for Number {}
