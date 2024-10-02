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

    #[inline(always)]
    pub fn iter<'a>(&'a self, context: &'a Local<Context>) -> FixedArrayIterator<'a> {
        FixedArrayIterator {
            fixed_array: self,
            context,
            index: 0,
        }
    }
}

impl Data for FixedArray {}

pub struct FixedArrayIterator<'a> {
    fixed_array: &'a FixedArray,
    context: &'a Local<Context>,
    index: isize,
}

impl<'a> Iterator for FixedArrayIterator<'a> {
    type Item = Local<data::Data>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.fixed_array.length() as isize {
            None
        } else {
            let index = self.index;

            self.index += 1;

            Some(self.fixed_array.get(self.context, index as i32))
        }
    }
}
