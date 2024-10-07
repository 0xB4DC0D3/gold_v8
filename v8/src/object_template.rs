use crate::{
    array::Array,
    boolean::Boolean,
    c_support::ClosureToFunction,
    context::Context,
    data::traits::Data,
    function::ExternFunctionCallback,
    function_callback_info::FunctionCallbackInfo,
    function_template::FunctionTemplate,
    integer::Integer,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    name::Name,
    object::Object,
    property_callback_info::PropertyCallbackInfo,
    template::traits::Template,
    value::Value,
};

extern "C" {
    fn v8cxx_object_template_new(
        local_buf: *mut Local<ObjectTemplate>,
        isolate: *mut Isolate,
        constructor: *const Local<FunctionTemplate>,
    );
    fn v8cxx_object_template_new_instance(
        local_buf: *mut MaybeLocal<Object>,
        object_template: *mut ObjectTemplate,
        context: *const Local<Context>,
    );
    fn v8cxx_object_template_set_named_property_handler(
        object_template: *mut ObjectTemplate,
        getter: Option<ExternNamedPropertyGetterCallback>,
        setter: Option<ExternNamedPropertySetterCallback>,
        query: Option<ExternNamedPropertyQueryCallback>,
        deleter: Option<ExternNamedPropertyDeleterCallback>,
        enumerator: Option<ExternNamedPropertyEnumeratorCallback>,
        definer: Option<ExternNamedPropertyDefinerCallback>,
        descriptor: Option<ExternNamedPropertyDescriptorCallback>,
        data: *const Local<Value>,
        flags: PropertyHandlerFlags,
    );
    fn v8cxx_object_template_set_indexed_property_handler(
        object_template: *mut ObjectTemplate,
        getter: Option<ExternIndexedPropertyGetterCallbackV2>,
        setter: Option<ExternIndexedPropertySetterCallbackV2>,
        query: Option<ExternIndexedPropertyQueryCallbackV2>,
        deleter: Option<ExternIndexedPropertyDeleterCallbackV2>,
        enumerator: Option<ExternIndexedPropertyEnumeratorCallback>,
        definer: Option<ExternIndexedPropertyDefinerCallbackV2>,
        descriptor: Option<ExternIndexedPropertyDescriptorCallbackV2>,
        data: *const Local<Value>,
        flags: PropertyHandlerFlags,
    );
    fn v8cxx_object_template_set_call_as_function_handler(
        object_template: *mut ObjectTemplate,
        callback: ExternFunctionCallback,
        data: *const Local<Value>,
    );
    fn v8cxx_object_template_mark_as_undetectable(object_template: *mut ObjectTemplate);
    fn v8cxx_object_template_internal_field_count(object_template: *const ObjectTemplate) -> i32;
    fn v8cxx_object_template_set_internal_field_count(
        object_template: *mut ObjectTemplate,
        count: i32,
    );
    fn v8cxx_object_template_is_immutable_proto(object_template: *const ObjectTemplate) -> bool;
}

#[repr(C)]
pub struct ObjectTemplate([u8; 0]);

impl ObjectTemplate {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, constructor: &Local<FunctionTemplate>) -> Local<Self> {
        let mut local_object_template = Local::empty();

        unsafe { v8cxx_object_template_new(&mut local_object_template, isolate, constructor) };

        local_object_template
    }

    #[inline(always)]
    pub fn new_instance(&mut self, context: &Local<Context>) -> MaybeLocal<Object> {
        let mut maybe_local_object = MaybeLocal::empty();

        unsafe { v8cxx_object_template_new_instance(&mut maybe_local_object, self, context) };

        maybe_local_object
    }

    #[inline(always)]
    pub fn set_named_property_handler(
        &mut self,
        getter: Option<impl Fn(Local<Name>, &PropertyCallbackInfo<Value>) -> Intercepted>,
        setter: Option<
            impl Fn(Local<Name>, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted,
        >,
        query: Option<impl Fn(Local<Name>, &PropertyCallbackInfo<Integer>) -> Intercepted>,
        deleter: Option<impl Fn(Local<Name>, &PropertyCallbackInfo<Boolean>) -> Intercepted>,
        enumerator: Option<impl Fn(&PropertyCallbackInfo<Array>)>,
        definer: Option<
            impl Fn(Local<Name>, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
        >,
        descriptor: Option<impl Fn(Local<Name>, &PropertyCallbackInfo<Value>) -> Intercepted>,
        data: &Local<Value>,
        flags: PropertyHandlerFlags,
    ) {
        unsafe {
            v8cxx_object_template_set_named_property_handler(
                self,
                getter.map(|f| f.to_function()),
                setter.map(|f| f.to_function()),
                query.map(|f| f.to_function()),
                deleter.map(|f| f.to_function()),
                enumerator.map(|f| f.to_function()),
                definer.map(|f| f.to_function()),
                descriptor.map(|f| f.to_function()),
                data,
                flags,
            )
        };
    }

    #[inline(always)]
    pub fn set_indexed_property_handler(
        &mut self,
        getter: Option<impl Fn(u32, &PropertyCallbackInfo<Value>) -> Intercepted>,
        setter: Option<impl Fn(u32, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted>,
        query: Option<impl Fn(u32, &PropertyCallbackInfo<Integer>) -> Intercepted>,
        deleter: Option<impl Fn(u32, &PropertyCallbackInfo<Boolean>) -> Intercepted>,
        enumerator: Option<impl Fn(&PropertyCallbackInfo<Array>)>,
        definer: Option<
            impl Fn(u32, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
        >,
        descriptor: Option<impl Fn(u32, &PropertyCallbackInfo<Value>) -> Intercepted>,
        data: &Local<Value>,
        flags: PropertyHandlerFlags,
    ) {
        unsafe {
            v8cxx_object_template_set_indexed_property_handler(
                self,
                getter.map(|f| f.to_function()),
                setter.map(|f| f.to_function()),
                query.map(|f| f.to_function()),
                deleter.map(|f| f.to_function()),
                enumerator.map(|f| f.to_function()),
                definer.map(|f| f.to_function()),
                descriptor.map(|f| f.to_function()),
                data,
                flags,
            )
        };
    }

    #[inline(always)]
    pub fn set_call_as_function_handler<F>(&mut self, callback: F, data: &Local<Value>)
    where
        F: Fn(&FunctionCallbackInfo<Value>),
    {
        unsafe {
            v8cxx_object_template_set_call_as_function_handler(self, callback.to_function(), data)
        };
    }

    #[inline(always)]
    pub fn mark_as_undetectable(&mut self) {
        unsafe { v8cxx_object_template_mark_as_undetectable(self) };
    }

    #[inline(always)]
    pub fn internal_field_count(&self) -> i32 {
        unsafe { v8cxx_object_template_internal_field_count(self) }
    }

    #[inline(always)]
    pub fn set_internal_field_count(&mut self, count: i32) {
        unsafe { v8cxx_object_template_set_internal_field_count(self, count) };
    }

    #[inline(always)]
    pub fn is_immutable_proto(&self) -> bool {
        unsafe { v8cxx_object_template_is_immutable_proto(self) }
    }
}

impl Data for ObjectTemplate {}
impl Template for ObjectTemplate {}

#[repr(u8)]
pub enum Intercepted {
    No,
    Yes,
}

#[repr(C)]
pub enum PropertyHandlerFlags {
    None = 0,
    NonMasking = 1,
    OnlyInterceptStrings = 1 << 1,
    HasNoSideEffect = 1 << 2,
    InternalNewCallbacksSignatures = 1 << 10,
}

// TODO: implement this
#[repr(C)]
pub struct PropertyDescriptor([u8; 0]);

type ExternNamedPropertyGetterCallback =
    extern "C" fn(Local<Name>, *const PropertyCallbackInfo<Value>) -> Intercepted;

type ExternNamedPropertySetterCallback =
    extern "C" fn(Local<Name>, Local<Value>, *const PropertyCallbackInfo<()>) -> Intercepted;

type ExternNamedPropertyQueryCallback =
    extern "C" fn(Local<Name>, *const PropertyCallbackInfo<Integer>) -> Intercepted;

type ExternNamedPropertyDeleterCallback =
    extern "C" fn(Local<Name>, *const PropertyCallbackInfo<Boolean>) -> Intercepted;

type ExternNamedPropertyEnumeratorCallback = extern "C" fn(*const PropertyCallbackInfo<Array>);

type ExternNamedPropertyDefinerCallback = extern "C" fn(
    Local<Name>,
    *const PropertyDescriptor,
    *const PropertyCallbackInfo<()>,
) -> Intercepted;

type ExternNamedPropertyDescriptorCallback =
    extern "C" fn(Local<Name>, *const PropertyCallbackInfo<Value>) -> Intercepted;

type ExternIndexedPropertyGetterCallbackV2 =
    extern "C" fn(u32, *const PropertyCallbackInfo<Value>) -> Intercepted;

type ExternIndexedPropertySetterCallbackV2 =
    extern "C" fn(u32, Local<Value>, *const PropertyCallbackInfo<()>) -> Intercepted;

type ExternIndexedPropertyQueryCallbackV2 =
    extern "C" fn(u32, *const PropertyCallbackInfo<Integer>) -> Intercepted;

type ExternIndexedPropertyDeleterCallbackV2 =
    extern "C" fn(u32, *const PropertyCallbackInfo<Boolean>) -> Intercepted;

type ExternIndexedPropertyEnumeratorCallback = extern "C" fn(*const PropertyCallbackInfo<Array>);

type ExternIndexedPropertyDefinerCallbackV2 =
    extern "C" fn(u32, *const PropertyDescriptor, *const PropertyCallbackInfo<()>) -> Intercepted;

type ExternIndexedPropertyDescriptorCallbackV2 =
    extern "C" fn(u32, *const PropertyCallbackInfo<Value>) -> Intercepted;

impl<T> ClosureToFunction<ExternNamedPropertyGetterCallback> for T
where
    T: Fn(Local<Name>, &PropertyCallbackInfo<Value>) -> Intercepted,
{
    fn to_function(self) -> ExternNamedPropertyGetterCallback {
        extern "C" fn function<T>(
            property: Local<Name>,
            pci: *const PropertyCallbackInfo<Value>,
        ) -> Intercepted
        where
            T: Fn(Local<Name>, &PropertyCallbackInfo<Value>) -> Intercepted,
        {
            T::get()(property, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternNamedPropertySetterCallback> for T
where
    T: Fn(Local<Name>, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted,
{
    fn to_function(self) -> ExternNamedPropertySetterCallback {
        extern "C" fn function<T>(
            property: Local<Name>,
            value: Local<Value>,
            pci: *const PropertyCallbackInfo<()>,
        ) -> Intercepted
        where
            T: Fn(Local<Name>, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted,
        {
            T::get()(property, value, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternNamedPropertyQueryCallback> for T
where
    T: Fn(Local<Name>, &PropertyCallbackInfo<Integer>) -> Intercepted,
{
    fn to_function(self) -> ExternNamedPropertyQueryCallback {
        extern "C" fn function<T>(
            property: Local<Name>,
            pci: *const PropertyCallbackInfo<Integer>,
        ) -> Intercepted
        where
            T: Fn(Local<Name>, &PropertyCallbackInfo<Integer>) -> Intercepted,
        {
            T::get()(property, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternNamedPropertyDeleterCallback> for T
where
    T: Fn(Local<Name>, &PropertyCallbackInfo<Boolean>) -> Intercepted,
{
    fn to_function(self) -> ExternNamedPropertyDeleterCallback {
        extern "C" fn function<T>(
            property: Local<Name>,
            pci: *const PropertyCallbackInfo<Boolean>,
        ) -> Intercepted
        where
            T: Fn(Local<Name>, &PropertyCallbackInfo<Boolean>) -> Intercepted,
        {
            T::get()(property, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternNamedPropertyEnumeratorCallback> for T
where
    T: Fn(&PropertyCallbackInfo<Array>),
{
    fn to_function(self) -> ExternNamedPropertyEnumeratorCallback {
        extern "C" fn function<T>(pci: *const PropertyCallbackInfo<Array>)
        where
            T: Fn(&PropertyCallbackInfo<Array>),
        {
            T::get()(unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternNamedPropertyDefinerCallback> for T
where
    T: Fn(Local<Name>, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
{
    fn to_function(self) -> ExternNamedPropertyDefinerCallback {
        extern "C" fn function<T>(
            property: Local<Name>,
            desc: *const PropertyDescriptor,
            pci: *const PropertyCallbackInfo<()>,
        ) -> Intercepted
        where
            T: Fn(Local<Name>, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
        {
            T::get()(property, unsafe { desc.as_ref().unwrap() }, unsafe {
                pci.as_ref().unwrap()
            })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternIndexedPropertyGetterCallbackV2> for T
where
    T: Fn(u32, &PropertyCallbackInfo<Value>) -> Intercepted,
{
    fn to_function(self) -> ExternIndexedPropertyGetterCallbackV2 {
        extern "C" fn function<T>(
            index: u32,
            pci: *const PropertyCallbackInfo<Value>,
        ) -> Intercepted
        where
            T: Fn(u32, &PropertyCallbackInfo<Value>) -> Intercepted,
        {
            T::get()(index, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternIndexedPropertySetterCallbackV2> for T
where
    T: Fn(u32, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted,
{
    fn to_function(self) -> ExternIndexedPropertySetterCallbackV2 {
        extern "C" fn function<T>(
            index: u32,
            value: Local<Value>,
            pci: *const PropertyCallbackInfo<()>,
        ) -> Intercepted
        where
            T: Fn(u32, Local<Value>, &PropertyCallbackInfo<()>) -> Intercepted,
        {
            T::get()(index, value, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternIndexedPropertyQueryCallbackV2> for T
where
    T: Fn(u32, &PropertyCallbackInfo<Integer>) -> Intercepted,
{
    fn to_function(self) -> ExternIndexedPropertyQueryCallbackV2 {
        extern "C" fn function<T>(
            index: u32,
            pci: *const PropertyCallbackInfo<Integer>,
        ) -> Intercepted
        where
            T: Fn(u32, &PropertyCallbackInfo<Integer>) -> Intercepted,
        {
            T::get()(index, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternIndexedPropertyDeleterCallbackV2> for T
where
    T: Fn(u32, &PropertyCallbackInfo<Boolean>) -> Intercepted,
{
    fn to_function(self) -> ExternIndexedPropertyDeleterCallbackV2 {
        extern "C" fn function<T>(
            index: u32,
            pci: *const PropertyCallbackInfo<Boolean>,
        ) -> Intercepted
        where
            T: Fn(u32, &PropertyCallbackInfo<Boolean>) -> Intercepted,
        {
            T::get()(index, unsafe { pci.as_ref().unwrap() })
        }

        function::<T>
    }
}

impl<T> ClosureToFunction<ExternIndexedPropertyDefinerCallbackV2> for T
where
    T: Fn(u32, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
{
    fn to_function(self) -> ExternIndexedPropertyDefinerCallbackV2 {
        extern "C" fn function<T>(
            index: u32,
            desc: *const PropertyDescriptor,
            pci: *const PropertyCallbackInfo<()>,
        ) -> Intercepted
        where
            T: Fn(u32, &PropertyDescriptor, &PropertyCallbackInfo<()>) -> Intercepted,
        {
            T::get()(index, unsafe { desc.as_ref().unwrap() }, unsafe {
                pci.as_ref().unwrap()
            })
        }

        function::<T>
    }
}
