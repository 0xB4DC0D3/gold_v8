use crate::{
    data::traits::Data, isolate::Isolate, local::Local, scope::HandleScope, value::traits::Value,
};

extern "C" {
    fn v8cxx__primitive_undefined(local_buf: *mut Local<Primitive>, isolate: *mut Isolate);
    fn v8cxx__primitive_null(local_buf: *mut Local<Primitive>, isolate: *mut Isolate);
}

#[repr(C)]
pub struct Primitive([u8; 0]);

impl Primitive {
    #[inline(always)]
    pub fn undefined(handle_scope: &mut HandleScope) -> Local<Self> {
        let mut local_primitive = Local::<Self>::empty();

        unsafe {
            v8cxx__primitive_undefined(&mut local_primitive, handle_scope.get_isolate().unwrap())
        };

        local_primitive
    }

    #[inline(always)]
    pub fn null(handle_scope: &mut HandleScope) -> Local<Self> {
        let mut local_primitive = Local::<Self>::empty();

        unsafe { v8cxx__primitive_null(&mut local_primitive, handle_scope.get_isolate().unwrap()) };

        local_primitive
    }
}

impl Data for Primitive {}
impl Value for Primitive {}
impl traits::Primitive for Primitive {}

pub mod traits {
    use crate::value::traits::Value;

    pub trait Primitive: Value {}
}
