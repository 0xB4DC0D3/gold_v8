use crate::{data::traits::Data, primitive::traits::Primitive, value::traits::Value};

extern "C" {
    fn v8cxx__name_get_identity_hash(this: *mut Name) -> i32;
}

#[repr(C)]
pub struct Name([u8; 0]);

impl Data for Name {}
impl Value for Name {}
impl Primitive for Name {}
impl traits::Name for Name {}

pub mod traits {
    use crate::primitive::traits::Primitive;

    use super::v8cxx__name_get_identity_hash;

    pub trait Name: Primitive {
        fn get_identity_hash(&mut self) -> i32 {
            unsafe { v8cxx__name_get_identity_hash(self as *mut _ as *mut _) }
        }
    }
}
