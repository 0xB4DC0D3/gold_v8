use crate::{
    data::{self, traits::Data},
    function_template::FunctionTemplate,
    isolate::Isolate,
    local::Local,
    name::Name,
    object::PropertyAttribute,
    private::Private,
};

extern "C" {
    fn v8cxx__template_set(
        this: *mut Template,
        name: *const Local<Name>,
        value: *const Local<data::Data>,
        attributes: PropertyAttribute,
    );
    fn v8cxx__template_set_private(
        this: *mut Template,
        name: *const Local<Private>,
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
    fn v8cxx__template_set_accessor_property(
        this: *mut Template,
        name: *const Local<Name>,
        getter: *const Local<FunctionTemplate>,
        setter: *const Local<FunctionTemplate>,
        attribute: PropertyAttribute,
    );
}

#[repr(C)]
pub struct Template([u8; 0]);

impl Data for Template {}
impl traits::Template for Template {}

pub mod traits {
    use std::ffi::CString;

    use crate::{
        data::traits::Data, function_template::FunctionTemplate, isolate::Isolate, local::Local,
        name::traits::Name, object::PropertyAttribute,
    };

    use super::*;

    pub trait Template: Data {
        fn set(
            &mut self,
            name: &Local<impl Name>,
            value: &Local<impl Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe {
                v8cxx__template_set(
                    self as *mut _ as *mut _,
                    name.cast_ref(),
                    value.cast_ref(),
                    attributes,
                )
            };
        }

        fn set_private(
            &mut self,
            name: &Local<impl Name>,
            value: &Local<impl Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe {
                v8cxx__template_set_private(
                    self as *mut _ as *mut _,
                    name.cast_ref(),
                    value.cast_ref(),
                    attributes,
                )
            };
        }

        fn set_with_isolate(
            &mut self,
            isolate: &mut Isolate,
            name: CString,
            value: &Local<impl Data>,
            attributes: PropertyAttribute,
        ) {
            unsafe {
                v8cxx__template_set_with_isolate(
                    self as *mut _ as *mut _,
                    isolate,
                    name.as_ptr() as *const u8,
                    value.cast_ref(),
                    attributes,
                )
            };
        }

        fn set_accessor_property(
            &mut self,
            name: &Local<impl Name>,
            getter: &Local<FunctionTemplate>,
            setter: &Local<FunctionTemplate>,
            attribute: PropertyAttribute,
        ) {
            unsafe {
                v8cxx__template_set_accessor_property(
                    self as *mut _ as *mut _,
                    name.cast_ref(),
                    getter,
                    setter,
                    attribute,
                )
            };
        }
    }
}
