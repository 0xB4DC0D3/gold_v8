use crate::{
    context::Context,
    data::traits::Data,
    function::Function,
    local::{Local, MaybeLocal},
    object::traits::Object,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__promise_catch(
        maybe_local_buf: *mut MaybeLocal<Promise>,
        this: *mut Promise,
        context: *const Local<Context>,
        handler: *const Local<Function>,
    );
    fn v8cxx__promise_then(
        maybe_local_buf: *mut MaybeLocal<Promise>,
        this: *mut Promise,
        context: *const Local<Context>,
        handler: *const Local<Function>,
    );
    fn v8cxx__promise_has_handler(this: *const Promise) -> bool;
    fn v8cxx__promise_result(local_buf: *mut Local<value::Value>, this: *mut Promise);
    fn v8cxx__promise_state(this: *mut Promise) -> PromiseState;
    fn v8cxx__promise_mark_as_handled(this: *mut Promise);
    fn v8cxx__promise_mark_as_silent(this: *mut Promise);
}

#[repr(C)]
pub struct Promise([u8; 0]);

impl Promise {
    #[inline(always)]
    pub fn catch(
        &mut self,
        context: &Local<Context>,
        handler: &Local<Function>,
    ) -> MaybeLocal<Self> {
        let mut maybe_local_promise = MaybeLocal::empty();

        unsafe { v8cxx__promise_catch(&mut maybe_local_promise, self, context, handler) };

        maybe_local_promise
    }

    #[inline(always)]
    pub fn then(
        &mut self,
        context: &Local<Context>,
        handler: &Local<Function>,
    ) -> MaybeLocal<Self> {
        let mut maybe_local_promise = MaybeLocal::empty();

        unsafe { v8cxx__promise_then(&mut maybe_local_promise, self, context, handler) };

        maybe_local_promise
    }

    #[inline(always)]
    pub fn has_handler(&self) -> bool {
        unsafe { v8cxx__promise_has_handler(self) }
    }

    #[inline(always)]
    pub fn result(&mut self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__promise_result(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn state(&mut self) -> PromiseState {
        unsafe { v8cxx__promise_state(self) }
    }

    #[inline(always)]
    pub fn mark_as_handled(&mut self) {
        unsafe { v8cxx__promise_mark_as_handled(self) };
    }

    #[inline(always)]
    pub fn mark_as_silent(&mut self) {
        unsafe { v8cxx__promise_mark_as_silent(self) };
    }
}

impl Data for Promise {}
impl Value for Promise {}
impl Object for Promise {}

#[repr(C)]
pub enum PromiseState {
    Pending,
    Fulfilled,
    Rejected,
}

extern "C" {
    fn v8cxx__promise__resolver_new(
        maybe_local_buf: *mut MaybeLocal<PromiseResolver>,
        context: *const Local<Context>,
    );
    fn v8cxx__promise__resolver_get_promise(
        local_buf: *mut Local<Promise>,
        this: *mut PromiseResolver,
    );
    fn v8cxx__promise__resolver_resolve(
        this: *mut PromiseResolver,
        context: *const Local<Context>,
        value: *const Local<value::Value>,
    ) -> bool;
    fn v8cxx__promise__resolver_reject(
        this: *mut PromiseResolver,
        context: *const Local<Context>,
        value: *const Local<value::Value>,
    ) -> bool;
}

#[repr(C)]
pub struct PromiseResolver([u8; 0]);

impl PromiseResolver {
    #[inline(always)]
    pub fn new(context: &Local<Context>) -> MaybeLocal<Self> {
        let mut maybe_local_promise_resolver = MaybeLocal::empty();

        unsafe { v8cxx__promise__resolver_new(&mut maybe_local_promise_resolver, context) };

        maybe_local_promise_resolver
    }

    #[inline(always)]
    pub fn get_promise(&mut self) -> Local<Promise> {
        let mut local_promise = Local::empty();

        unsafe { v8cxx__promise__resolver_get_promise(&mut local_promise, self) };

        local_promise
    }

    #[inline(always)]
    pub fn resolve(&mut self, context: &Local<Context>, value: &Local<value::Value>) -> bool {
        unsafe { v8cxx__promise__resolver_resolve(self, context, value) }
    }

    #[inline(always)]
    pub fn reject(&mut self, context: &Local<Context>, value: &Local<value::Value>) -> bool {
        unsafe { v8cxx__promise__resolver_reject(self, context, value) }
    }
}

impl Data for PromiseResolver {}
impl Value for PromiseResolver {}
impl Object for PromiseResolver {}
