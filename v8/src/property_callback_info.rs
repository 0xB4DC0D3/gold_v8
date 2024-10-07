use std::{
    marker::PhantomData,
    mem::{transmute, MaybeUninit},
};

use crate::{
    bindings,
    isolate::Isolate,
    local::Local,
    object::Object,
    return_value::ReturnValue,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx_property_callback_info_get_isolate(
        this: *const PropertyCallbackInfo<value::Value>,
    ) -> *mut Isolate;
    fn v8cxx_property_callback_info_data(
        local_buf: *mut Local<value::Value>,
        this: *const PropertyCallbackInfo<value::Value>,
    );
    fn v8cxx_property_callback_info_this(
        local_buf: *mut Local<Object>,
        this: *const PropertyCallbackInfo<value::Value>,
    );
    fn v8cxx_property_callback_info_holder(
        local_buf: *mut Local<Object>,
        this: *const PropertyCallbackInfo<value::Value>,
    );
    fn v8cxx_property_callback_info_get_return_value(
        buf: *mut ReturnValue<value::Value>,
        this: *const PropertyCallbackInfo<value::Value>,
    );
    fn v8cxx_property_callback_info_should_throw_on_error(
        this: *const PropertyCallbackInfo<value::Value>,
    ) -> bool;
}

#[repr(C)]
pub struct PropertyCallbackInfo<T>(
    [u8; bindings::v8cxx__sizeof_propertycallbackinfo],
    PhantomData<T>,
);

impl<T> PropertyCallbackInfo<T> {
    #[inline(always)]
    pub fn cast<U: Value>(self) -> PropertyCallbackInfo<U> {
        PropertyCallbackInfo(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn cast_ref<U: Value>(&self) -> &PropertyCallbackInfo<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn cast_mut<U: Value>(&mut self) -> &mut PropertyCallbackInfo<U> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn get_isolate(&self) -> Option<&mut Isolate> {
        unsafe { v8cxx_property_callback_info_get_isolate(self.cast_ref()).as_mut() }
    }

    #[inline(always)]
    pub fn data(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx_property_callback_info_data(&mut local_value, self.cast_ref()) };

        local_value
    }

    #[inline(always)]
    pub fn this(&self) -> Local<Object> {
        let mut local_object = Local::empty();

        unsafe { v8cxx_property_callback_info_this(&mut local_object, self.cast_ref()) };

        local_object
    }

    #[inline(always)]
    pub fn holder(&self) -> Local<Object> {
        let mut local_object = Local::empty();

        unsafe { v8cxx_property_callback_info_holder(&mut local_object, self.cast_ref()) };

        local_object
    }

    #[inline(always)]
    pub fn get_return_value(&self) -> ReturnValue<value::Value> {
        unsafe {
            let mut return_value = MaybeUninit::zeroed();

            v8cxx_property_callback_info_get_return_value(return_value.as_mut_ptr(), self.cast_ref());

            return_value.assume_init()
        }
    }

    #[inline(always)]
    pub fn should_throw_on_error(&self) -> bool {
        unsafe { v8cxx_property_callback_info_should_throw_on_error(self.cast_ref()) }
    }
}
