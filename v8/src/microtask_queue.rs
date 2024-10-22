use std::{ffi::c_void, ptr::NonNull};

use crate::{c_support::ClosureToFunction, function::Function, isolate::Isolate, local::Local};

extern "C" {
    fn v8cxx__microtask_queue_new(
        isolate: *mut Isolate,
        policy: MicrotasksPolicy,
    ) -> *mut MicrotaskQueue;
    fn v8cxx__microtask_queue_drop(this: *mut MicrotaskQueue);
    fn v8cxx__microtask_queue_enqueue_microtask(
        this: *mut MicrotaskQueue,
        isolate: *mut Isolate,
        microtask: *const Local<Function>,
    );
    fn v8cxx__microtask_queue_enqueue_microtask_with_callback(
        this: *mut MicrotaskQueue,
        isolate: *mut Isolate,
        callback: ExternMicrotaskCallback,
        data: *mut c_void,
    );
    fn v8cxx__microtask_queue_add_microtasks_completed_callback(
        this: *mut MicrotaskQueue,
        callback: ExternMicrotasksCompletedCallbackWithData,
        data: *mut c_void,
    );
    fn v8cxx__microtask_queue_remove_microtasks_completed_callback(
        this: *mut MicrotaskQueue,
        callback: ExternMicrotasksCompletedCallbackWithData,
        data: *mut c_void,
    );
    fn v8cxx__microtask_queue_perform_checkpoint(this: *mut MicrotaskQueue, isolate: *mut Isolate);
    fn v8cxx__microtask_queue_is_running_microtasks(this: *const MicrotaskQueue) -> bool;
    fn v8cxx__microtask_queue_get_microtasks_scope_depth(this: *const MicrotaskQueue) -> isize;
}

type ExternMicrotaskCallback = extern "C" fn(data: *mut c_void);
type ExternMicrotasksCompletedCallbackWithData =
    extern "C" fn(isolate: *mut Isolate, data: *mut c_void);

#[repr(C)]
pub struct MicrotaskQueue([u8; 0]);

impl MicrotaskQueue {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, policy: MicrotasksPolicy) -> OwnedMicrotaskQueue {
        OwnedMicrotaskQueue::new(unsafe { v8cxx__microtask_queue_new(isolate, policy) })
    }

    #[inline(always)]
    pub fn enqueue_microtask(&mut self, isolate: &mut Isolate, microtask: &Local<Function>) {
        unsafe { v8cxx__microtask_queue_enqueue_microtask(self, isolate, microtask) };
    }

    #[inline(always)]
    pub fn enqueue_microtask_with_callback<T>(
        &mut self,
        isolate: &mut Isolate,
        callback: impl Fn(*mut c_void),
        data: &mut T,
    ) {
        unsafe {
            v8cxx__microtask_queue_enqueue_microtask_with_callback(
                self,
                isolate,
                callback.to_function(),
                data as *mut _ as *mut _,
            )
        };
    }

    #[inline(always)]
    pub fn add_microtasks_completed_callback<T>(
        &mut self,
        callback: impl Fn(&mut Isolate, *mut c_void),
        data: &mut T,
    ) {
        unsafe {
            v8cxx__microtask_queue_add_microtasks_completed_callback(
                self,
                callback.to_function(),
                data as *mut _ as *mut _,
            )
        };
    }

    #[inline(always)]
    pub fn remove_microtasks_completed_callback<T>(
        &mut self,
        callback: impl Fn(&mut Isolate, *mut c_void),
        data: &mut T,
    ) {
        unsafe {
            v8cxx__microtask_queue_remove_microtasks_completed_callback(
                self,
                callback.to_function(),
                data as *mut _ as *mut _,
            )
        };
    }

    #[inline(always)]
    pub fn perform_checkpoint(&mut self, isolate: &mut Isolate) {
        unsafe { v8cxx__microtask_queue_perform_checkpoint(self, isolate) };
    }

    #[inline(always)]
    pub fn is_running_microtasks(&self) -> bool {
        unsafe { v8cxx__microtask_queue_is_running_microtasks(self) }
    }

    #[inline(always)]
    pub fn get_microtasks_scope_depth(&self) -> isize {
        unsafe { v8cxx__microtask_queue_get_microtasks_scope_depth(self) }
    }
}

impl<T> ClosureToFunction<ExternMicrotaskCallback> for T
where
    T: Fn(*mut c_void),
{
    fn to_function(self) -> ExternMicrotaskCallback {
        extern "C" fn function<T>(data: *mut c_void)
        where
            T: Fn(*mut c_void),
        {
            T::get()(data)
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternMicrotasksCompletedCallbackWithData> for T
where
    T: Fn(&mut Isolate, *mut c_void),
{
    fn to_function(self) -> ExternMicrotasksCompletedCallbackWithData {
        extern "C" fn function<T>(isolate: *mut Isolate, data: *mut c_void)
        where
            T: Fn(&mut Isolate, *mut c_void),
        {
            T::get()(unsafe { isolate.as_mut().unwrap() }, data)
        }

        function::<T>
    }
}

pub struct OwnedMicrotaskQueue(NonNull<MicrotaskQueue>);

impl OwnedMicrotaskQueue {
    #[inline(always)]
    pub fn new(microtask_queue: *mut MicrotaskQueue) -> Self {
        Self(NonNull::new(microtask_queue).unwrap())
    }
}

impl Drop for OwnedMicrotaskQueue {
    fn drop(&mut self) {
        unsafe { v8cxx__microtask_queue_drop(self.0.as_ptr()) };
    }
}

#[repr(C)]
pub enum MicrotasksPolicy {
    Explicit,
    Scoped,
    Auto,
}
