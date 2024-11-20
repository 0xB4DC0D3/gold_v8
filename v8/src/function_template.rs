use crate::{
    c_support::ClosureToFunction,
    context::Context,
    data::traits::Data,
    function::{ExternFunctionCallback, Function},
    function_callback_info::FunctionCallbackInfo,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    object_template::ObjectTemplate,
    signature::Signature,
    string::String,
    template::traits::Template,
    value::Value,
};

extern "C" {
    fn v8cxx__function_template_new(
        local_buf: *mut Local<FunctionTemplate>,
        isolate: *mut Isolate,
        fn_callback: ExternFunctionCallback,
        data: *const Local<Value>,
        signature: *const Local<Signature>,
        length: i32,
        behavior: ConstructorBehavior,
        side_effect_type: SideEffectType,
    );
    fn v8cxx__function_template_get_function(
        maybe_local_buf: *mut MaybeLocal<Function>,
        this: *mut FunctionTemplate,
        context: *const Local<Context>,
    );
    fn v8cxx__function_template_instance_template(
        local_buf: *mut Local<ObjectTemplate>,
        this: *mut FunctionTemplate,
    );
    fn v8cxx__function_template_inherit(
        this: *mut FunctionTemplate,
        parent: *const Local<FunctionTemplate>,
    );
    fn v8cxx__function_template_prototype_template(
        local_buf: *mut Local<ObjectTemplate>,
        this: *mut FunctionTemplate,
    );
    fn v8cxx__function_template_set_prototype_provider_template(
        this: *mut FunctionTemplate,
        prototype_provider: *const Local<FunctionTemplate>,
    );
    fn v8cxx__function_template_set_class_name(
        this: *mut FunctionTemplate,
        name: *const Local<String>,
    );
    fn v8cxx__function_template_set_interface_name(
        this: *mut FunctionTemplate,
        name: *const Local<String>,
    );
    fn v8cxx__function_template_read_only_prototype(this: *mut FunctionTemplate);
    fn v8cxx__function_template_remove_prototype(this: *mut FunctionTemplate);
    fn v8cxx__function_template_has_instance(
        this: *mut FunctionTemplate,
        object: *const Local<Value>,
    ) -> bool;
    fn v8cxx__function_template_set_exception_context(
        this: *mut FunctionTemplate,
        context: ExceptionContext,
    );
    fn v8cxx__function_template_set_accept_any_receiver(this: *mut FunctionTemplate, accept: bool);
    fn v8cxx__function_template_is_leaf_template_for_api_object(
        this: *mut FunctionTemplate,
        value: *const Local<Value>,
    );
}

#[repr(C)]
pub struct FunctionTemplate([u8; 0]);

impl FunctionTemplate {
    #[inline(always)]
    pub fn new<F>(
        isolate: &mut Isolate,
        callback: F,
        data: &Local<Value>,
        signature: &Local<Signature>,
        length: i32,
        behavior: ConstructorBehavior,
        side_effect_type: SideEffectType,
    ) -> Local<Self>
    where
        F: Fn(&FunctionCallbackInfo<Value>),
    {
        let mut local_function_template = Local::empty();

        unsafe {
            v8cxx__function_template_new(
                &mut local_function_template,
                isolate,
                callback.to_function(),
                data,
                signature,
                length,
                behavior,
                side_effect_type,
            )
        };

        local_function_template
    }

    #[inline(always)]
    pub fn new_default<F>(isolate: &mut Isolate, callback: F) -> Local<Self>
    where
        F: Fn(&FunctionCallbackInfo<Value>),
    {
        Self::new(
            isolate,
            callback,
            &Local::empty(),
            &Local::empty(),
            0,
            ConstructorBehavior::Allow,
            SideEffectType::HasSideEffect,
        )
    }

    #[inline(always)]
    pub fn get_function(&mut self, context: &Local<Context>) -> MaybeLocal<Function> {
        let mut maybe_local_function = MaybeLocal::empty();

        unsafe { v8cxx__function_template_get_function(&mut maybe_local_function, self, context) };

        maybe_local_function
    }

    #[inline(always)]
    pub fn instance_template(&mut self) -> Local<ObjectTemplate> {
        let mut local_object_template = Local::empty();

        unsafe { v8cxx__function_template_instance_template(&mut local_object_template, self) };

        local_object_template
    }

    #[inline(always)]
    pub fn inherit(&mut self, parent: &Local<Self>) {
        unsafe { v8cxx__function_template_inherit(self, parent) };
    }

    #[inline(always)]
    pub fn prototype_template(&mut self) -> Local<ObjectTemplate> {
        let mut local_object_template = Local::empty();

        unsafe { v8cxx__function_template_prototype_template(&mut local_object_template, self) };

        local_object_template
    }

    #[inline(always)]
    pub fn set_prototype_provider_template(&mut self, prototype_provider: &Local<Self>) {
        unsafe {
            v8cxx__function_template_set_prototype_provider_template(self, prototype_provider)
        };
    }

    #[inline(always)]
    pub fn set_class_name(&mut self, name: &Local<String>) {
        unsafe { v8cxx__function_template_set_class_name(self, name) };
    }

    #[inline(always)]
    pub fn set_interface_name(&mut self, name: &Local<String>) {
        unsafe { v8cxx__function_template_set_interface_name(self, name) };
    }

    #[inline(always)]
    pub fn read_only_prototype(&mut self) {
        unsafe { v8cxx__function_template_read_only_prototype(self) };
    }

    #[inline(always)]
    pub fn remove_prototype(&mut self) {
        unsafe { v8cxx__function_template_remove_prototype(self) };
    }

    #[inline(always)]
    pub fn has_instance(&mut self, object: &Local<Value>) -> bool {
        unsafe { v8cxx__function_template_has_instance(self, object) }
    }

    #[inline(always)]
    pub fn set_exception_context(&mut self, context: ExceptionContext) {
        unsafe { v8cxx__function_template_set_exception_context(self, context) };
    }

    #[inline(always)]
    pub fn set_accept_any_receiver(&mut self, accept: bool) {
        unsafe { v8cxx__function_template_set_accept_any_receiver(self, accept) };
    }

    #[inline(always)]
    pub fn is_leaf_template_for_api_object(&mut self, value: &Local<Value>) {
        unsafe { v8cxx__function_template_is_leaf_template_for_api_object(self, value) };
    }
}

impl Data for FunctionTemplate {}
impl Template for FunctionTemplate {}

#[repr(C)]
pub enum ConstructorBehavior {
    Throw,
    Allow,
}

#[repr(C)]
pub enum SideEffectType {
    HasSideEffect,
    HasNoSideEffect,
    HasSideEffectToReceiver,
}

#[repr(C)]
pub enum ExceptionContext {
    Unknown,
    Constructor,
    Operation,
    AttributeGet,
    AttributeSet,
    IndexedQuery,
    IndexedGetter,
    IndexedDescriptor,
    IndexedSetter,
    IndexedDefiner,
    IndexedDeleter,
    NamedQuery,
    NamedGetter,
    NamedDescriptor,
    NamedSetter,
    NamedDefiner,
    NamedDeleter,
    NamedEnumerator,
}
