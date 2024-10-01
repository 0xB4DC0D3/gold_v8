use crate::{
    data::traits::Data, isolate::Isolate, local::Local, primitive::traits::Primitive,
    scope::HandleScope, value::traits::Value,
};

extern "C" {
    fn v8cxx__boolean_new(local_buf: *mut Local<Boolean>, isolate: *mut Isolate, value: bool);
    fn v8cxx__boolean_value(this: *const Boolean) -> bool;
}

#[repr(C)]
pub struct Boolean([u8; 0]);

impl Boolean {
    #[inline(always)]
    pub fn new(handle_scope: &mut HandleScope, value: bool) -> Local<Self> {
        let mut local_boolean = Local::<Self>::empty();

        unsafe {
            v8cxx__boolean_new(
                &mut local_boolean,
                handle_scope.get_isolate().unwrap(),
                value,
            );
        }

        local_boolean
    }
}

impl Into<bool> for &Boolean {
    fn into(self) -> bool {
        unsafe { v8cxx__boolean_value(self) }
    }
}

impl Data for Boolean {}
impl Value for Boolean {}
impl Primitive for Boolean {}
