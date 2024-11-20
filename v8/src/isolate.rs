use std::{
    ffi::c_void,
    mem::transmute,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    array_buffer::ArrayBufferAllocator,
    bindings,
    context::Context,
    data::Data,
    function::Function,
    local::{Local, MaybeLocal},
    microtask_queue::MicrotasksPolicy,
    value::Value,
};

extern "C" {
    fn v8cxx__isolate__createparams(
        array_buffer_allocator: *mut ArrayBufferAllocator,
    ) -> IsolateCreateParams;

    fn v8cxx__isolate_new(create_params: *const IsolateCreateParams) -> *mut Isolate;
    fn v8cxx__isolate_enter(this: *mut Isolate);
    fn v8cxx__isolate_exit(this: *mut Isolate);
    fn v8cxx__isolate_get_current() -> *mut Isolate;
    fn v8cxx__isolate_get_current_context(local_buf: *mut Local<Context>, this: *mut Isolate);
    fn v8cxx__isolate_allocate() -> *mut Isolate;
    fn v8cxx__isolate_initialize(isolate: *mut Isolate, params: *const IsolateCreateParams);
    fn v8cxx__isolate_is_current(isolate: *const Isolate) -> bool;
    fn v8cxx__isolate_clear_kept_objects(isolate: *mut Isolate);
    fn v8cxx__isolate_set_abort_on_uncaught_exception_callback(
        isolate: *mut Isolate,
        callback: ExternAbortOnUncaughtExceptionCallback,
    );
    fn v8cxx__isolate_set_host_import_module_dynamically_callback(
        isolate: *mut Isolate,
        callback: ExternHostImportModuleDynamicallyCallback,
    );
    fn v8cxx__isolate_set_host_create_shadow_realm_context_callback(
        isolate: *mut Isolate,
        callback: ExternHostCreateShadowRealmContextCallback,
    );
    fn v8cxx__isolate_set_prepare_stack_trace_callback(
        isolate: *mut Isolate,
        callback: ExternPrepareStackTraceCallback,
    );
    fn v8cxx__isolate_memory_pressure_notification(
        isolate: *mut Isolate,
        level: MemoryPressureLevel,
    );
    fn v8cxx__isolate_dispose(isolate: *mut Isolate);
    fn v8cxx__isolate_dump_and_reset_stats(isolate: *mut Isolate);
    fn v8cxx__isolate_discard_thread_specific_metadata(isolate: *mut Isolate);
    fn v8cxx__isolate_set_data(isolate: *mut Isolate, slot: u32, data: *mut c_void);
    fn v8cxx__isolate_get_data(isolate: *mut Isolate, slot: u32) -> *mut c_void;
    fn v8cxx__isolate_get_number_of_data_slots() -> u32;
    fn v8cxx__isolate_get_continuation_preserved_embedder_data(
        local_buf: *mut Local<Value>,
        isolate: *mut Isolate,
    );
    fn v8cxx__isolate_set_continuation_preserved_embedder_data(
        isolate: *mut Isolate,
        data: *const Local<Value>,
    );
    fn v8cxx__isolate_set_idle(isolate: *mut Isolate, is_idle: bool);
    fn v8cxx__isolate_in_context(isolate: *mut Isolate) -> bool;
    fn v8cxx__isolate_get_entered_or_microtask_context(
        local_buf: *mut Local<Context>,
        isolate: *mut Isolate,
    );
    fn v8cxx__isolate_get_incumbent_context(local_buf: *mut LocalContext, isolate: *mut Isolate);
    fn v8cxx__isolate_get_current_host_defined_options(
        maybe_local_buf: *mut MaybeLocal<Data>,
        isolate: *mut Isolate,
    );
    fn v8cxx__isolate_throw_error(
        local_buf: *mut Local<Value>,
        isolate: *mut Isolate,
        message: *const Local<String>,
    );
    fn v8cxx__isolate_throw_exception(
        local_buf: *mut Local<Value>,
        isolate: *mut Isolate,
        exception: *const Local<Value>,
    );
    fn v8cxx__isolate_terminate_execution(isolate: *mut Isolate);
    fn v8cxx__isolate_is_execution_terminating(isolate: *mut Isolate) -> bool;
    fn v8cxx__isolate_cancel_terminate_execution(isolate: *mut Isolate);
    fn v8cxx__isolate_request_interrupt(
        isolate: *mut Isolate,
        callback: ExternInterruptCallback,
        data: *mut c_void,
    );
    fn v8cxx__isolate_has_pending_background_tasks(isolate: *mut Isolate) -> bool;
    fn v8cxx__isolate_set_event_logger(isolate: *mut Isolate, callback: ExternLogEventCallback);
    fn v8cxx__isolate_add_before_call_entered_callback(
        isolate: *mut Isolate,
        callback: ExternBeforeCallEnteredCallback,
    );
    fn v8cxx__isolate_remove_before_call_entered_callback(
        isolate: *mut Isolate,
        callback: ExternBeforeCallEnteredCallback,
    );
    fn v8cxx__isolate_add_call_completed_callback(
        isolate: *mut Isolate,
        callback: ExternCallCompletedCallback,
    );
    fn v8cxx__isolate_remove_call_completed_callback(
        isolate: *mut Isolate,
        callback: ExternCallCompletedCallback,
    );
    fn v8cxx__isolate_set_promise_hook(isolate: *mut Isolate, hook: ExternPromiseHook);
    fn v8cxx__isolate_set_promise_reject_callback(
        isolate: *mut Isolate,
        callback: ExternPromiseRejectCallback,
    );
    fn v8cxx__isolate_perform_micrtask_checkpoint(isolate: *mut Isolate);
    fn v8cxx__isolate_enqueue_microtask(isolate: *mut Isolate, microtask: *const Local<Function>);
    fn v8cxx__isolate_enqueue_microtask_with_callback(
        isolate: *mut Isolate,
        callback: ExternMicrotaskCallback,
        data: *mut c_void,
    );
    fn v8cxx__isolate_set_microtasks_policy(isolate: *mut Isolate, policy: MicrotasksPolicy);
    fn v8cxx__isolate_get_microtasks_policy(isolate: *mut Isolate) -> MicrotasksPolicy;
    fn v8cxx__isolate_add_microtasks_completed_callback(
        isolate: *mut Isolate,
        callback: ExternMicrotasksCompletedCallbackWithData,
        data: *mut c_void,
    );
    fn v8cxx__isolate_remove_microtasks_completed_callback(
        isolate: *mut Isolate,
        callback: ExternMicrotasksCompletedCallbackWithData,
        data: *mut c_void,
    );
    fn v8cxx__isolate_set_stack_limit(isolate: *mut Isolate, stack_limit: usize);
    fn v8cxx__isolate_set_fatal_error_handler(isolate: *mut Isolate, callback: FatalErrorCallback);
    fn v8cxx__isolate_set_oom_error_handler(isolate: *mut Isolate, callback: OOMErrorCallback);
    fn v8cxx__isolate_install_conditional_features(
        isolate: *mut Isolate,
        context: *const Local<Context>,
    );
    fn v8cxx__isolate_is_dead(isolate: *mut Isolate) -> bool;
    fn v8cxx__isolate_set_capture_stack_trace_for_uncaught_exceptions(
        isolate: *mut Isolate,
        capture: bool,
        frame_limit: i32,
        options: StackTraceOptions,
    );
    fn v8cxx__isolate_is_in_use(isolate: *mut Isolate) -> bool;
}

#[repr(C)]
pub struct IsolateCreateParams([u8; bindings::v8cxx__sizeof_isolate__createparams]);

impl IsolateCreateParams {
    #[inline(always)]
    pub fn new(array_buffer_allocator: &mut ArrayBufferAllocator) -> Self {
        unsafe { v8cxx__isolate__createparams(array_buffer_allocator) }
    }
}

#[repr(C)]
pub struct Isolate([u8; 0]);

impl Isolate {
    #[inline(always)]
    pub fn new(create_params: &IsolateCreateParams) -> OwnedIsolate {
        unsafe { OwnedIsolate::new(v8cxx__isolate_new(create_params)) }
    }

    #[inline(always)]
    pub(super) fn enter(&mut self) {
        unsafe { v8cxx__isolate_enter(self) };
    }

    #[inline(always)]
    pub(super) fn exit(&mut self) {
        unsafe { v8cxx__isolate_exit(self) };
    }

    #[inline(always)]
    pub fn get_current<'a>() -> Option<&'a mut Self> {
        unsafe { v8cxx__isolate_get_current().as_mut() }
    }

    #[inline(always)]
    pub fn get_current_context(&mut self) -> Local<Context> {
        let mut local_context = Local::<Context>::empty();

        unsafe { v8cxx__isolate_get_current_context(&mut local_context, self) };

        local_context
    }

    #[inline(always)]
    pub fn allocate() -> OwnedIsolate {
        unsafe { OwnedIsolate::new(v8cxx__isolate_allocate()) }
    }

    #[inline(always)]
    pub fn initialize(isolate: &mut Isolate, params: &IsolateCreateParams) {
        unsafe { v8cxx__isolate_initialize(isolate, params) };
    }

    #[inline(always)]
    pub fn is_current(&self) -> bool {
        unsafe { v8cxx__isolate_is_current(self) }
    }

    #[inline(always)]
    pub fn clear_kept_objects(&mut self) {
        unsafe { v8cxx__isolate_clear_kept_objects(self) };
    }

    #[inline(always)]
    pub fn set_abort_on_uncaught_exception_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_abort_on_uncaught_exception_callback(self, callback) };
    }

    #[inline(always)]
    pub fn set_host_import_module_dynamically_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_host_import_module_dynamically_callback(self, callback) };
    }

    #[inline(always)]
    pub fn set_prepare_stack_trace_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_prepare_stack_trace_callback(self, callback) };
    }

    #[inline(always)]
    pub fn memory_pressure_notification(&mut self, level: MemoryPressureLevel) {
        unsafe { v8cxx__isolate_memory_pressure_notification(self, level) };
    }

    #[inline(always)]
    pub fn dispose(&mut self) {
        unsafe { v8cxx__isolate_dispose(self) };
    }

    #[inline(always)]
    pub fn dump_and_reset_stats(&mut self) {
        unsafe { v8cxx__isolate_dump_and_reset_stats(self) };
    }

    #[inline(always)]
    pub fn discard_thread_specific_metadata(&mut self) {
        unsafe { v8cxx__isolate_discard_thread_specific_metadata(self) };
    }

    #[inline(always)]
    pub fn set_data<T>(&mut self, slot: u32, data: &mut T) {
        unsafe { v8cxx__isolate_set_data(self, slot, data as *mut _ as *mut _) };
    }

    #[inline(always)]
    pub fn get_data(&mut self, slot: u32) -> Option<&mut T> {
        unsafe { (v8cxx__isolate_get_data(self, slot) as *mut T).as_mut() }
    }

    #[inline(always)]
    pub fn get_number_of_data_slots() -> u32 {
        unsafe { v8cxx__isolate_get_number_of_data_slots() }
    }

    #[inline(always)]
    pub fn get_continuation_preserved_embedder_data(&mut self) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe {
            v8cxx__isolate_get_continuation_preserved_embedder_data(local_value.cast_mut(), self)
        };

        local_value
    }

    #[inline(always)]
    pub fn set_continuation_preserved_embedder_data(&mut self, data: &Local<Value>) {
        unsafe { v8cxx__isolate_set_continuation_preserved_embedder_data(self, data) };
    }

    #[inline(always)]
    pub fn set_idle(&mut self, idle: bool) {
        unsafe { v8cxx__isolate_set_idle(self, idle) };
    }

    #[inline(always)]
    pub fn in_context(&mut self) -> bool {
        unsafe { v8cxx__isolate_in_context(self) }
    }

    #[inline(always)]
    pub fn get_entered_or_microtask_context(&mut self) -> Local<Context> {
        let mut local_context = Local::empty();

        unsafe {
            v8cxx__isolate_get_entered_or_microtask_context(local_context.cast_mut(), isolate)
        };

        local_context
    }

    #[inline(always)]
    pub fn get_incumbent_context(&mut self) -> Local<Context> {
        let mut local_context = Local::empty();

        unsafe { v8cxx__isolate_get_incumbent_context(local_context.cast_mut(), self) };

        local_context
    }

    #[inline(always)]
    pub fn get_current_host_defined_options(&mut self) -> MaybeLocal<Data> {
        let mut maybe_local_data = MaybeLocal::empty();

        unsafe {
            v8cxx__isolate_get_current_host_defined_options(maybe_local_data.cast_mut(), self)
        };

        maybe_local_data
    }

    #[inline(always)]
    pub fn throw_error(&mut self, message: &Local<String>) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__isolate_throw_error(local_value.cast_mut(), self, message) };

        local_value
    }

    #[inline(always)]
    pub fn throw_exception(&mut self, exception: &Local<Value>) -> Local<Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__isolate_throw_exception(local_value.cast_mut(), self, exception) };

        local_value
    }

    #[inline(always)]
    pub fn terminate_execution(&mut self) {
        unsafe { v8cxx__isolate_terminate_execution(self) };
    }

    #[inline(always)]
    pub fn is_execution_terminating(&mut self) -> bool {
        unsafe { v8cxx__isolate_is_execution_terminating(self) }
    }

    #[inline(always)]
    pub fn cancel_terminate_execution(&mut self) {
        unsafe { v8cxx__isolate_cancel_terminate_execution(self) };
    }

    #[inline(always)]
    pub fn request_interrupt<T>(&mut self, callback: impl Fn(), data: &mut T) {
        unsafe { v8cxx__isolate_request_interrupt(self, callback, data as *mut _ as *mut _) };
    }

    #[inline(always)]
    pub fn has_pending_background_tasks(&mut self) -> bool {
        unsafe { v8cxx__isolate_has_pending_background_tasks(self) }
    }

    #[inline(always)]
    pub fn set_event_logger(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_event_logger(self, callback) };
    }

    #[inline(always)]
    pub fn add_before_call_entered_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_add_before_call_entered_callback(self, callback) };
    }

    #[inline(always)]
    pub fn remove_before_call_entered_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_remove_before_call_entered_callback(self, callback) };
    }

    #[inline(always)]
    pub fn add_call_completed_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_add_call_completed_callback(self, callback) };
    }

    #[inline(always)]
    pub fn remove_call_completed_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_remove_call_completed_callback(self, callback) };
    }

    #[inline(always)]
    pub fn set_promise_hook(&mut self, hook: impl Fn()) {
        unsafe { v8cxx__isolate_set_promise_hook(self, hook) };
    }

    #[inline(always)]
    pub fn set_promise_reject_callback(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_promise_reject_callback(self, callback) };
    }

    #[inline(always)]
    pub fn perform_microtask_checkpoint(&mut self) {
        unsafe { v8cxx__isolate_perform_micrtask_checkpoint(self) };
    }

    #[inline(always)]
    pub fn enqueue_microtask(&mut self, microtask: &Local<Function>) {
        unsafe { v8cxx__isolate_enqueue_microtask(self, microtask) };
    }

    #[inline(always)]
    pub fn enqueue_microtask_with_callback<T>(&mut self, callback: impl Fn(), data: &mut T) {
        unsafe { v8cxx__isolate_enqueue_microtask_with_callback(self, callback, transmute(data)) };
    }

    #[inline(always)]
    pub fn set_microtasks_policy(&mut self, policy: MicrotasksPolicy) {
        unsafe { v8cxx__isolate_set_microtasks_policy(self, policy) };
    }

    #[inline(always)]
    pub fn get_microtasks_policy(&mut self) -> MicrotasksPolicy {
        unsafe { v8cxx__isolate_get_microtasks_policy(self) }
    }

    #[inline(always)]
    pub fn add_microtasks_completed_callback<T>(&mut self, callback: impl Fn(), data: &mut T) {
        unsafe {
            v8cxx__isolate_add_microtasks_completed_callback(self, callback, transmute(data))
        };
    }

    #[inline(always)]
    pub fn remove_microtasks_completed_callback<T>(&mut self, callback: impl Fn(), data: &mut T) {
        unsafe {
            v8cxx__isolate_remove_microtasks_completed_callback(self, callback, transmute(data))
        };
    }

    #[inline(always)]
    pub fn set_stack_limit(&mut self, limit: usize) {
        unsafe { v8cxx__isolate_set_stack_limit(self, limit) };
    }

    #[inline(always)]
    pub fn set_fatal_error_handler(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_fatal_error_handler(self, callback) };
    }

    #[inline(always)]
    pub fn set_oom_error_handler(&mut self, callback: impl Fn()) {
        unsafe { v8cxx__isolate_set_oom_error_handler(self, callback) };
    }

    #[inline(always)]
    pub fn install_conditional_features(&mut self, context: &Local<Context>) {
        unsafe { v8cxx__isolate_install_conditional_features(self, context) };
    }

    #[inline(always)]
    pub fn is_dead(&mut self) -> bool {
        unsafe { v8cxx__isolate_is_dead(self) }
    }

    #[inline(always)]
    pub fn set_capture_stack_trace_for_uncaught_exceptions(
        &mut self,
        capture: bool,
        limit: i32,
        options: StackTraceOptions,
    ) {
        unsafe {
            v8cxx__isolate_set_capture_stack_trace_for_uncaught_exceptions(
                self, capture, limit, options,
            )
        };
    }

    #[inline(always)]
    pub fn is_in_use(&mut self) -> bool {
        unsafe { v8cxx__isolate_is_in_use(self) }
    }
}

pub struct OwnedIsolate(NonNull<Isolate>);

impl OwnedIsolate {
    pub fn new(isolate: *mut Isolate) -> Self {
        let mut owned_isolate = Self(NonNull::new(isolate).unwrap());

        owned_isolate.enter();
        owned_isolate
    }
}

impl Deref for OwnedIsolate {
    type Target = Isolate;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnedIsolate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl AsMut<Isolate> for OwnedIsolate {
    fn as_mut(&mut self) -> &mut Isolate {
        self
    }
}

impl Drop for OwnedIsolate {
    fn drop(&mut self) {
        self.exit();
    }
}
