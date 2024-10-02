use crate::{context::Context, data::traits::Data, local::{Local, MaybeLocal}, string::String, value::Value};

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
}

#[repr(C)]
pub struct Script([u8; 0]);

impl Script {
    #[inline(always)]
    pub fn compile(context: &Local<Context>, source: &Local<String>) -> MaybeLocal<Self> {
        let mut maybe_local_script = MaybeLocal::<Self>::empty();

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
}

impl Data for Script {}
