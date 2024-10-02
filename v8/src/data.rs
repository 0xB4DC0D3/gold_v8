#[repr(C)]
pub struct Data([u8; 0]);

extern "C" {
    fn v8cxx__data_is_context(data: *const Data) -> bool;
    fn v8cxx__data_is_fixed_array(data: *const Data) -> bool;
    fn v8cxx__data_is_function_template(data: *const Data) -> bool;
    fn v8cxx__data_is_module(data: *const Data) -> bool;
    fn v8cxx__data_is_object_template(data: *const Data) -> bool;
    fn v8cxx__data_is_private(data: *const Data) -> bool;
    fn v8cxx__data_is_value(data: *const Data) -> bool;
}

pub mod traits {
    use super::{
        v8cxx__data_is_context, v8cxx__data_is_fixed_array, v8cxx__data_is_function_template,
        v8cxx__data_is_module, v8cxx__data_is_object_template, v8cxx__data_is_private,
        v8cxx__data_is_value,
    };

    pub trait Data: Sized {
        fn is_context(&self) -> bool {
            unsafe { v8cxx__data_is_context(self as *const _ as *const _) }
        }

        fn is_fixed_array(&self) -> bool {
            unsafe { v8cxx__data_is_fixed_array(self as *const _ as *const _) }
        }

        fn is_function_template(&self) -> bool {
            unsafe { v8cxx__data_is_function_template(self as *const _ as *const _) }
        }

        fn is_module(&self) -> bool {
            unsafe { v8cxx__data_is_module(self as *const _ as *const _) }
        }

        fn is_object_template(&self) -> bool {
            unsafe { v8cxx__data_is_object_template(self as *const _ as *const _) }
        }

        fn is_private(&self) -> bool {
            unsafe { v8cxx__data_is_private(self as *const _ as *const _) }
        }

        fn is_value(&self) -> bool {
            unsafe { v8cxx__data_is_value(self as *const _ as *const _) }
        }
    }
}

impl traits::Data for Data {}
