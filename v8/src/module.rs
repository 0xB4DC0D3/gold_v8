use crate::data::traits::Data;

extern "C" {}

#[repr(C)]
pub struct Module([u8; 0]);

impl Module {}

impl Data for Module {}

#[repr(C)]
pub struct Location(i32, i32);

impl Location {
    #[inline(always)]
    pub const fn line_number(&self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub const fn column_number(&self) -> i32 {
        self.1
    }
}
