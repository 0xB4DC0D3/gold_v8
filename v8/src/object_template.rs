use crate::data::traits::Data;

#[repr(C)]
pub struct ObjectTemplate([u8; 0]);

impl Data for ObjectTemplate {}
