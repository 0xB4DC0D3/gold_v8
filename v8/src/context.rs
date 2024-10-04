use crate::{
    data::traits::Data, isolate::Isolate, local::Local, microtask_queue::MicrotaskQueue,
    object::Object, object_template::ObjectTemplate, value::Value,
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
}

#[repr(C)]
pub struct Context([u8; 0]);

impl Context {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, options: Option<ContextOptions>) -> Local<Self> {
        let mut local_context = Local::<Self>::empty();

        unsafe {
            v8cxx__context_new(
                &mut local_context,
                isolate,
                match &options {
                    Some(options) => &options.global_template,
                    None => std::ptr::null::<Local<ObjectTemplate>>(),
                },
                match &options {
                    Some(options) => &options.global_object,
                    None => std::ptr::null::<Local<Value>>(),
                },
                // TODO: MicrotaskQueue
                std::ptr::null_mut(),
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
}

impl Data for Context {}

// TODO: implement MicrotaskQueue
#[allow(dead_code)]
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
