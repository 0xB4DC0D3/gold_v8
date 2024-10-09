use std::mem::MaybeUninit;

use crate::{bindings, data::Data, local::Local, value::Value};

extern "C" {
    fn v8cxx__script_origin_new(
        buf: *mut ScriptOrigin,
        resource_name: *const Local<Value>,
        resource_line_offset: i32,
        resource_column_offset: i32,
        resource_is_shared_cross_origin: bool,
        script_id: i32,
        source_map_url: *const Local<Value>,
        resource_is_opaque: bool,
        is_wasm: bool,
        is_module: bool,
        host_defined_options: *const Local<Data>,
    );
    fn v8cxx__script_origin_resource_name(local_buf: *mut Local<Value>, this: *const ScriptOrigin);
    fn v8cxx__script_origin_line_offset(this: *const ScriptOrigin) -> i32;
    fn v8cxx__script_origin_column_offset(this: *const ScriptOrigin) -> i32;
    fn v8cxx__script_origin_script_id(this: *const ScriptOrigin) -> i32;
    fn v8cxx__script_origin_source_map_url(local_buf: *mut Local<Value>, this: *const ScriptOrigin);
    fn v8cxx__script_origin_get_host_defined_options(
        local_buf: *mut Local<Data>,
        this: *const ScriptOrigin,
    );
    fn v8cxx__script_origin_is_module(this: *const ScriptOrigin) -> bool;
    fn v8cxx__script_origin_is_opaque(this: *const ScriptOrigin) -> bool;
    fn v8cxx__script_origin_is_shared_cross_origin(this: *const ScriptOrigin) -> bool;
    fn v8cxx__script_origin_is_wasm(this: *const ScriptOrigin) -> bool;
}

#[repr(C)]
pub struct ScriptOrigin([u8; bindings::v8cxx__sizeof_scriptorigin]);

impl ScriptOrigin {
    #[inline(always)]
    pub fn new(
        resource_name: &Local<Value>,
        resource_line_offset: i32,
        resource_column_offset: i32,
        resource_is_shared_cross_origin: bool,
        script_id: i32,
        source_map_url: &Local<Value>,
        resource_is_opaque: bool,
        is_wasm: bool,
        is_module: bool,
        host_defined_options: &Local<Data>,
    ) -> Self {
        unsafe {
            let mut script_origin = MaybeUninit::zeroed();

            v8cxx__script_origin_new(
                script_origin.as_mut_ptr(),
                resource_name,
                resource_line_offset,
                resource_column_offset,
                resource_is_shared_cross_origin,
                script_id,
                source_map_url,
                resource_is_opaque,
                is_wasm,
                is_module,
                host_defined_options,
            );

            script_origin.assume_init()
        }
    }

    #[inline(always)]
    pub fn new_default(resource_name: &Local<Value>, is_module: bool) -> Self {
        Self::new(
            resource_name,
            0,
            0,
            false,
            -1,
            &Local::empty(),
            false,
            false,
            is_module,
            &Local::empty(),
        )
    }

    #[inline(always)]
    pub fn resource_name(&self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__script_origin_resource_name(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn line_offset(&self) -> i32 {
        unsafe { v8cxx__script_origin_line_offset(self) }
    }

    #[inline(always)]
    pub fn column_offset(&self) -> i32 {
        unsafe { v8cxx__script_origin_column_offset(self) }
    }

    #[inline(always)]
    pub fn script_id(&self) -> i32 {
        unsafe { v8cxx__script_origin_script_id(self) }
    }

    #[inline(always)]
    pub fn source_map_url(&self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__script_origin_source_map_url(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_host_defined_options(&self) -> Local<Data> {
        let mut local_data = Local::empty();

        unsafe { v8cxx__script_origin_get_host_defined_options(&mut local_data, self) };

        local_data
    }

    #[inline(always)]
    pub fn is_module(&self) -> bool {
        unsafe { v8cxx__script_origin_is_module(self) }
    }

    #[inline(always)]
    pub fn is_opaque(&self) -> bool {
        unsafe { v8cxx__script_origin_is_opaque(self) }
    }

    #[inline(always)]
    pub fn is_shared_cross_origin(&self) -> bool {
        unsafe { v8cxx__script_origin_is_shared_cross_origin(self) }
    }

    #[inline(always)]
    pub fn is_wasm(&self) -> bool {
        unsafe { v8cxx__script_origin_is_wasm(self) }
    }
}
