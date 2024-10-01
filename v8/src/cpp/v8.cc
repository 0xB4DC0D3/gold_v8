#include <memory>
#include <iostream>

#include "libplatform/libplatform.h"
#include "v8.h"

// v8::platform
extern "C"
{
    v8::Platform *v8cxx__platform__new_default_platform(
        int thread_pool_size,
        v8::platform::IdleTaskSupport idle_task_support,
        v8::platform::InProcessStackDumping in_process_stack_dumping,
        v8::platform::PriorityMode priority_mode)
    {
        return v8::platform::NewDefaultPlatform(thread_pool_size, idle_task_support, in_process_stack_dumping, nullptr, priority_mode).release();
    }
}

// v8::Platform
extern "C"
{
    void v8cxx__platform_drop(v8::Platform *self)
    {
        delete self;
    }
}

// v8::V8
extern "C"
{
    void v8cxx__v8__initialize_platform(v8::Platform *platform)
    {
        v8::V8::InitializePlatform(platform);
    }

    bool v8cxx__v8__initialize()
    {
        return v8::V8::Initialize();
    }

    void v8cxx__v8__dispose_platform()
    {
        v8::V8::DisposePlatform();
    }

    bool v8cxx__v8__dispose()
    {
        return v8::V8::Dispose();
    }
}

// v8::ArrayBuffer
extern "C"
{
    v8::ArrayBuffer::Allocator *v8cxx__arraybuffer__allocator_new_default_allocator()
    {
        return v8::ArrayBuffer::Allocator::NewDefaultAllocator();
    }
}

// v8::Isolate
extern "C"
{
    v8::Isolate::CreateParams v8cxx__isolate__createparams(v8::ArrayBuffer::Allocator *array_buffer_allocator)
    {
        v8::Isolate::CreateParams create_params;
        create_params.array_buffer_allocator = array_buffer_allocator;

        return create_params;
    }

    v8::Isolate *v8cxx__isolate_new(const v8::Isolate::CreateParams &create_params)
    {
        return v8::Isolate::New(create_params);
    }

    v8::Isolate *v8cxx__isolate_get_current(v8::Isolate *isolate)
    {
        return isolate->GetCurrent();
    }

    void v8cxx__isolate_get_current_context(v8::Local<v8::Context> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Context>(isolate->GetCurrentContext());
    }

    void v8cxx__isolate_enter(v8::Isolate *isolate)
    {
        isolate->Enter();
    }

    void v8cxx__isolate_exit(v8::Isolate *isolate)
    {
        isolate->Exit();
    }
}

// v8::HandleScope
extern "C"
{
    class HandleScope
    {
    public:
        HandleScope(v8::Isolate *isolate) : handle_scope(isolate) {}
        ~HandleScope() {}

        v8::HandleScope &get()
        {
            return this->handle_scope;
        }

    private:
        v8::HandleScope handle_scope;
    };

    HandleScope *v8cxx__handlescope_new(v8::Isolate *isolate)
    {
        return new HandleScope(isolate);
    }

    void v8cxx__handlescope_drop(HandleScope *handle_scope)
    {
        delete handle_scope;
    }

    v8::Isolate *v8cxx__handlescope_get_isolate(HandleScope *handle_scope)
    {
        return handle_scope->get().GetIsolate();
    }
}

// v8::Context
extern "C"
{
    void v8cxx__context_new(v8::Local<v8::Context> *local_buf, v8::Isolate *isolate)
    {
        // TODO: new (local_buf) v8::Local<v8::Context(v8::Context::New(isolate, nullptr, global_template, global_object, v8::DeserializeInternalFieldsCallback(), microtask_queue));
        new (local_buf) v8::Local<v8::Context>(v8::Context::New(isolate));
    }

    void v8cxx__context_enter(v8::Context *context)
    {
        context->Enter();
    }

    void v8cxx__context_exit(v8::Context *context)
    {
        context->Exit();
    }

    v8::Isolate *v8cxx__context_get_isolate(v8::Context *context)
    {
        return context->GetIsolate();
    }
}

// v8::Data
extern "C"
{
    bool v8cxx__data_is_context(const v8::Data &data)
    {
        return data.IsContext();
    }

    bool v8cxx__data_is_fixed_array(const v8::Data &data)
    {
        return data.IsFixedArray();
    }

    bool v8cxx__data_is_function_template(const v8::Data &data)
    {
        return data.IsFunctionTemplate();
    }

    bool v8cxx__data_is_module(const v8::Data &data)
    {
        return data.IsModule();
    }

    bool v8cxx__data_is_object_template(const v8::Data &data)
    {
        return data.IsObjectTemplate();
    }

    bool v8cxx__data_is_private(const v8::Data &data)
    {
        return data.IsPrivate();
    }

    bool v8cxx__data_is_value(const v8::Data &data)
    {
        return data.IsValue();
    }
}

// v8::Value
extern "C"
{
    bool v8cxx__value_is_undefined(const v8::Value &value)
    {
        return value.IsUndefined();
    }

    bool v8cxx__value_is_null(const v8::Value &value)
    {
        return value.IsNull();
    }

    bool v8cxx__value_is_null_or_undefined(const v8::Value &value)
    {
        return value.IsNullOrUndefined();
    }

    bool v8cxx__value_is_true(const v8::Value &value)
    {
        return value.IsTrue();
    }

    bool v8cxx__value_is_false(const v8::Value &value)
    {
        return value.IsFalse();
    }

    bool v8cxx__value_is_name(const v8::Value &value)
    {
        return value.IsName();
    }

    bool v8cxx__value_is_string(const v8::Value &value)
    {
        return value.IsString();
    }

    bool v8cxx__value_is_symbol(const v8::Value &value)
    {
        return value.IsSymbol();
    }

    bool v8cxx__value_is_function(const v8::Value &value)
    {
        return value.IsFunction();
    }

    bool v8cxx__value_is_array(const v8::Value &value)
    {
        return value.IsArray();
    }

    bool v8cxx__value_is_object(const v8::Value &value)
    {
        return value.IsObject();
    }

    bool v8cxx__value_is_bigint(const v8::Value &value)
    {
        return value.IsBigInt();
    }

    bool v8cxx__value_is_boolean(const v8::Value &value)
    {
        return value.IsBoolean();
    }

    bool v8cxx__value_is_number(const v8::Value &value)
    {
        return value.IsNumber();
    }

    bool v8cxx__value_is_external(const v8::Value &value)
    {
        return value.IsExternal();
    }

    bool v8cxx__value_is_int32(const v8::Value &value)
    {
        return value.IsInt32();
    }

    bool v8cxx__value_is_uint32(const v8::Value &value)
    {
        return value.IsUint32();
    }

    bool v8cxx__value_is_date(const v8::Value &value)
    {
        return value.IsDate();
    }

    bool v8cxx__value_is_arguments_object(const v8::Value &value)
    {
        return value.IsArgumentsObject();
    }

    bool v8cxx__value_is_bigint_object(const v8::Value &value)
    {
        return value.IsBigIntObject();
    }

    bool v8cxx__value_is_boolean_object(const v8::Value &value)
    {
        return value.IsBooleanObject();
    }

    bool v8cxx__value_is_number_object(const v8::Value &value)
    {
        return value.IsNumberObject();
    }

    bool v8cxx__value_is_string_object(const v8::Value &value)
    {
        return value.IsStringObject();
    }

    bool v8cxx__value_is_symbol_object(const v8::Value &value)
    {
        return value.IsSymbolObject();
    }

    bool v8cxx__value_is_native_error(const v8::Value &value)
    {
        return value.IsNativeError();
    }

    bool v8cxx__value_is_regexp(const v8::Value &value)
    {
        return value.IsRegExp();
    }

    bool v8cxx__value_is_async_function(const v8::Value &value)
    {
        return value.IsAsyncFunction();
    }

    bool v8cxx__value_is_generator_function(const v8::Value &value)
    {
        return value.IsGeneratorFunction();
    }

    bool v8cxx__value_is_generator_object(const v8::Value &value)
    {
        return value.IsGeneratorObject();
    }

    bool v8cxx__value_is_promise(const v8::Value &value)
    {
        return value.IsPromise();
    }

    bool v8cxx__value_is_map(const v8::Value &value)
    {
        return value.IsMap();
    }

    bool v8cxx__value_is_set(const v8::Value &value)
    {
        return value.IsSet();
    }

    bool v8cxx__value_is_map_iterator(const v8::Value &value)
    {
        return value.IsMapIterator();
    }

    bool v8cxx__value_is_set_iterator(const v8::Value &value)
    {
        return value.IsSetIterator();
    }

    bool v8cxx__value_is_weak_map(const v8::Value &value)
    {
        return value.IsWeakMap();
    }

    bool v8cxx__value_is_weak_set(const v8::Value &value)
    {
        return value.IsWeakSet();
    }

    bool v8cxx__value_is_weak_ref(const v8::Value &value)
    {
        return value.IsWeakRef();
    }

    bool v8cxx__value_is_array_buffer(const v8::Value &value)
    {
        return value.IsArrayBuffer();
    }

    bool v8cxx__value_is_array_buffer_view(const v8::Value &value)
    {
        return value.IsArrayBufferView();
    }

    bool v8cxx__value_is_typed_array(const v8::Value &value)
    {
        return value.IsTypedArray();
    }

    bool v8cxx__value_is_uint8_array(const v8::Value &value)
    {
        return value.IsUint8Array();
    }

    bool v8cxx__value_is_uint8_clamped_array(const v8::Value &value)
    {
        return value.IsUint8ClampedArray();
    }

    bool v8cxx__value_is_int8_array(const v8::Value &value)
    {
        return value.IsInt8Array();
    }

    bool v8cxx__value_is_uint16_array(const v8::Value &value)
    {
        return value.IsUint16Array();
    }

    bool v8cxx__value_is_int16_array(const v8::Value &value)
    {
        return value.IsInt16Array();
    }

    bool v8cxx__value_is_uint32_array(const v8::Value &value)
    {
        return value.IsUint32Array();
    }

    bool v8cxx__value_is_int32_array(const v8::Value &value)
    {
        return value.IsInt32Array();
    }

    bool v8cxx__value_is_float16_array(const v8::Value &value)
    {
        return value.IsFloat16Array();
    }

    bool v8cxx__value_is_float32_array(const v8::Value &value)
    {
        return value.IsFloat32Array();
    }

    bool v8cxx__value_is_float64_array(const v8::Value &value)
    {
        return value.IsFloat64Array();
    }

    bool v8cxx__value_is_bigint64_array(const v8::Value &value)
    {
        return value.IsBigInt64Array();
    }

    bool v8cxx__value_is_biguint64_array(const v8::Value &value)
    {
        return value.IsBigUint64Array();
    }

    bool v8cxx__value_is_data_view(const v8::Value &value)
    {
        return value.IsDataView();
    }

    bool v8cxx__value_is_shared_array_buffer(const v8::Value &value)
    {
        return value.IsSharedArrayBuffer();
    }

    bool v8cxx__value_is_proxy(const v8::Value &value)
    {
        return value.IsProxy();
    }

    bool v8cxx__value_is_wasm_memory_object(const v8::Value &value)
    {
        return value.IsWasmMemoryObject();
    }

    bool v8cxx__value_is_wasm_module_object(const v8::Value &value)
    {
        return value.IsWasmModuleObject();
    }

    bool v8cxx__value_is_wasm_null(const v8::Value &value)
    {
        return value.IsWasmNull();
    }

    bool v8cxx__value_is_module_namespace_object(const v8::Value &value)
    {
        return value.IsModuleNamespaceObject();
    }

    void v8cxx__value_to_primitive(v8::Local<v8::Primitive> *local_buf, const v8::Value &value, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::Primitive>(value.ToPrimitive(*context).ToLocalChecked());
    }

    void v8cxx__value_to_bigint(v8::Local<v8::BigInt> *local_buf, const v8::Value &value, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::BigInt>(value.ToBigInt(*context).ToLocalChecked());
    }

    void v8cxx__value_to_number(v8::Local<v8::Number> *local_buf, const v8::Value &value, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::Number>(value.ToNumber(*context).ToLocalChecked());
    }

    void v8cxx__value_to_string(v8::Local<v8::String> *local_buf, const v8::Value &value, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::String>(value.ToString(*context).ToLocalChecked());
    }

    void v8cxx__value_to_object(v8::Local<v8::Object> *local_buf, const v8::Value &value, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::Object>(value.ToObject(*context).ToLocalChecked());
    }

    void v8cxx__value_to_boolean(v8::Local<v8::Boolean> *local_buf, const v8::Value &value, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Boolean>(value.ToBoolean(isolate));
    }

    void v8cxx__value_typeof(v8::Local<v8::String> *local_buf, v8::Value &value, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::String>(value.TypeOf(isolate));
    }

    bool v8cxx__value_instanceof(v8::Value &value, const v8::Local<v8::Context> *context, const v8::Local<v8::Object> *object)
    {
        return value.InstanceOf(*context, *object).ToChecked();
    }
}

// v8::Primitive
extern "C"
{
    void v8cxx__primitive_undefined(v8::Local<v8::Primitive> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Primitive>(v8::Undefined(isolate));
    }

    void v8cxx__primitive_null(v8::Local<v8::Primitive> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Primitive>(v8::Null(isolate));
    }
}

// v8::Boolean
extern "C"
{
    void v8cxx__boolean_new(v8::Local<v8::Boolean> *local_buf, v8::Isolate *isolate, bool value)
    {
        new (local_buf) v8::Local<v8::Boolean>(value ? v8::True(isolate) : v8::False(isolate));
    }

    bool v8cxx__boolean_value(const v8::Boolean *boolean)
    {
        return boolean->Value();
    }
}

// v8::String
extern "C"
{
    void v8cxx__string_new_from_utf8(v8::Local<v8::String> *local_buf, v8::Isolate *isolate, const char *value, v8::NewStringType type, int length)
    {
        new (local_buf) v8::Local<v8::String>(v8::String::NewFromUtf8(isolate, value, type, length).ToLocalChecked());
    }

    void v8cxx__string_new_from_onebyte(v8::Local<v8::String> *local_buf, v8::Isolate *isolate, const uint8_t *value, v8::NewStringType type, int length)
    {
        new (local_buf) v8::Local<v8::String>(v8::String::NewFromOneByte(isolate, value, type, length).ToLocalChecked());
    }

    void v8cxx__string_new_from_twobyte(v8::Local<v8::String> *local_buf, v8::Isolate *isolate, const uint16_t *value, v8::NewStringType type, int length)
    {
        new (local_buf) v8::Local<v8::String>(v8::String::NewFromTwoByte(isolate, value, type, length).ToLocalChecked());
    }

    int v8cxx__string_length(const v8::String &string)
    {
        return string.Length();
    }

    int v8cxx__string_utf8length(const v8::String &string, v8::Isolate *isolate)
    {
        return string.Utf8Length(isolate);
    }

    bool v8cxx__string_is_onebyte(const v8::String &string)
    {
        return string.IsOneByte();
    }

    bool v8cxx__string_contains_only_onebyte(const v8::String &string)
    {
        return string.ContainsOnlyOneByte();
    }

    bool v8cxx__string_is_external(const v8::String &string)
    {
        return string.IsExternal();
    }

    bool v8cxx__string_is_external_twobyte(const v8::String &string)
    {
        return string.IsExternalTwoByte();
    }

    bool v8cxx__string_is_external_onebyte(const v8::String &string)
    {
        return string.IsExternalOneByte();
    }

    void v8cxx__string_internalize_string(v8::Local<v8::String> *local_buf, v8::String &string, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::String>(string.InternalizeString(isolate));
    }

    const char *v8cxx__string_view(const v8::String *string, v8::Isolate *isolate)
    {
        auto local_string = string->ToString(isolate->GetCurrentContext()).ToLocalChecked();
        return *v8::String::Utf8Value(isolate, local_string);
    }
}

// v8::Symbol
extern "C"
{
    void v8cxx__symbol_new(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate, const v8::Local<v8::String> *name)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::New(isolate, *name));
    }

    void v8cxx__symbol_for(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate, const v8::Local<v8::String> *description)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::For(isolate, *description));
    }

    void v8cxx__symbol_for_api(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate, const v8::Local<v8::String> *description)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::For(isolate, *description));
    }

    void v8cxx__symbol_get_async_iterator(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetAsyncIterator(isolate));
    }

    void v8cxx__symbol_get_has_instance(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetHasInstance(isolate));
    }

    void v8cxx__symbol_get_is_concat_spreadable(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetIsConcatSpreadable(isolate));
    }

    void v8cxx__symbol_get_iterator(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetIterator(isolate));
    }

    void v8cxx__symbol_get_match(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetMatch(isolate));
    }

    void v8cxx__symbol_get_replace(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetReplace(isolate));
    }

    void v8cxx__symbol_get_search(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetSearch(isolate));
    }

    void v8cxx__symbol_get_split(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetSplit(isolate));
    }

    void v8cxx__symbol_get_to_primitive(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetToPrimitive(isolate));
    }

    void v8cxx__symbol_get_to_string_tag(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetToStringTag(isolate));
    }

    void v8cxx__symbol_get_unscopables(v8::Local<v8::Symbol> *local_buf, v8::Isolate *isolate)
    {
        new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetUnscopables(isolate));
    }
}

// v8::BigInt
extern "C"
{
    void v8cxx__bigint_new(v8::Local<v8::BigInt> *local_buf, v8::Isolate *isolate, int64_t value)
    {
        new (local_buf) v8::Local<v8::BigInt>(v8::BigInt::New(isolate, value));
    }
}

// v8::Script
extern "C"
{
    void v8cxx__script_compile(v8::Local<v8::Script> *local_buf, const v8::Local<v8::Context> *context, const v8::Local<v8::String> *source)
    {
        new (local_buf) v8::Local<v8::Script>(v8::Script::Compile(*context, *source).ToLocalChecked());
    }

    void v8cxx__script_run(v8::Local<v8::Value> *local_buf, v8::Script *script, const v8::Local<v8::Context> *context)
    {
        new (local_buf) v8::Local<v8::Value>(script->Run(*context).ToLocalChecked());
    }
}