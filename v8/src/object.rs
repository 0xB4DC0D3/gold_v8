use crate::{
    context::Context,
    data::traits::Data,
    isolate::Isolate,
    local::{Local, MaybeLocal},
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
        receiver: *const MaybeLocal<Object>,
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
        maybe_local_buf: *mut MaybeLocal<Value>,
        this: *mut Object,
        context: *const Local<Context>,
        key: *const Local<Value>,
        receiver: *const MaybeLocal<Object>,
    );
    fn v8cxx__object_get_indexed(
        maybe_local_buf: *mut MaybeLocal<Value>,
        this: *mut Object,
        context: *const Local<Context>,
        index: u32,
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
    pub fn new(handle_scope: &HandleScope) -> Local<Self> {
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
    use crate::{local::Local, name::traits::Name, value::traits::Value};

    use super::*;

    pub trait Object: Value {
        fn set(
            &mut self,
            context: &Local<Context>,
            key: &Local<impl Value>,
            value: &Local<impl Value>,
            receiver: Option<&MaybeLocal<super::Object>>,
        ) {
            unsafe {
                v8cxx__object_set(
                    self as *mut _ as *mut _,
                    context,
                    key.cast_ref(),
                    value.cast_ref(),
                    receiver.unwrap_or(&MaybeLocal::empty()),
                )
            };
        }

        fn set_indexed(&mut self, context: &Local<Context>, index: u32, value: &Local<impl Value>) {
            unsafe {
                v8cxx__object_set_indexed(
                    self as *mut _ as *mut _,
                    context,
                    index,
                    value.cast_ref(),
                )
            };
        }

        fn create_data_property(
            &mut self,
            context: &Local<Context>,
            key: &Local<impl Name>,
            value: &Local<impl Value>,
        ) {
            unsafe {
                v8cxx__object_create_data_property(
                    self as *mut _ as *mut _,
                    context,
                    key.cast_ref(),
                    value.cast_ref(),
                )
            };
        }

        fn create_data_property_indexed(
            &mut self,
            context: &Local<Context>,
            index: u32,
            value: &Local<impl Value>,
        ) {
            unsafe {
                v8cxx__object_create_data_property_indexed(
                    self as *mut _ as *mut _,
                    context,
                    index,
                    value.cast_ref(),
                )
            };
        }

        fn define_own_property(
            &mut self,
            context: &Local<Context>,
            key: &Local<impl Name>,
            value: &Local<impl Value>,
            attributes: PropertyAttribute,
        ) -> bool {
            unsafe {
                v8cxx__object_define_own_property(
                    self as *mut _ as *mut _,
                    context,
                    key.cast_ref(),
                    value.cast_ref(),
                    attributes,
                )
            }
        }

        fn get(
            &mut self,
            context: &Local<Context>,
            key: &Local<impl Value>,
            receiver: Option<&MaybeLocal<super::Object>>,
        ) -> MaybeLocal<value::Value> {
            let mut maybe_local_value = MaybeLocal::<value::Value>::empty();

            unsafe {
                v8cxx__object_get(
                    &mut maybe_local_value,
                    self as *mut _ as *mut _,
                    context,
                    key.cast_ref(),
                    receiver.unwrap_or(&MaybeLocal::empty()),
                );
            }

            maybe_local_value
        }

        fn get_indexed(
            &mut self,
            context: &Local<Context>,
            index: u32,
        ) -> MaybeLocal<value::Value> {
            let mut maybe_local_value = MaybeLocal::empty();

            unsafe {
                v8cxx__object_get_indexed(
                    &mut maybe_local_value,
                    self as *mut _ as *mut _,
                    context,
                    index,
                );
            }

            maybe_local_value
        }
    }
}
