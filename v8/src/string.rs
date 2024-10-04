use core::str;

use crate::{
    data::traits::Data,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    primitive::traits::Primitive,
    value::traits::Value,
};

extern "C" {
    fn v8cxx__string_new_from_utf8(
        maybe_local_buf: *mut MaybeLocal<String>,
        isolate: *mut Isolate,
        value: *const u8,
        string_type: NewStringType,
        length: i32,
    );

    fn v8cxx__string_new_from_onebyte(
        maybe_local_buf: *mut MaybeLocal<String>,
        isolate: *mut Isolate,
        value: *const u8,
        string_type: NewStringType,
        length: i32,
    );

    fn v8cxx__string_new_from_twobyte(
        maybe_local_buf: *mut MaybeLocal<String>,
        isolate: *mut Isolate,
        value: *const u16,
        string_type: NewStringType,
        length: i32,
    );

    fn v8cxx__string_length(string: *const String) -> i32;
    fn v8cxx__string_utf8length(string: *const String, isolate: *mut Isolate) -> i32;
    fn v8cxx__string_is_onebyte(string: *const String) -> bool;
    fn v8cxx__string_contains_only_onebyte(string: *const String) -> bool;
    fn v8cxx__string_is_external(string: *const String) -> bool;
    fn v8cxx__string_is_external_twobyte(string: *const String) -> bool;
    fn v8cxx__string_is_external_onebyte(string: *const String) -> bool;
    fn v8cxx__string_internalize_string(
        local_buf: *mut Local<String>,
        string: *mut String,
        isolate: *mut Isolate,
    );
    fn v8cxx__string_view(this: *const String, isolate: *mut Isolate) -> *const u8;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum NewStringType {
    Normal,
    Internalized,
}

#[repr(C)]
pub struct String([u8; 0]);

impl String {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, value: &str) -> MaybeLocal<Self> {
        Self::new_from_utf8(isolate, value, NewStringType::Normal)
    }

    #[inline(always)]
    pub fn new_from_utf8(
        isolate: &mut Isolate,
        value: &str,
        string_type: NewStringType,
    ) -> MaybeLocal<Self> {
        let mut maybe_local_string = MaybeLocal::<Self>::empty();

        unsafe {
            v8cxx__string_new_from_utf8(
                &mut maybe_local_string,
                isolate,
                value.as_ptr(),
                string_type,
                value.len() as i32,
            );
        }

        maybe_local_string
    }

    #[inline(always)]
    pub fn new_from_onebyte(
        isolate: &mut Isolate,
        value: &[u8],
        string_type: NewStringType,
    ) -> MaybeLocal<Self> {
        let mut maybe_local_string = MaybeLocal::<Self>::empty();

        unsafe {
            v8cxx__string_new_from_onebyte(
                &mut maybe_local_string,
                isolate,
                value.as_ptr(),
                string_type,
                value.len() as i32,
            );
        }

        maybe_local_string
    }

    #[inline(always)]
    pub fn new_from_twobyte(
        isolate: &mut Isolate,
        value: &[u16],
        string_type: NewStringType,
    ) -> MaybeLocal<Self> {
        let mut maybe_local_string = MaybeLocal::<Self>::empty();

        unsafe {
            v8cxx__string_new_from_twobyte(
                &mut maybe_local_string,
                isolate,
                value.as_ptr(),
                string_type,
                value.len() as i32,
            );
        }

        maybe_local_string
    }

    #[inline(always)]
    pub fn length(&self) -> i32 {
        unsafe { v8cxx__string_length(self) }
    }

    #[inline(always)]
    pub fn utf8length(&self, isolate: &mut Isolate) -> i32 {
        unsafe { v8cxx__string_utf8length(self, isolate) }
    }

    #[inline(always)]
    pub fn is_onebyte(&self) -> bool {
        unsafe { v8cxx__string_is_onebyte(self) }
    }

    #[inline(always)]
    pub fn contains_only_onebyte(&self) -> bool {
        unsafe { v8cxx__string_contains_only_onebyte(self) }
    }

    #[inline(always)]
    pub fn is_external(&self) -> bool {
        unsafe { v8cxx__string_is_external(self) }
    }

    #[inline(always)]
    pub fn is_external_twobyte(&self) -> bool {
        unsafe { v8cxx__string_is_external_twobyte(self) }
    }

    #[inline(always)]
    pub fn is_external_onebyte(&self) -> bool {
        unsafe { v8cxx__string_is_external_onebyte(self) }
    }

    #[inline(always)]
    pub fn internalize_string(&mut self, isolate: &mut Isolate) -> Local<Self> {
        let mut local_string = Local::<Self>::empty();

        unsafe {
            v8cxx__string_internalize_string(&mut local_string, self, isolate);
        }

        local_string
    }

    #[inline(always)]
    pub fn as_str(&self, isolate: &mut Isolate) -> &str {
        unsafe {
            let length = self.length() as usize;
            let str_buffer = v8cxx__string_view(self, isolate);

            std::str::from_utf8(std::slice::from_raw_parts(str_buffer, length)).unwrap()
        }
    }
}

impl Data for String {}
impl Value for String {}
impl Primitive for String {}
