use crate::{
    context::Context,
    data::traits::Data,
    isolate::Isolate,
    local::Local,
    name::Name,
    scope::HandleScope,
    value::{self, Value},
};

extern "C" {
    fn v8cxx__object_new(local_buf: *mut Local<Object>, isolate: *mut Isolate);
    fn v8cxx__object_set(
        this: *mut Object,
        context: *const Local<Context>,
        key: *const Local<value::Value>,
        value: *const Local<value::Value>,
        receiver: *const Local<Object>,
    ) -> bool;
    fn v8cxx__object_set_indexed(
        this: *mut Object,
        context: *const Local<Context>,
        index: u32,
        value: *const Local<value::Value>,
    ) -> bool;
    fn v8cxx__object_create_data_property(
        this: *mut Object,
        context: *const Local<Context>,
        key: *const Local<Name>,
        value: *const Local<value::Value>,
    ) -> bool;
    fn v8cxx__object_create_data_property_indexed(
        this: *mut Object,
        context: *const Local<Context>,
        index: u32,
        value: *const Local<value::Value>,
    ) -> bool;
    fn v8cxx__object_define_own_property(
        this: *mut Object,
        context: *const Local<Context>,
        key: *const Local<Name>,
        value: *const Local<Value>,
        attributes: PropertyAttribute,
    ) -> bool;
    fn v8cxx__object_get(
        local_buf: *mut Local<Value>,
        this: *mut Object,
        context: *const Local<Context>,
        key: *const Local<Value>,
        receiver: *const Local<Object>,
    );
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum PropertyAttribute {
    None = 0,
    ReadOnly = 1 << 0,
    DontEnum = 1 << 1,
    DontDelete = 1 << 2,
}

#[repr(C)]
pub struct Object([u8; 0]);

impl Object {
    #[inline(always)]
    pub fn new(handle_scope: &mut HandleScope) -> Local<Self> {
        let mut local_object = Local::<Self>::empty();

        unsafe {
            v8cxx__object_new(&mut local_object, handle_scope.get_isolate().unwrap());
        }

        local_object
    }
}

impl Data for Object {}
impl value::traits::Value for Object {}
impl traits::Object for Object {}

pub mod traits {
    use crate::{
        context::Context,
        local::Local,
        name::Name,
        value::{self, traits::Value},
    };

    use super::*;

    pub trait Object: Value {
        fn set(
            &mut self,
            context: &Local<Context>,
            key: &Local<value::Value>,
            value: &Local<value::Value>,
            receiver: Option<&Local<super::Object>>,
        ) {
            unsafe {
                v8cxx__object_set(
                    self as *mut _ as *mut _,
                    context,
                    key,
                    value,
                    match receiver {
                        Some(receiver) => receiver,
                        None => std::ptr::null::<Local<super::Object>>(),
                    },
                )
            };
        }

        fn set_indexed(
            &mut self,
            context: &Local<Context>,
            index: u32,
            value: &Local<value::Value>,
        ) {
            unsafe { v8cxx__object_set_indexed(self as *mut _ as *mut _, context, index, value) };
        }

        fn create_data_property(
            &mut self,
            context: &Local<Context>,
            key: &Local<Name>,
            value: &Local<value::Value>,
        ) {
            unsafe {
                v8cxx__object_create_data_property(self as *mut _ as *mut _, context, key, value)
            };
        }

        fn create_data_property_indexed(
            &mut self,
            context: &Local<Context>,
            index: u32,
            value: &Local<value::Value>,
        ) {
            unsafe {
                v8cxx__object_create_data_property_indexed(
                    self as *mut _ as *mut _,
                    context,
                    index,
                    value,
                )
            };
        }

        fn define_own_property(
            &mut self,
            context: &Local<Context>,
            key: &Local<Name>,
            value: &Local<value::Value>,
            attributes: PropertyAttribute,
        ) -> bool {
            unsafe {
                v8cxx__object_define_own_property(
                    self as *mut _ as *mut _,
                    context,
                    key,
                    value,
                    attributes,
                )
            }
        }

        fn get(
            &mut self,
            context: &Local<Context>,
            key: &Local<value::Value>,
            receiver: Option<&Local<super::Object>>,
        ) -> Local<value::Value> {
            let mut local_value = Local::<value::Value>::empty();

            unsafe {
                v8cxx__object_get(
                    &mut local_value,
                    self as *mut _ as *mut _,
                    context,
                    key,
                    match receiver {
                        Some(receiver) => receiver,
                        None => std::ptr::null::<Local<super::Object>>(),
                    },
                );
            }

            local_value
        }
    }
}
