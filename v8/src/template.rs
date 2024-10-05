use crate::{data::traits::Data, local::Local, object::PropertyAttribute};

extern "C" {
    fn v8cxx__template_set(
        this: *mut Template,
        name: *const Local<Name>,
        value: *const Local<data::Data>,
        attributes: PropertyAttribute,
    );
    fn v8cxx__template_set_private(
        this: *mut Template,
        name: *const Local<Name>,
        value: *const Local<data::Data>,
        attributes: PropertyAttribute,
    );
    fn v8cxx__template_set_with_isolate(
        this: *mut Template,
        isolate: *mut Isolate,
        name: *const u8,
        value: *const Local<data::Data>,
        attributes: PropertyAttribute,
    );
}

#[repr(C)]
pub struct Template([u8; 0]);

impl Data for Template {}

pub mod traits {
    use std::ffi::CStr;

    use crate::{
        data::{self, traits::Data},
        local::Local,
        name::Name,
        object::PropertyAttribute,
    };

    use super::{v8cxx__template_set, v8cxx__template_set_with_isolate};

    pub trait Template: Data {
        fn set(
            &mut self,
            name: &Local<impl Name>,
            value: &Local<impl data::Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe { v8cxx__template_set(self, name, value, attributes) };
        }

        fn set_private(
            &mut self,
            name: &Local<impl Name>,
            value: &Local<impl data::Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe { v8cxx__template_set_private(self, name, value, attributes) };
        }

        fn set_with_isolate(
            &mut self,
            isolate: &mut Isolate,
            name: CStr,
            value: &Local<impl data::Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe {
                v8cxx__template_set_with_isolate(self, isolate, name.as_ptr(), value, attributes)
            };
        }
    }
}
