use crate::{
    data::traits::Data,
    isolate::Isolate,
    local::Local,
    primitive::{self, Primitive},
};

extern "C" {
    fn v8cxx__primitive_array_new(
        local_buf: *mut Local<PrimitiveArray>,
        isolate: *mut Isolate,
        length: i32,
    );
    fn v8cxx__primitive_array_length(primitive_array: *const PrimitiveArray) -> i32;
    fn v8cxx__primitive_array_set(
        primitive_array: *mut PrimitiveArray,
        isolate: *mut Isolate,
        index: i32,
        item: *const Local<Primitive>,
    );
    fn v8cxx__primitive_array_get(
        local_buf: *mut Local<Primitive>,
        primitive_array: *mut PrimitiveArray,
        isolate: *mut Isolate,
        index: i32,
    );
}

#[repr(C)]
pub struct PrimitiveArray([u8; 0]);

impl PrimitiveArray {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, length: i32) -> Local<PrimitiveArray> {
        let mut local_primitive_array = Local::empty();

        unsafe { v8cxx__primitive_array_new(&mut local_primitive_array, isolate, length) };

        local_primitive_array
    }

    #[inline(always)]
    pub fn length(&self) -> i32 {
        unsafe { v8cxx__primitive_array_length(self) }
    }

    #[inline(always)]
    pub fn set(
        &mut self,
        isolate: &mut Isolate,
        index: i32,
        item: &Local<impl primitive::traits::Primitive>,
    ) {
        unsafe { v8cxx__primitive_array_set(self, isolate, index, item.cast_ref()) };
    }

    #[inline(always)]
    pub fn get(&mut self, isolate: &mut Isolate, index: i32) -> Local<Primitive> {
        let mut local_primitive = Local::empty();

        unsafe { v8cxx__primitive_array_get(&mut local_primitive, self, isolate, index) };

        local_primitive
    }
}

impl Data for PrimitiveArray {}
