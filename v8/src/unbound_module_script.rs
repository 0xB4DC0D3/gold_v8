use crate::{data::traits::Data, local::Local, value::Value};

extern "C" {
    fn v8cxx__unbound_module_script_get_source_url(
        local_buf: *mut Local<Value>,
        this: *mut UnboundModuleScript,
    );
    fn v8cxx__unbound_module_script_get_source_mapping_url(
        local_buf: *mut Local<Value>,
        this: *mut UnboundModuleScript,
    );
}

#[repr(C)]
pub struct UnboundModuleScript([u8; 0]);

impl UnboundModuleScript {
    #[inline(always)]
    pub fn get_source_url(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__unbound_module_script_get_source_url(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_source_mapping_url(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__unbound_module_script_get_source_mapping_url(&mut local_value, self) };

        local_value
    }
}

impl Data for UnboundModuleScript {}
