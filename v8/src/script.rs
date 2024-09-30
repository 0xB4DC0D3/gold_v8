use crate::{context::Context, local::Local, string::String, value::Value};

extern "C" {
    fn v8cxx__script_compile(
        local_buf: *mut Local<Script>,
        context: *const Local<Context>,
        source: *const Local<String>,
    );

    fn v8cxx__script_run(
        local_buf: *mut Local<Value>,
        script: *mut Script,
        context: *const Local<Context>,
    );
}

#[repr(C)]
pub struct Script([u8; 0]);

impl Script {
    #[inline(always)]
    pub fn compile(context: &Local<Context>, source: &Local<String>) -> Local<Self> {
        let mut local_script = Local::<Self>::empty();

        unsafe { v8cxx__script_compile(&mut local_script, context, source) };

        local_script
    }

    #[inline(always)]
    pub fn run(&mut self, context: &Local<Context>) -> Local<Value> {
        let mut local_value = Local::<Value>::empty();

        unsafe {
            v8cxx__script_run(&mut local_value, self, context);
        }

        local_value
    }
}
