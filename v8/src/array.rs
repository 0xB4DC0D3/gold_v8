use crate::{
    data::traits::Data,
    isolate::Isolate,
    local::Local,
    object::traits::Object,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__array_length(this: *const Array) -> u32;
    fn v8cxx__array_new(local_buf: *mut Local<Array>, isolate: *mut Isolate, length: i32);
    fn v8cxx__array_new_with_elements(
        local_buf: *mut Local<Array>,
        isolate: *mut Isolate,
        elements: *const Local<value::Value>,
        length: usize,
    );
}

#[repr(C)]
pub struct Array([u8; 0]);

impl Array {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, length: i32) -> Local<Self> {
        let mut local_array = Local::empty();

        unsafe { v8cxx__array_new(&mut local_array, isolate, length) };

        local_array
    }

    #[inline(always)]
    pub fn new_with_elements(
        isolate: &mut Isolate,
        elements: &Vec<Local<value::Value>>,
    ) -> Local<Self> {
        let mut local_array = Local::empty();

        unsafe {
            v8cxx__array_new_with_elements(
                &mut local_array,
                isolate,
                elements.as_ptr(),
                elements.len(),
            )
        };

        local_array
    }

    #[inline(always)]
    pub fn length(&self) -> u32 {
        unsafe { v8cxx__array_length(self) }
    }
}

impl Data for Array {}
impl Value for Array {}
impl Object for Array {}
