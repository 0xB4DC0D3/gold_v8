use crate::{
    context::Context,
    data::traits::Data,
    local::{Local, MaybeLocal},
    string::String,
    unbound_script::UnboundScript,
    value::Value,
};

extern "C" {
    fn v8cxx__script_compile(
        maybe_local_buf: *mut MaybeLocal<Script>,
        context: *const Local<Context>,
        source: *const Local<String>,
    );
    fn v8cxx__script_run(
        maybe_local_buf: *mut MaybeLocal<Value>,
        script: *mut Script,
        context: *const Local<Context>,
    );
    fn v8cxx__get_unbound_script(local_buf: *mut Local<UnboundScript>, this: *mut Script);
    fn v8cxx__script_get_resource_name(local_buf: *mut Local<Value>, script: *mut Script);
}

#[repr(C)]
pub struct Script([u8; 0]);

impl Script {
    #[inline(always)]
    pub fn compile(context: &Local<Context>, source: &Local<String>) -> MaybeLocal<Self> {
        let mut maybe_local_script = MaybeLocal::empty();

        unsafe { v8cxx__script_compile(&mut maybe_local_script, context, source) };

        maybe_local_script
    }

    #[inline(always)]
    pub fn run(&mut self, context: &Local<Context>) -> MaybeLocal<Value> {
        let mut maybe_local_value = MaybeLocal::<Value>::empty();

        unsafe {
            v8cxx__script_run(&mut maybe_local_value, self, context);
        }

        maybe_local_value
    }

    #[inline(always)]
    pub fn get_unbound_script(&mut self) -> Local<UnboundScript> {
        let mut local_unbound_script = Local::empty();

        unsafe { v8cxx__get_unbound_script(&mut local_unbound_script, self) };

        local_unbound_script
    }

    #[inline(always)]
    pub fn get_resource_name(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__script_get_resource_name(&mut local_value, self) };

        local_value
    }
}

impl Data for Script {}
