use crate::{
    bigint::BigInt, boolean::Boolean, context::Context, data::traits::Data, isolate::Isolate,
    local::Local, number::Number, object::Object, primitive::Primitive, string::String,
};

extern "C" {
    fn v8cxx__value_is_undefined(value: *const Value) -> bool;
    fn v8cxx__value_is_null(value: *const Value) -> bool;
    fn v8cxx__value_is_null_or_undefined(value: *const Value) -> bool;
    fn v8cxx__value_is_true(value: *const Value) -> bool;
    fn v8cxx__value_is_false(value: *const Value) -> bool;
    fn v8cxx__value_is_name(value: *const Value) -> bool;
    fn v8cxx__value_is_string(value: *const Value) -> bool;
    fn v8cxx__value_is_symbol(value: *const Value) -> bool;
    fn v8cxx__value_is_function(value: *const Value) -> bool;
    fn v8cxx__value_is_array(value: *const Value) -> bool;
    fn v8cxx__value_is_object(value: *const Value) -> bool;
    fn v8cxx__value_is_big_int(value: *const Value) -> bool;
    fn v8cxx__value_is_boolean(value: *const Value) -> bool;
    fn v8cxx__value_is_number(value: *const Value) -> bool;
    fn v8cxx__value_is_external(value: *const Value) -> bool;
    fn v8cxx__value_is_int32(value: *const Value) -> bool;
    fn v8cxx__value_is_uint32(value: *const Value) -> bool;
    fn v8cxx__value_is_date(value: *const Value) -> bool;
    fn v8cxx__value_is_arguments_object(value: *const Value) -> bool;
    fn v8cxx__value_is_big_int_object(value: *const Value) -> bool;
    fn v8cxx__value_is_boolean_object(value: *const Value) -> bool;
    fn v8cxx__value_is_number_object(value: *const Value) -> bool;
    fn v8cxx__value_is_string_object(value: *const Value) -> bool;
    fn v8cxx__value_is_symbol_object(value: *const Value) -> bool;
    fn v8cxx__value_is_native_error(value: *const Value) -> bool;
    fn v8cxx__value_is_reg_exp(value: *const Value) -> bool;
    fn v8cxx__value_is_async_function(value: *const Value) -> bool;
    fn v8cxx__value_is_generator_function(value: *const Value) -> bool;
    fn v8cxx__value_is_generator_object(value: *const Value) -> bool;
    fn v8cxx__value_is_promise(value: *const Value) -> bool;
    fn v8cxx__value_is_map(value: *const Value) -> bool;
    fn v8cxx__value_is_set(value: *const Value) -> bool;
    fn v8cxx__value_is_map_iterator(value: *const Value) -> bool;
    fn v8cxx__value_is_set_iterator(value: *const Value) -> bool;
    fn v8cxx__value_is_weak_map(value: *const Value) -> bool;
    fn v8cxx__value_is_weak_set(value: *const Value) -> bool;
    fn v8cxx__value_is_weak_ref(value: *const Value) -> bool;
    fn v8cxx__value_is_array_buffer(value: *const Value) -> bool;
    fn v8cxx__value_is_array_buffer_view(value: *const Value) -> bool;
    fn v8cxx__value_is_typed_array(value: *const Value) -> bool;
    fn v8cxx__value_is_uint8_array(value: *const Value) -> bool;
    fn v8cxx__value_is_uint8_clamped_array(value: *const Value) -> bool;
    fn v8cxx__value_is_int8_array(value: *const Value) -> bool;
    fn v8cxx__value_is_uint16_array(value: *const Value) -> bool;
    fn v8cxx__value_is_int16_array(value: *const Value) -> bool;
    fn v8cxx__value_is_uint32_array(value: *const Value) -> bool;
    fn v8cxx__value_is_int32_array(value: *const Value) -> bool;
    fn v8cxx__value_is_float16_array(value: *const Value) -> bool;
    fn v8cxx__value_is_float32_array(value: *const Value) -> bool;
    fn v8cxx__value_is_float64_array(value: *const Value) -> bool;
    fn v8cxx__value_is_big_int64_array(value: *const Value) -> bool;
    fn v8cxx__value_is_big_uint64_array(value: *const Value) -> bool;
    fn v8cxx__value_is_data_view(value: *const Value) -> bool;
    fn v8cxx__value_is_shared_array_buffer(value: *const Value) -> bool;
    fn v8cxx__value_is_proxy(value: *const Value) -> bool;
    fn v8cxx__value_is_wasm_memory_object(value: *const Value) -> bool;
    fn v8cxx__value_is_wasm_module_object(value: *const Value) -> bool;
    fn v8cxx__value_is_wasm_null(value: *const Value) -> bool;
    fn v8cxx__value_is_module_namespace_object(value: *const Value) -> bool;
    fn v8cxx__value_to_primitive(
        local_buf: *mut Local<Primitive>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_to_bigint(
        local_buf: *mut Local<BigInt>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_to_number(
        local_buf: *mut Local<Number>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_to_string(
        local_buf: *mut Local<String>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_to_object(
        local_buf: *mut Local<Object>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_to_boolean(
        local_buf: *mut Local<Boolean>,
        value: *const Value,
        context: *const Local<Context>,
    );
    fn v8cxx__value_typeof(local_buf: *mut Local<String>, value: *mut Value, isolate: *mut Isolate);
    fn v8cxx__value_instanceof(
        this: *const Value,
        context: *const Local<Context>,
        object: *const Local<Object>,
    ) -> bool;
}

#[repr(C)]
pub struct Value([u8; 0]);

impl Data for Value {}
impl traits::Value for Value {}

pub mod traits {
    use crate::{
        bigint::BigInt, boolean::Boolean, context::Context, data::traits::Data, local::Local,
        number::Number, object::Object, primitive::Primitive,
    };

    use super::*;

    pub trait Value: Data {
        fn is_undefined(&self) -> bool {
            unsafe { v8cxx__value_is_undefined(self as *const _ as *const _) }
        }

        fn is_null(&self) -> bool {
            unsafe { v8cxx__value_is_null(self as *const _ as *const _) }
        }

        fn is_null_or_undefined(&self) -> bool {
            unsafe { v8cxx__value_is_null_or_undefined(self as *const _ as *const _) }
        }

        fn is_true(&self) -> bool {
            unsafe { v8cxx__value_is_true(self as *const _ as *const _) }
        }

        fn is_false(&self) -> bool {
            unsafe { v8cxx__value_is_false(self as *const _ as *const _) }
        }

        fn is_name(&self) -> bool {
            unsafe { v8cxx__value_is_name(self as *const _ as *const _) }
        }

        fn is_string(&self) -> bool {
            unsafe { v8cxx__value_is_string(self as *const _ as *const _) }
        }

        fn is_symbol(&self) -> bool {
            unsafe { v8cxx__value_is_symbol(self as *const _ as *const _) }
        }

        fn is_function(&self) -> bool {
            unsafe { v8cxx__value_is_function(self as *const _ as *const _) }
        }

        fn is_array(&self) -> bool {
            unsafe { v8cxx__value_is_array(self as *const _ as *const _) }
        }

        fn is_object(&self) -> bool {
            unsafe { v8cxx__value_is_object(self as *const _ as *const _) }
        }

        fn is_big_int(&self) -> bool {
            unsafe { v8cxx__value_is_big_int(self as *const _ as *const _) }
        }

        fn is_boolean(&self) -> bool {
            unsafe { v8cxx__value_is_boolean(self as *const _ as *const _) }
        }

        fn is_number(&self) -> bool {
            unsafe { v8cxx__value_is_number(self as *const _ as *const _) }
        }

        fn is_external(&self) -> bool {
            unsafe { v8cxx__value_is_external(self as *const _ as *const _) }
        }

        fn is_int32(&self) -> bool {
            unsafe { v8cxx__value_is_int32(self as *const _ as *const _) }
        }

        fn is_uint32(&self) -> bool {
            unsafe { v8cxx__value_is_uint32(self as *const _ as *const _) }
        }

        fn is_date(&self) -> bool {
            unsafe { v8cxx__value_is_date(self as *const _ as *const _) }
        }

        fn is_arguments_object(&self) -> bool {
            unsafe { v8cxx__value_is_arguments_object(self as *const _ as *const _) }
        }

        fn is_big_int_object(&self) -> bool {
            unsafe { v8cxx__value_is_big_int_object(self as *const _ as *const _) }
        }

        fn is_boolean_object(&self) -> bool {
            unsafe { v8cxx__value_is_boolean_object(self as *const _ as *const _) }
        }

        fn is_number_object(&self) -> bool {
            unsafe { v8cxx__value_is_number_object(self as *const _ as *const _) }
        }

        fn is_string_object(&self) -> bool {
            unsafe { v8cxx__value_is_string_object(self as *const _ as *const _) }
        }

        fn is_symbol_object(&self) -> bool {
            unsafe { v8cxx__value_is_symbol_object(self as *const _ as *const _) }
        }

        fn is_native_error(&self) -> bool {
            unsafe { v8cxx__value_is_native_error(self as *const _ as *const _) }
        }

        fn is_reg_exp(&self) -> bool {
            unsafe { v8cxx__value_is_reg_exp(self as *const _ as *const _) }
        }

        fn is_async_function(&self) -> bool {
            unsafe { v8cxx__value_is_async_function(self as *const _ as *const _) }
        }

        fn is_generator_function(&self) -> bool {
            unsafe { v8cxx__value_is_generator_function(self as *const _ as *const _) }
        }

        fn is_generator_object(&self) -> bool {
            unsafe { v8cxx__value_is_generator_object(self as *const _ as *const _) }
        }

        fn is_promise(&self) -> bool {
            unsafe { v8cxx__value_is_promise(self as *const _ as *const _) }
        }

        fn is_map(&self) -> bool {
            unsafe { v8cxx__value_is_map(self as *const _ as *const _) }
        }

        fn is_set(&self) -> bool {
            unsafe { v8cxx__value_is_set(self as *const _ as *const _) }
        }

        fn is_map_iterator(&self) -> bool {
            unsafe { v8cxx__value_is_map_iterator(self as *const _ as *const _) }
        }

        fn is_set_iterator(&self) -> bool {
            unsafe { v8cxx__value_is_set_iterator(self as *const _ as *const _) }
        }

        fn is_weak_map(&self) -> bool {
            unsafe { v8cxx__value_is_weak_map(self as *const _ as *const _) }
        }

        fn is_weak_set(&self) -> bool {
            unsafe { v8cxx__value_is_weak_set(self as *const _ as *const _) }
        }

        fn is_weak_ref(&self) -> bool {
            unsafe { v8cxx__value_is_weak_ref(self as *const _ as *const _) }
        }

        fn is_array_buffer(&self) -> bool {
            unsafe { v8cxx__value_is_array_buffer(self as *const _ as *const _) }
        }

        fn is_array_buffer_view(&self) -> bool {
            unsafe { v8cxx__value_is_array_buffer_view(self as *const _ as *const _) }
        }

        fn is_typed_array(&self) -> bool {
            unsafe { v8cxx__value_is_typed_array(self as *const _ as *const _) }
        }

        fn is_uint8_array(&self) -> bool {
            unsafe { v8cxx__value_is_uint8_array(self as *const _ as *const _) }
        }

        fn is_uint8_clamped_array(&self) -> bool {
            unsafe { v8cxx__value_is_uint8_clamped_array(self as *const _ as *const _) }
        }

        fn is_int8_array(&self) -> bool {
            unsafe { v8cxx__value_is_int8_array(self as *const _ as *const _) }
        }

        fn is_uint16_array(&self) -> bool {
            unsafe { v8cxx__value_is_uint16_array(self as *const _ as *const _) }
        }

        fn is_int16_array(&self) -> bool {
            unsafe { v8cxx__value_is_int16_array(self as *const _ as *const _) }
        }

        fn is_uint32_array(&self) -> bool {
            unsafe { v8cxx__value_is_uint32_array(self as *const _ as *const _) }
        }

        fn is_int32_array(&self) -> bool {
            unsafe { v8cxx__value_is_int32_array(self as *const _ as *const _) }
        }

        fn is_float16_array(&self) -> bool {
            unsafe { v8cxx__value_is_float16_array(self as *const _ as *const _) }
        }

        fn is_float32_array(&self) -> bool {
            unsafe { v8cxx__value_is_float32_array(self as *const _ as *const _) }
        }

        fn is_float64_array(&self) -> bool {
            unsafe { v8cxx__value_is_float64_array(self as *const _ as *const _) }
        }

        fn is_big_int64_array(&self) -> bool {
            unsafe { v8cxx__value_is_big_int64_array(self as *const _ as *const _) }
        }

        fn is_big_uint64_array(&self) -> bool {
            unsafe { v8cxx__value_is_big_uint64_array(self as *const _ as *const _) }
        }

        fn is_data_view(&self) -> bool {
            unsafe { v8cxx__value_is_data_view(self as *const _ as *const _) }
        }

        fn is_shared_array_buffer(&self) -> bool {
            unsafe { v8cxx__value_is_shared_array_buffer(self as *const _ as *const _) }
        }

        fn is_proxy(&self) -> bool {
            unsafe { v8cxx__value_is_proxy(self as *const _ as *const _) }
        }

        fn is_wasm_memory_object(&self) -> bool {
            unsafe { v8cxx__value_is_wasm_memory_object(self as *const _ as *const _) }
        }

        fn is_wasm_module_object(&self) -> bool {
            unsafe { v8cxx__value_is_wasm_module_object(self as *const _ as *const _) }
        }

        fn is_wasm_null(&self) -> bool {
            unsafe { v8cxx__value_is_wasm_null(self as *const _ as *const _) }
        }

        fn is_module_namespace_object(&self) -> bool {
            unsafe { v8cxx__value_is_module_namespace_object(self as *const _ as *const _) }
        }

        fn to_primitive(&self, context: &Local<Context>) -> Local<Primitive> {
            let mut local_primitive = Local::<Primitive>::empty();

            unsafe {
                v8cxx__value_to_primitive(
                    &mut local_primitive,
                    self as *const _ as *const _,
                    context,
                );
            }

            local_primitive
        }

        fn to_bigint(&self, context: &Local<Context>) -> Local<BigInt> {
            let mut local_primitive = Local::<BigInt>::empty();

            unsafe {
                v8cxx__value_to_bigint(&mut local_primitive, self as *const _ as *const _, context);
            }

            local_primitive
        }

        fn to_number(&self, context: &Local<Context>) -> Local<Number> {
            let mut local_primitive = Local::<Number>::empty();

            unsafe {
                v8cxx__value_to_number(&mut local_primitive, self as *const _ as *const _, context);
            }

            local_primitive
        }

        fn to_string(&self, context: &Local<Context>) -> Local<String> {
            let mut local_primitive = Local::<String>::empty();

            unsafe {
                v8cxx__value_to_string(&mut local_primitive, self as *const _ as *const _, context);
            }

            local_primitive
        }

        fn to_object(&self, context: &Local<Context>) -> Local<Object> {
            let mut local_primitive = Local::<Object>::empty();

            unsafe {
                v8cxx__value_to_object(&mut local_primitive, self as *const _ as *const _, context);
            }

            local_primitive
        }

        fn to_boolean(&self, context: &Local<Context>) -> Local<Boolean> {
            let mut local_primitive = Local::<Boolean>::empty();

            unsafe {
                v8cxx__value_to_boolean(
                    &mut local_primitive,
                    self as *const _ as *const _,
                    context,
                );
            }

            local_primitive
        }

        fn type_of(&mut self, isolate: &mut Isolate) -> Local<String> {
            let mut local_string = Local::<String>::empty();

            unsafe {
                v8cxx__value_typeof(&mut local_string, self as *mut _ as *mut _, isolate);
            }

            local_string
        }

        fn instance_of(&self, context: &Local<Context>, object: &Local<Object>) -> bool {
            unsafe { v8cxx__value_instanceof(self as *const _ as *const _, context, object) }
        }
    }
}
