use crate::{
    context::Context,
    data::{self, traits::Data},
    local::Local,
};

extern "C" {
    fn v8cxx__fixed_array_length(this: *const FixedArray) -> i32;
    fn v8cxx__fixed_array_get(
        local_buf: *mut Local<data::Data>,
        this: *const FixedArray,
        context: *const Local<Context>,
        i: i32,
    );
}

#[repr(C)]
pub struct FixedArray([u8; 0]);

impl FixedArray {
    #[inline(always)]
    pub fn length(&self) -> i32 {
        unsafe { v8cxx__fixed_array_length(self) }
    }

    #[inline(always)]
    pub fn get(&self, context: &Local<Context>, i: i32) -> Local<data::Data> {
        let mut local_data = Local::<data::Data>::empty();

        unsafe {
            v8cxx__fixed_array_get(&mut local_data, self, context, i);
        }

        local_data
    }
}

impl Data for FixedArray {}
