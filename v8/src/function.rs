use std::mem::MaybeUninit;

use crate::{
    c_support::ClosureToFunction,
    context::Context,
    data::traits::Data,
    function_callback_info::FunctionCallbackInfo,
    function_template::{ConstructorBehavior, SideEffectType},
    local::{Local, MaybeLocal},
    object::{self, traits::Object},
    script_origin::ScriptOrigin,
    string::String,
    value::{self, traits::Value},
};

extern "C" {
    fn v8cxx__function_new(
        maybe_local_buf: *mut MaybeLocal<Function>,
        context: *const Local<Context>,
        callback: ExternFunctionCallback,
        data: *const Local<value::Value>,
        length: i32,
        behavior: ConstructorBehavior,
        side_effect_type: SideEffectType,
    );
    fn v8cxx__function_new_instance(
        maybe_local_buf: *mut MaybeLocal<object::Object>,
        this: *const Function,
        context: *const Local<Context>,
        argc: i32,
        argv: *mut Local<value::Value>,
    );
    fn v8cxx__function_new_instance_with_side_effect_type(
        maybe_local_buf: *mut MaybeLocal<object::Object>,
        this: *const Function,
        context: *const Local<Context>,
        argc: i32,
        argv: *mut Local<value::Value>,
        side_effect_type: SideEffectType,
    );
    fn v8cxx__function_set_name(this: *mut Function, name: *const Local<String>);
    fn v8cxx__function_get_name(local_buf: *mut Local<value::Value>, this: *const Function);
    fn v8cxx__function_get_inferred_name(
        local_buf: *mut Local<value::Value>,
        this: *const Function,
    );
    fn v8cxx__function_get_script_line_number(this: *const Function) -> i32;
    fn v8cxx__function_get_script_column_number(this: *const Function) -> i32;
    fn v8cxx__function_script_id(this: *const Function) -> i32;
    fn v8cxx__function_get_bound_function(
        local_buf: *mut Local<value::Value>,
        this: *const Function,
    );
    fn v8cxx__function_function_proto_to_string(
        maybe_local_buf: *mut MaybeLocal<String>,
        this: *mut Function,
        context: *const Local<Context>,
    );
    fn v8cxx__function_get_script_origin(buf: *mut ScriptOrigin, this: *const Function);
}

#[repr(C)]
pub struct Function([u8; 0]);

impl Function {
    #[inline(always)]
    pub fn new<F>(
        context: &Local<Context>,
        callback: F,
        data: &Local<value::Value>,
        length: i32,
        behavior: ConstructorBehavior,
        side_effect_type: SideEffectType,
    ) -> MaybeLocal<Function>
    where
        F: Fn(&FunctionCallbackInfo<value::Value>),
    {
        let mut maybe_local_function = MaybeLocal::empty();

        unsafe {
            v8cxx__function_new(
                &mut maybe_local_function,
                context,
                callback.to_function(),
                data,
                length,
                behavior,
                side_effect_type,
            )
        };

        maybe_local_function
    }

    #[inline(always)]
    pub fn new_instance(
        &self,
        context: &Local<Context>,
        args: &mut Vec<Local<value::Value>>,
    ) -> MaybeLocal<object::Object> {
        let mut maybe_local_object = MaybeLocal::empty();

        unsafe {
            v8cxx__function_new_instance(
                &mut maybe_local_object,
                self,
                context,
                args.len() as i32,
                args.as_mut_ptr(),
            )
        };

        maybe_local_object
    }

    #[inline(always)]
    pub fn new_instance_with_side_effect_type(
        &self,
        context: &Local<Context>,
        args: &mut Vec<Local<value::Value>>,
        side_effect_type: SideEffectType,
    ) -> MaybeLocal<object::Object> {
        let mut maybe_local_object = MaybeLocal::empty();

        unsafe {
            v8cxx__function_new_instance_with_side_effect_type(
                &mut maybe_local_object,
                self,
                context,
                args.len() as i32,
                args.as_mut_ptr(),
                side_effect_type,
            )
        };

        maybe_local_object
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &Local<String>) {
        unsafe { v8cxx__function_set_name(self, name) };
    }

    #[inline(always)]
    pub fn get_name(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__function_get_name(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_inferred_name(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe {
            v8cxx__function_get_inferred_name(&mut local_value, self);
        }

        local_value
    }

    #[inline(always)]
    pub fn get_script_line_number(&self) -> i32 {
        unsafe { v8cxx__function_get_script_line_number(self) }
    }

    #[inline(always)]
    pub fn get_script_column_number(&self) -> i32 {
        unsafe { v8cxx__function_get_script_column_number(self) }
    }

    #[inline(always)]
    pub fn script_id(&self) -> i32 {
        unsafe { v8cxx__function_script_id(self) }
    }

    #[inline(always)]
    pub fn get_bound_function(&self) -> Local<value::Value> {
        let mut local_value = Local::empty();

        unsafe { v8cxx__function_get_bound_function(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn function_proto_to_string(&mut self, context: &Local<Context>) -> MaybeLocal<String> {
        let mut maybe_local_string = MaybeLocal::empty();

        unsafe { v8cxx__function_function_proto_to_string(&mut maybe_local_string, self, context) };

        maybe_local_string
    }

    #[inline(always)]
    pub fn get_script_origin(&self) -> ScriptOrigin {
        unsafe {
            let mut script_origin = MaybeUninit::<ScriptOrigin>::zeroed();

            v8cxx__function_get_script_origin(script_origin.as_mut_ptr(), self);

            script_origin.assume_init()
        }
    }
}

impl Data for Function {}
impl Value for Function {}
impl Object for Function {}

pub type ExternFunctionCallback = extern "C" fn(info: *const FunctionCallbackInfo<value::Value>);

impl<T> ClosureToFunction<ExternFunctionCallback> for T
where
    T: Fn(&FunctionCallbackInfo<value::Value>),
{
    fn to_function(self) -> ExternFunctionCallback {
        extern "C" fn function<T>(info: *const FunctionCallbackInfo<value::Value>)
        where
            T: Fn(&FunctionCallbackInfo<value::Value>),
        {
            T::get()(unsafe { info.as_ref().unwrap() });
        }

        function::<T>
    }
}
