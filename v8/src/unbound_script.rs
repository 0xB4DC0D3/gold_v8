use crate::{data::traits::Data, local::Local, script::Script, value::Value};

extern "C" {
    fn v8cxx__unbound_script_bind_to_current_context(
        local_buf: *mut Local<Script>,
        this: *mut UnboundScript,
    );
    fn v8cxx__unbound_script_get_id(this: *const UnboundScript) -> i32;
    fn v8cxx__unbound_script_get_script_name(
        local_buf: *mut Local<Value>,
        this: *mut UnboundScript,
    );
    fn v8cxx__unbound_script_get_source_url(local_buf: *mut Local<Value>, this: *mut UnboundScript);
    fn v8cxx__unbound_script_get_source_mapping_url(
        local_buf: *mut Local<Value>,
        this: *mut UnboundScript,
    );
    fn v8cxx__unbound_script_get_line_number(this: *mut UnboundScript, code_pos: i32) -> i32;
    fn v8cxx__unbound_script_get_column_number(this: *mut UnboundScript, code_pos: i32) -> i32;
}

#[repr(C)]
pub struct UnboundScript([u8; 0]);

impl UnboundScript {
    #[inline(always)]
    pub fn bind_to_current_context(&mut self) -> Local<Script> {
        let mut local_script = Local::empty();

        unsafe { v8cxx__unbound_script_bind_to_current_context(&mut local_script, self) };

        local_script
    }

    #[inline(always)]
    pub fn get_id(&self) -> i32 {
        unsafe { v8cxx__unbound_script_get_id(self) }
    }

    #[inline(always)]
    pub fn get_script_name(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__unbound_script_get_script_name(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_source_url(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__unbound_script_get_source_url(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_source_mapping_url(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__unbound_script_get_source_mapping_url(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_line_number(&mut self, code_pos: i32) -> i32 {
        unsafe { v8cxx__unbound_script_get_line_number(self, code_pos) }
    }

    #[inline(always)]
    pub fn get_column_number(&mut self, code_pos: i32) -> i32 {
        unsafe { v8cxx__unbound_script_get_column_number(self, code_pos) }
    }
}

impl Data for UnboundScript {}
