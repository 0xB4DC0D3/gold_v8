use crate::{
    c_support::ClosureToFunction,
    context::Context,
    data::traits::Data,
    fixed_array::FixedArray,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    object::Object,
    string::String,
    value::Value,
};

extern "C" {
    fn v8cxx__module_get_status(this: *const Module) -> Status;
    fn v8cxx__module_get_exception(local_buf: *mut Local<Value>, this: *const Module);
    fn v8cxx__module_get_module_requests(local_buf: *mut Local<FixedArray>, this: *const Module);
    fn v8cxx__module_source_offset_to_location(this: *const Module, offset: i32) -> Location;
    fn v8cxx__module_get_identity_hash(this: *const Module) -> i32;
    fn v8cxx__module_instantiate_module(
        this: *mut Module,
        context: *const Local<Context>,
        module_callback: ExternResolveModuleCallback,
    ) -> bool;
    fn v8cxx__module_evaluate(
        maybe_local_buf: *mut MaybeLocal<Value>,
        this: *mut Module,
        context: *const Local<Context>,
    );
    fn v8cxx__module_get_module_namespace(local_buf: *mut Local<Value>, this: *mut Module);
    fn v8cxx__module_script_id(this: *const Module) -> i32;
    fn v8cxx__module_is_graph_async(this: *const Module) -> bool;
    fn v8cxx__module_has_top_level_await(this: *const Module) -> bool;
    fn v8cxx__module_is_source_text_module(this: *const Module) -> bool;
    fn v8cxx__module_is_synthetic_module(this: *const Module) -> bool;
    fn v8cxx__module_create_synthetic_module(
        local_buf: *mut Local<Module>,
        isolate: *mut Isolate,
        module_name: *const Local<String>,
        export_names: *const Local<String>,
        export_names_length: usize,
        evaluation_steps: ExternSyntheticModuleEvaluationSteps,
    );
    fn v8cxx__module_set_synthetic_module_export(
        this: *mut Module,
        isolate: *mut Isolate,
        export_name: *const Local<String>,
        export_value: *const Local<Value>,
    ) -> bool;
}

#[repr(C)]
pub struct Module([u8; 0]);

impl Module {
    #[inline(always)]
    pub fn get_status(&self) -> Status {
        unsafe { v8cxx__module_get_status(self) }
    }

    #[inline(always)]
    pub fn get_exception(&self) -> Local<Value> {
        let mut local_value = Local::<Value>::empty();

        unsafe { v8cxx__module_get_exception(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn get_module_requests(&self) -> Local<FixedArray> {
        let mut local_fixedarray = Local::<FixedArray>::empty();

        unsafe { v8cxx__module_get_module_requests(&mut local_fixedarray, self) };

        local_fixedarray
    }

    #[inline(always)]
    pub fn source_offset_to_location(&self, offset: i32) -> Location {
        unsafe { v8cxx__module_source_offset_to_location(self, offset) }
    }

    #[inline(always)]
    pub fn get_identity_hash(&self) -> i32 {
        unsafe { v8cxx__module_get_identity_hash(self) }
    }

    #[inline(always)]
    pub fn instantiate_module<M>(
        &mut self,
        context: &Local<Context>,
        module_callback: M,
        // source_callback: Option<S>,
    ) -> bool
    where
        M: Fn(
            Local<Context>,
            Local<String>,
            Local<FixedArray>,
            Local<Module>,
        ) -> MaybeLocal<Module>,
        /* S: Fn(
            Local<Context>,
            Local<String>,
            Local<FixedArray>,
            Local<Module>,
        ) -> MaybeLocal<Object>, */
    {
        unsafe {
            v8cxx__module_instantiate_module(
                self,
                context,
                module_callback.to_function(),
                // source_callback.map(|sc| sc.to_function()),
            )
        }
    }

    #[inline(always)]
    pub fn evaluate(&mut self, context: &Local<Context>) -> MaybeLocal<Value> {
        let mut maybe_local_value = MaybeLocal::<Value>::empty();

        unsafe { v8cxx__module_evaluate(&mut maybe_local_value, self, context) };

        maybe_local_value
    }

    #[inline(always)]
    pub fn get_module_namespace(&mut self) -> Local<Value> {
        let mut local_value = Local::<Value>::empty();

        unsafe { v8cxx__module_get_module_namespace(&mut local_value, self) };

        local_value
    }

    #[inline(always)]
    pub fn script_id(&self) -> i32 {
        unsafe { v8cxx__module_script_id(self) }
    }

    #[inline(always)]
    pub fn is_graph_async(&self) -> bool {
        unsafe { v8cxx__module_is_graph_async(self) }
    }

    #[inline(always)]
    pub fn has_top_level_await(&self) -> bool {
        unsafe { v8cxx__module_has_top_level_await(self) }
    }

    #[inline(always)]
    pub fn is_source_text_module(&self) -> bool {
        unsafe { v8cxx__module_is_source_text_module(self) }
    }

    #[inline(always)]
    pub fn is_synthetic_module(&self) -> bool {
        unsafe { v8cxx__module_is_synthetic_module(self) }
    }

    #[inline(always)]
    pub fn create_synthetic_module<F>(
        isolate: &mut Isolate,
        module_name: &Local<String>,
        export_names: Vec<Local<String>>,
        evaluation_steps: F,
    ) -> Local<Self>
    where
        F: Fn(Local<Context>, Local<Module>) -> MaybeLocal<Value>,
    {
        let mut local_module = Local::empty();

        unsafe {
            v8cxx__module_create_synthetic_module(
                &mut local_module,
                isolate,
                module_name,
                export_names.as_ptr(),
                export_names.len(),
                evaluation_steps.to_function(),
            )
        };

        local_module
    }

    #[inline(always)]
    pub fn set_synthetic_module_export(
        &mut self,
        isolate: &mut Isolate,
        export_name: &Local<String>,
        export_value: &Local<Value>,
    ) -> bool {
        unsafe {
            v8cxx__module_set_synthetic_module_export(self, isolate, export_name, export_value)
        }
    }
}

impl Data for Module {}

type ExternSyntheticModuleEvaluationSteps =
    extern "C" fn(*mut MaybeLocal<Value>, Local<Context>, Local<Module>) -> *mut MaybeLocal<Value>;

impl<T> ClosureToFunction<ExternSyntheticModuleEvaluationSteps> for T
where
    T: Fn(Local<Context>, Local<Module>) -> MaybeLocal<Value>,
{
    fn to_function(self) -> ExternSyntheticModuleEvaluationSteps {
        extern "C" fn function<T>(
            return_value: *mut MaybeLocal<Value>,
            context: Local<Context>,
            module: Local<Module>,
        ) -> *mut MaybeLocal<Value>
        where
            T: Fn(Local<Context>, Local<Module>) -> MaybeLocal<Value>,
        {
            unsafe { return_value.write(T::get()(context, module)) };

            return_value
        }

        function::<T>
    }
}

type ExternResolveModuleCallback = extern "C" fn(
    *mut MaybeLocal<Module>,
    Local<Context>,
    Local<String>,
    Local<FixedArray>,
    Local<Module>,
) -> *mut MaybeLocal<Module>;

impl<T> ClosureToFunction<ExternResolveModuleCallback> for T
where
    T: Fn(
        Local<Context>,
        Local<String>,
        Local<FixedArray>,
        Local<Module>,
    ) -> MaybeLocal<Module>,
{
    fn to_function(self) -> ExternResolveModuleCallback {
        extern "C" fn function<T>(
            return_value: *mut MaybeLocal<Module>,
            context: Local<Context>,
            specifier: Local<String>,
            import_attributes: Local<FixedArray>,
            referrer: Local<Module>,
        ) -> *mut MaybeLocal<Module>
        where
            T: Fn(
                Local<Context>,
                Local<String>,
                Local<FixedArray>,
                Local<Module>,
            ) -> MaybeLocal<Module>,
        {
            unsafe { return_value.write(T::get()(context, specifier, import_attributes, referrer)) }

            return_value
        }

        function::<T>
    }
}

type ExternResolveSourceCallback = extern "C" fn(
    Local<Context>,
    Local<String>,
    Local<FixedArray>,
    Local<Module>,
) -> MaybeLocal<Object>;

impl<T> ClosureToFunction<ExternResolveSourceCallback> for T
where
    T: Fn(Local<Context>, Local<String>, Local<FixedArray>, Local<Module>) -> MaybeLocal<Object>,
{
    fn to_function(self) -> ExternResolveSourceCallback {
        extern "C" fn function<T>(
            context: Local<Context>,
            specifier: Local<String>,
            import_attributes: Local<FixedArray>,
            referrer: Local<Module>,
        ) -> MaybeLocal<Object>
        where
            T: Fn(
                Local<Context>,
                Local<String>,
                Local<FixedArray>,
                Local<Module>,
            ) -> MaybeLocal<Object>,
        {
            T::get()(context, specifier, import_attributes, referrer)
        }

        function::<T>
    }
}

#[repr(C)]
pub struct Location(i32, i32);

impl Location {
    #[inline(always)]
    pub const fn line_number(&self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub const fn column_number(&self) -> i32 {
        self.1
    }
}

#[repr(C)]
pub enum Status {
    Uninstantiated,
    Instantiating,
    Instantiated,
    Evaluating,
    Evaluated,
    Errored,
}
