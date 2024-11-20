use std::ffi::c_void;

use crate::{
    c_support::ClosureToFunction, data::traits::Data, function::Function, isolate::Isolate,
    local::Local, microtask_queue::MicrotaskQueue, object::Object, object_template::ObjectTemplate,
    string::String, value::Value,
};

extern "C" {
    fn v8cxx__context_new(
        local_buf: *mut Local<Context>,
        isolate: *mut Isolate,
        global_template: *const Local<ObjectTemplate>,
        global_object: *const Local<Value>,
        microtask_queue: *mut MicrotaskQueue,
    );
    fn v8cxx__context_enter(this: *mut Context);
    fn v8cxx__context_exit(this: *mut Context);
    fn v8cxx__context_global(local_buf: *mut Local<Object>, this: *mut Context);
    fn v8cxx__context_detach_global(this: *mut Context);
    fn v8cxx__context_get_isolate(this: *mut Context) -> *mut Isolate;
    fn v8cxx__context_get_microtask_queue(this: *mut Context) -> *mut MicrotaskQueue;
    fn v8cxx__context_set_microtask_queue(this: *mut Context, microtask_queue: *mut MicrotaskQueue);
    fn v8cxx__context_set_security_token(this: *mut Context, token: *const Local<Value>);
    fn v8cxx__context_use_default_security_token(this: *mut Context);
    fn v8cxx__context_get_number_of_embedder_data_fields(this: *mut Context) -> u32;
    fn v8cxx__context_get_embedder_data(
        local_buf: *mut Local<Value>,
        this: *mut Context,
        index: isize,
    );
    fn v8cxx__context_get_extras_binding_object(local_buf: *mut Local<Object>, this: *mut Context);
    fn v8cxx__context_set_embedder_data(
        this: *mut Context,
        index: isize,
        value: *const Local<Value>,
    );
    fn v8cxx__context_get_aligned_pointer_from_embedder_data(
        this: *mut Context,
        isolate: *mut Isolate,
        index: isize,
    ) -> *mut c_void;
    fn v8cxx__context_set_aligned_pointer_in_embedder_data(
        this: *mut Context,
        index: isize,
        value: *mut c_void,
    );
    fn v8cxx__context_allow_code_generation_from_strings(this: *mut Context, allow: bool);
    fn v8cxx__context_is_code_generation_from_string_allowed(this: *const Context) -> bool;
    fn v8cxx__context_set_error_message_for_code_generation_from_strings(
        this: *mut Context,
        message: *const Local<String>,
    );
    fn v8cxx__context_set_error_message_for_wasm_code_generation(
        this: *mut Context,
        message: *const Local<String>,
    );
    fn v8cxx__context_set_abort_script_execution(
        this: *mut Context,
        callback: ExternAbortScriptExecutionCallback,
    );
    fn v8cxx__context_set_promise_hooks(
        this: *mut Context,
        init_hook: *const Local<Function>,
        before_hook: *const Local<Function>,
        after_hook: *const Local<Function>,
        resolve_hook: *const Local<Function>,
    );
    fn v8cxx__context_has_template_literal_object(
        this: *mut Context,
        object: *const Local<Value>,
    ) -> bool;
}

#[repr(C)]
pub struct Context([u8; 0]);

impl Context {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, mut options: Option<ContextOptions>) -> Local<Self> {
        let mut local_context = Local::empty();

        unsafe {
            v8cxx__context_new(
                &mut local_context,
                isolate,
                match options.as_ref() {
                    Some(options) => options.global_template.cast_ref(),
                    None => std::ptr::null::<Local<ObjectTemplate>>(),
                },
                match options.as_ref() {
                    Some(options) => options.global_object.cast_ref(),
                    None => std::ptr::null::<Local<Value>>(),
                },
                match options.as_mut() {
                    Some(options) => options.microtask_queue.as_mut().unwrap(),
                    None => std::ptr::null_mut(),
                },
            );
        }

        local_context
    }

    #[inline(always)]
    pub(super) fn enter(&mut self) {
        unsafe { v8cxx__context_enter(self) };
    }

    #[inline(always)]
    pub(super) fn exit(&mut self) {
        unsafe { v8cxx__context_exit(self) };
    }

    #[inline(always)]
    pub fn global(&mut self) -> Local<Object> {
        let mut local_object = Local::<Object>::empty();

        unsafe {
            v8cxx__context_global(&mut local_object, self);
        }

        local_object
    }

    #[inline(always)]
    pub fn detach_global(&mut self) {
        unsafe { v8cxx__context_detach_global(self) };
    }

    #[inline(always)]
    pub fn get_isolate(&mut self) -> Option<&mut Isolate> {
        unsafe { v8cxx__context_get_isolate(self).as_mut() }
    }

    #[inline(always)]
    pub fn get_microtask_queue(&mut self) -> Option<&mut MicrotaskQueue> {
        unsafe { v8cxx__context_get_microtask_queue(self).as_mut() }
    }

    #[inline(always)]
    pub fn set_microtask_queue(&mut self, microtask_queue: &mut MicrotaskQueue) {
        unsafe { v8cxx__context_set_microtask_queue(self, microtask_queue) };
    }

    #[inline(always)]
    pub fn set_security_token(&mut self, token: &Local<Value>) {
        unsafe { v8cxx__context_set_security_token(self, token) };
    }

    #[inline(always)]
    pub fn use_default_security_token(&mut self) {
        unsafe { v8cxx__context_use_default_security_token(self) };
    }

    #[inline(always)]
    pub fn get_number_of_embedder_data_fields(&mut self) -> u32 {
        unsafe { v8cxx__context_get_number_of_embedder_data_fields(self) }
    }

    #[inline(always)]
    pub fn get_embedder_data(&mut self, index: isize) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__context_get_embedder_data(&mut local_value, self, index) };

        local_value
    }

    #[inline(always)]
    pub fn get_extras_binding_object(&mut self) -> Local<Object> {
        let mut local_object = Local::empty();

        unsafe { v8cxx__context_get_extras_binding_object(&mut local_object, self) };

        local_object
    }

    #[inline(always)]
    pub fn set_embedder_data(&mut self, index: isize, value: &Local<Value>) {
        unsafe { v8cxx__context_set_embedder_data(self, index, value) };
    }

    #[inline(always)]
    pub fn get_aligned_pointer_from_embedder_data<T>(
        &mut self,
        isolate: &mut Isolate,
        index: isize,
    ) -> &mut T {
        unsafe {
            (v8cxx__context_get_aligned_pointer_from_embedder_data(self, isolate, index) as *mut T)
                .as_mut()
                .unwrap()
        }
    }

    #[inline(always)]
    pub fn set_aligned_pointer_in_embedder_data<T>(&mut self, index: isize, value: &mut T) {
        unsafe {
            v8cxx__context_set_aligned_pointer_in_embedder_data(
                self,
                index,
                value as *mut _ as *mut _,
            )
        };
    }

    #[inline(always)]
    pub fn allow_code_generation_from_strings(&mut self, allow: bool) {
        unsafe { v8cxx__context_allow_code_generation_from_strings(self, allow) };
    }

    #[inline(always)]
    pub fn is_code_generation_from_string_allowed(&self) -> bool {
        unsafe { v8cxx__context_is_code_generation_from_string_allowed(self) }
    }

    #[inline(always)]
    pub fn set_error_message_for_code_generation_from_strings(&mut self, message: &Local<String>) {
        unsafe { v8cxx__context_set_error_message_for_code_generation_from_strings(self, message) };
    }

    #[inline(always)]
    pub fn set_error_message_for_wasm_code_generation(&mut self, message: &Local<String>) {
        unsafe { v8cxx__context_set_error_message_for_wasm_code_generation(self, message) };
    }

    #[inline(always)]
    pub fn set_abort_script_execution(&mut self, callback: impl Fn(&mut Isolate, Local<Context>)) {
        unsafe { v8cxx__context_set_abort_script_execution(self, callback.to_function()) };
    }

    #[inline(always)]
    pub fn set_promise_hooks(
        &mut self,
        init_hook: &Local<Function>,
        before_hook: &Local<Function>,
        after_hook: &Local<Function>,
        resolve_hook: &Local<Function>,
    ) {
        unsafe {
            v8cxx__context_set_promise_hooks(self, init_hook, before_hook, after_hook, resolve_hook)
        };
    }

    #[inline(always)]
    pub fn has_template_literal_object(&mut self, object: &Local<Value>) -> bool {
        unsafe { v8cxx__context_has_template_literal_object(self, object) }
    }
}

impl Data for Context {}

type ExternAbortScriptExecutionCallback = extern "C" fn(*mut Isolate, Local<Context>);

impl<T> ClosureToFunction<ExternAbortScriptExecutionCallback> for T
where
    T: Fn(&mut Isolate, Local<Context>),
{
    fn to_function(self) -> ExternAbortScriptExecutionCallback {
        extern "C" fn function<T>(isolate: *mut Isolate, context: Local<Context>)
        where
            T: Fn(&mut Isolate, Local<Context>),
        {
            T::get()(unsafe { isolate.as_mut().unwrap() }, context)
        }

        function::<T>
    }
}

pub struct ContextOptions {
    global_template: Local<ObjectTemplate>,
    global_object: Local<Value>,
    microtask_queue: Option<MicrotaskQueue>,
}

impl ContextOptions {
    pub fn new(global_template: Local<ObjectTemplate>, global_object: Local<Value>) -> Self {
        Self {
            global_template,
            global_object,
            microtask_queue: None,
        }
    }
}
