#include <iostream>
#include <memory>

#include "libplatform/libplatform.h"
#include "v8.h"

// v8::platform
extern "C" {
v8::Platform* v8cxx__platform__new_default_platform(
    int thread_pool_size,
    v8::platform::IdleTaskSupport idle_task_support,
    v8::platform::InProcessStackDumping in_process_stack_dumping,
    v8::platform::PriorityMode priority_mode) {
  return v8::platform::NewDefaultPlatform(thread_pool_size, idle_task_support,
                                          in_process_stack_dumping, nullptr,
                                          priority_mode)
      .release();
}
}

// v8::Platform
extern "C" {
void v8cxx__platform_drop(v8::Platform* self) {
  delete self;
}
}

// v8::V8
extern "C" {
void v8cxx__v8__initialize_platform(v8::Platform* platform) {
  v8::V8::InitializePlatform(platform);
}

bool v8cxx__v8__initialize() {
  return v8::V8::Initialize();
}

void v8cxx__v8__dispose_platform() {
  v8::V8::DisposePlatform();
}

bool v8cxx__v8__dispose() {
  return v8::V8::Dispose();
}
}

// v8::ArrayBuffer
extern "C" {
v8::ArrayBuffer::Allocator*
v8cxx__arraybuffer__allocator_new_default_allocator() {
  return v8::ArrayBuffer::Allocator::NewDefaultAllocator();
}
}

// v8::Isolate
extern "C" {
v8::Isolate::CreateParams v8cxx__isolate__createparams(
    v8::ArrayBuffer::Allocator* array_buffer_allocator) {
  v8::Isolate::CreateParams create_params;
  create_params.array_buffer_allocator = array_buffer_allocator;

  return create_params;
}

v8::Isolate* v8cxx__isolate_new(
    const v8::Isolate::CreateParams& create_params) {
  return v8::Isolate::New(create_params);
}

v8::Isolate* v8cxx__isolate_get_current() {
  return v8::Isolate::GetCurrent();
}

void v8cxx__isolate_get_current_context(v8::Local<v8::Context>* local_buf,
                                        v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Context>(isolate->GetCurrentContext());
}

void v8cxx__isolate_enter(v8::Isolate* isolate) {
  isolate->Enter();
}

void v8cxx__isolate_exit(v8::Isolate* isolate) {
  isolate->Exit();
}
}

// v8::HandleScope
extern "C" {
class HandleScope {
 public:
  HandleScope(v8::Isolate* isolate) : handle_scope(isolate) {}
  ~HandleScope() {}

  const v8::HandleScope& get() const { return this->handle_scope; }

 private:
  v8::HandleScope handle_scope;
};

void v8cxx__handlescope_new(HandleScope* buf, v8::Isolate* isolate) {
  new (buf) HandleScope(isolate);
}

void v8cxx__handlescope_drop(HandleScope* handle_scope) {
  handle_scope->~HandleScope();
}

v8::Isolate* v8cxx__handlescope_get_isolate(const HandleScope* handle_scope) {
  return handle_scope->get().GetIsolate();
}
}

// v8::Context
extern "C" {
void v8cxx__context_new(v8::Local<v8::Context>* local_buf,
                        v8::Isolate* isolate,
                        const v8::Local<v8::ObjectTemplate>* global_template,
                        const v8::Local<v8::Value>* global_object,
                        v8::MicrotaskQueue* microtask_queue) {
  new (local_buf) v8::Local<v8::Context>(v8::Context::New(
      isolate, nullptr,
      global_template == nullptr ? v8::MaybeLocal<v8::ObjectTemplate>()
                                 : *global_template,
      global_object == nullptr ? v8::MaybeLocal<v8::Value>() : *global_object,
      v8::DeserializeInternalFieldsCallback(), microtask_queue));
}

void v8cxx__context_enter(v8::Context* context) {
  context->Enter();
}

void v8cxx__context_exit(v8::Context* context) {
  context->Exit();
}

void v8cxx__context_global(v8::Local<v8::Object>* local_buf,
                           v8::Context* context) {
  new (local_buf) v8::Local<v8::Object>(context->Global());
}

void v8cxx__context_detach_global(v8::Context* context) {
  context->DetachGlobal();
}

v8::Isolate* v8cxx__context_get_isolate(v8::Context* context) {
  return context->GetIsolate();
}

v8::MicrotaskQueue* v8cxx__context_get_microtask_queue(v8::Context* context) {
  return context->GetMicrotaskQueue();
}

void v8cxx__context_set_microtask_queue(v8::Context* context,
                                        v8::MicrotaskQueue* microtask_queue) {
  context->SetMicrotaskQueue(microtask_queue);
}
}

// v8::Data
extern "C" {
bool v8cxx__data_is_context(const v8::Data& data) {
  return data.IsContext();
}

bool v8cxx__data_is_fixed_array(const v8::Data& data) {
  return data.IsFixedArray();
}

bool v8cxx__data_is_function_template(const v8::Data& data) {
  return data.IsFunctionTemplate();
}

bool v8cxx__data_is_module(const v8::Data& data) {
  return data.IsModule();
}

bool v8cxx__data_is_object_template(const v8::Data& data) {
  return data.IsObjectTemplate();
}

bool v8cxx__data_is_private(const v8::Data& data) {
  return data.IsPrivate();
}

bool v8cxx__data_is_value(const v8::Data& data) {
  return data.IsValue();
}
}

// v8::Value
extern "C" {
bool v8cxx__value_is_undefined(const v8::Value& value) {
  return value.IsUndefined();
}

bool v8cxx__value_is_null(const v8::Value& value) {
  return value.IsNull();
}

bool v8cxx__value_is_null_or_undefined(const v8::Value& value) {
  return value.IsNullOrUndefined();
}

bool v8cxx__value_is_true(const v8::Value& value) {
  return value.IsTrue();
}

bool v8cxx__value_is_false(const v8::Value& value) {
  return value.IsFalse();
}

bool v8cxx__value_is_name(const v8::Value& value) {
  return value.IsName();
}

bool v8cxx__value_is_string(const v8::Value& value) {
  return value.IsString();
}

bool v8cxx__value_is_symbol(const v8::Value& value) {
  return value.IsSymbol();
}

bool v8cxx__value_is_function(const v8::Value& value) {
  return value.IsFunction();
}

bool v8cxx__value_is_array(const v8::Value& value) {
  return value.IsArray();
}

bool v8cxx__value_is_object(const v8::Value& value) {
  return value.IsObject();
}

bool v8cxx__value_is_bigint(const v8::Value& value) {
  return value.IsBigInt();
}

bool v8cxx__value_is_boolean(const v8::Value& value) {
  return value.IsBoolean();
}

bool v8cxx__value_is_number(const v8::Value& value) {
  return value.IsNumber();
}

bool v8cxx__value_is_external(const v8::Value& value) {
  return value.IsExternal();
}

bool v8cxx__value_is_int32(const v8::Value& value) {
  return value.IsInt32();
}

bool v8cxx__value_is_uint32(const v8::Value& value) {
  return value.IsUint32();
}

bool v8cxx__value_is_date(const v8::Value& value) {
  return value.IsDate();
}

bool v8cxx__value_is_arguments_object(const v8::Value& value) {
  return value.IsArgumentsObject();
}

bool v8cxx__value_is_bigint_object(const v8::Value& value) {
  return value.IsBigIntObject();
}

bool v8cxx__value_is_boolean_object(const v8::Value& value) {
  return value.IsBooleanObject();
}

bool v8cxx__value_is_number_object(const v8::Value& value) {
  return value.IsNumberObject();
}

bool v8cxx__value_is_string_object(const v8::Value& value) {
  return value.IsStringObject();
}

bool v8cxx__value_is_symbol_object(const v8::Value& value) {
  return value.IsSymbolObject();
}

bool v8cxx__value_is_native_error(const v8::Value& value) {
  return value.IsNativeError();
}

bool v8cxx__value_is_regexp(const v8::Value& value) {
  return value.IsRegExp();
}

bool v8cxx__value_is_async_function(const v8::Value& value) {
  return value.IsAsyncFunction();
}

bool v8cxx__value_is_generator_function(const v8::Value& value) {
  return value.IsGeneratorFunction();
}

bool v8cxx__value_is_generator_object(const v8::Value& value) {
  return value.IsGeneratorObject();
}

bool v8cxx__value_is_promise(const v8::Value& value) {
  return value.IsPromise();
}

bool v8cxx__value_is_map(const v8::Value& value) {
  return value.IsMap();
}

bool v8cxx__value_is_set(const v8::Value& value) {
  return value.IsSet();
}

bool v8cxx__value_is_map_iterator(const v8::Value& value) {
  return value.IsMapIterator();
}

bool v8cxx__value_is_set_iterator(const v8::Value& value) {
  return value.IsSetIterator();
}

bool v8cxx__value_is_weak_map(const v8::Value& value) {
  return value.IsWeakMap();
}

bool v8cxx__value_is_weak_set(const v8::Value& value) {
  return value.IsWeakSet();
}

bool v8cxx__value_is_weak_ref(const v8::Value& value) {
  return value.IsWeakRef();
}

bool v8cxx__value_is_array_buffer(const v8::Value& value) {
  return value.IsArrayBuffer();
}

bool v8cxx__value_is_array_buffer_view(const v8::Value& value) {
  return value.IsArrayBufferView();
}

bool v8cxx__value_is_typed_array(const v8::Value& value) {
  return value.IsTypedArray();
}

bool v8cxx__value_is_uint8_array(const v8::Value& value) {
  return value.IsUint8Array();
}

bool v8cxx__value_is_uint8_clamped_array(const v8::Value& value) {
  return value.IsUint8ClampedArray();
}

bool v8cxx__value_is_int8_array(const v8::Value& value) {
  return value.IsInt8Array();
}

bool v8cxx__value_is_uint16_array(const v8::Value& value) {
  return value.IsUint16Array();
}

bool v8cxx__value_is_int16_array(const v8::Value& value) {
  return value.IsInt16Array();
}

bool v8cxx__value_is_uint32_array(const v8::Value& value) {
  return value.IsUint32Array();
}

bool v8cxx__value_is_int32_array(const v8::Value& value) {
  return value.IsInt32Array();
}

bool v8cxx__value_is_float16_array(const v8::Value& value) {
  return value.IsFloat16Array();
}

bool v8cxx__value_is_float32_array(const v8::Value& value) {
  return value.IsFloat32Array();
}

bool v8cxx__value_is_float64_array(const v8::Value& value) {
  return value.IsFloat64Array();
}

bool v8cxx__value_is_bigint64_array(const v8::Value& value) {
  return value.IsBigInt64Array();
}

bool v8cxx__value_is_biguint64_array(const v8::Value& value) {
  return value.IsBigUint64Array();
}

bool v8cxx__value_is_data_view(const v8::Value& value) {
  return value.IsDataView();
}

bool v8cxx__value_is_shared_array_buffer(const v8::Value& value) {
  return value.IsSharedArrayBuffer();
}

bool v8cxx__value_is_proxy(const v8::Value& value) {
  return value.IsProxy();
}

bool v8cxx__value_is_wasm_memory_object(const v8::Value& value) {
  return value.IsWasmMemoryObject();
}

bool v8cxx__value_is_wasm_module_object(const v8::Value& value) {
  return value.IsWasmModuleObject();
}

bool v8cxx__value_is_wasm_null(const v8::Value& value) {
  return value.IsWasmNull();
}

bool v8cxx__value_is_module_namespace_object(const v8::Value& value) {
  return value.IsModuleNamespaceObject();
}

void v8cxx__value_to_primitive(v8::MaybeLocal<v8::Primitive>* maybe_local_buf,
                               const v8::Value& value,
                               const v8::Local<v8::Context>* context) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Primitive>(value.ToPrimitive(*context));
}

void v8cxx__value_to_bigint(v8::MaybeLocal<v8::BigInt>* maybe_local_buf,
                            const v8::Value& value,
                            const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::BigInt>(value.ToBigInt(*context));
}

void v8cxx__value_to_number(v8::MaybeLocal<v8::Number>* maybe_local_buf,
                            const v8::Value& value,
                            const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Number>(value.ToNumber(*context));
}

void v8cxx__value_to_string(v8::MaybeLocal<v8::String>* maybe_local_buf,
                            const v8::Value& value,
                            const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::String>(value.ToString(*context));
}

void v8cxx__value_to_object(v8::MaybeLocal<v8::Object>* maybe_local_buf,
                            const v8::Value& value,
                            const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Object>(value.ToObject(*context));
}

void v8cxx__value_to_boolean(v8::Local<v8::Boolean>* local_buf,
                             const v8::Value& value,
                             v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Boolean>(value.ToBoolean(isolate));
}

void v8cxx__value_typeof(v8::Local<v8::String>* local_buf,
                         v8::Value& value,
                         v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::String>(value.TypeOf(isolate));
}

bool v8cxx__value_instanceof(v8::Value& value,
                             const v8::Local<v8::Context>* context,
                             const v8::Local<v8::Object>* object) {
  return value.InstanceOf(*context, *object).ToChecked();
}
}

// v8::Primitive
extern "C" {
void v8cxx__primitive_undefined(v8::Local<v8::Primitive>* local_buf,
                                v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Primitive>(v8::Undefined(isolate));
}

void v8cxx__primitive_null(v8::Local<v8::Primitive>* local_buf,
                           v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Primitive>(v8::Null(isolate));
}
}

// v8::Boolean
extern "C" {
void v8cxx__boolean_new(v8::Local<v8::Boolean>* local_buf,
                        v8::Isolate* isolate,
                        bool value) {
  new (local_buf)
      v8::Local<v8::Boolean>(value ? v8::True(isolate) : v8::False(isolate));
}

bool v8cxx__boolean_value(const v8::Boolean* boolean) {
  return boolean->Value();
}
}

// v8::String
extern "C" {
void v8cxx__string_new_from_utf8(v8::MaybeLocal<v8::String>* maybe_local_buf,
                                 v8::Isolate* isolate,
                                 const char* value,
                                 v8::NewStringType type,
                                 int length) {
  new (maybe_local_buf) v8::MaybeLocal<v8::String>(
      v8::String::NewFromUtf8(isolate, value, type, length));
}

void v8cxx__string_new_from_onebyte(v8::MaybeLocal<v8::String>* maybe_local_buf,
                                    v8::Isolate* isolate,
                                    const uint8_t* value,
                                    v8::NewStringType type,
                                    int length) {
  new (maybe_local_buf) v8::MaybeLocal<v8::String>(
      v8::String::NewFromOneByte(isolate, value, type, length));
}

void v8cxx__string_new_from_twobyte(v8::MaybeLocal<v8::String>* maybe_local_buf,
                                    v8::Isolate* isolate,
                                    const uint16_t* value,
                                    v8::NewStringType type,
                                    int length) {
  new (maybe_local_buf) v8::MaybeLocal<v8::String>(
      v8::String::NewFromTwoByte(isolate, value, type, length));
}

int v8cxx__string_length(const v8::String& string) {
  return string.Length();
}

int v8cxx__string_utf8length(const v8::String& string, v8::Isolate* isolate) {
  return string.Utf8Length(isolate);
}

bool v8cxx__string_is_onebyte(const v8::String& string) {
  return string.IsOneByte();
}

bool v8cxx__string_contains_only_onebyte(const v8::String& string) {
  return string.ContainsOnlyOneByte();
}

bool v8cxx__string_is_external(const v8::String& string) {
  return string.IsExternal();
}

bool v8cxx__string_is_external_twobyte(const v8::String& string) {
  return string.IsExternalTwoByte();
}

bool v8cxx__string_is_external_onebyte(const v8::String& string) {
  return string.IsExternalOneByte();
}

void v8cxx__string_internalize_string(v8::Local<v8::String>* local_buf,
                                      v8::String& string,
                                      v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::String>(string.InternalizeString(isolate));
}

const char* v8cxx__string_view(const v8::String* string, v8::Isolate* isolate) {
  auto local_string =
      string->ToString(isolate->GetCurrentContext()).ToLocalChecked();
  return *v8::String::Utf8Value(isolate, local_string);
}
}

// v8::Symbol
extern "C" {
void v8cxx__symbol_new(v8::Local<v8::Symbol>* local_buf,
                       v8::Isolate* isolate,
                       const v8::Local<v8::String>* name) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::New(isolate, *name));
}

void v8cxx__symbol_for(v8::Local<v8::Symbol>* local_buf,
                       v8::Isolate* isolate,
                       const v8::Local<v8::String>* description) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::For(isolate, *description));
}

void v8cxx__symbol_for_api(v8::Local<v8::Symbol>* local_buf,
                           v8::Isolate* isolate,
                           const v8::Local<v8::String>* description) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::For(isolate, *description));
}

void v8cxx__symbol_get_async_iterator(v8::Local<v8::Symbol>* local_buf,
                                      v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetAsyncIterator(isolate));
}

void v8cxx__symbol_get_has_instance(v8::Local<v8::Symbol>* local_buf,
                                    v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetHasInstance(isolate));
}

void v8cxx__symbol_get_is_concat_spreadable(v8::Local<v8::Symbol>* local_buf,
                                            v8::Isolate* isolate) {
  new (local_buf)
      v8::Local<v8::Symbol>(v8::Symbol::GetIsConcatSpreadable(isolate));
}

void v8cxx__symbol_get_iterator(v8::Local<v8::Symbol>* local_buf,
                                v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetIterator(isolate));
}

void v8cxx__symbol_get_match(v8::Local<v8::Symbol>* local_buf,
                             v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetMatch(isolate));
}

void v8cxx__symbol_get_replace(v8::Local<v8::Symbol>* local_buf,
                               v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetReplace(isolate));
}

void v8cxx__symbol_get_search(v8::Local<v8::Symbol>* local_buf,
                              v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetSearch(isolate));
}

void v8cxx__symbol_get_split(v8::Local<v8::Symbol>* local_buf,
                             v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetSplit(isolate));
}

void v8cxx__symbol_get_to_primitive(v8::Local<v8::Symbol>* local_buf,
                                    v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetToPrimitive(isolate));
}

void v8cxx__symbol_get_to_string_tag(v8::Local<v8::Symbol>* local_buf,
                                     v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetToStringTag(isolate));
}

void v8cxx__symbol_get_unscopables(v8::Local<v8::Symbol>* local_buf,
                                   v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Symbol>(v8::Symbol::GetUnscopables(isolate));
}
}

// v8::BigInt
extern "C" {
void v8cxx__bigint_new(v8::Local<v8::BigInt>* local_buf,
                       v8::Isolate* isolate,
                       int64_t value) {
  new (local_buf) v8::Local<v8::BigInt>(v8::BigInt::New(isolate, value));
}
}

// v8::Script
extern "C" {
void v8cxx__script_compile(v8::MaybeLocal<v8::Script>* maybe_local_buf,
                           const v8::Local<v8::Context>* context,
                           const v8::Local<v8::String>* source) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Script>(v8::Script::Compile(*context, *source));
}

void v8cxx__script_run(v8::MaybeLocal<v8::Value>* maybe_local_buf,
                       v8::Script* script,
                       const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Value>(script->Run(*context));
}

void v8cxx__script_get_unbound_script(v8::Local<v8::UnboundScript>* local_buf,
                                      v8::Script* script) {
  new (local_buf) v8::Local<v8::UnboundScript>(script->GetUnboundScript());
}

void v8cxx__script_get_resource_name(v8::Local<v8::Value>* local_buf,
                                     v8::Script* script) {
  new (local_buf) v8::Local<v8::Value>(script->GetResourceName());
}

// TODO: add this to v8::script::Script (Rust)
void v8cxx__script_get_compile_hints_collector(
    v8::Local<v8::CompileHintsCollector>* local_buf,
    const v8::Script* script) {
  new (local_buf)
      v8::Local<v8::CompileHintsCollector>(script->GetCompileHintsCollector());
}
}

// v8::Name
extern "C" {
int v8cxx__name_get_identity_hash(v8::Name* name) {
  return name->GetIdentityHash();
}
}

// v8::Object
extern "C" {
void v8cxx__object_new(v8::Local<v8::Object>* local_buf, v8::Isolate* isolate) {
  new (local_buf) v8::Local<v8::Object>(v8::Object::New(isolate));
}

bool v8cxx__object_set(v8::Object* object,
                       const v8::Local<v8::Context>* context,
                       const v8::Local<v8::Value>* key,
                       const v8::Local<v8::Value>* value,
                       const v8::MaybeLocal<v8::Object>* receiver) {
  auto result = false;

  object->Set(*context, *key, *value, *receiver).FromMaybe(&result);

  return result;
}

bool v8cxx__object_set_indexed(v8::Object* object,
                               const v8::Local<v8::Context>* context,
                               uint32_t index,
                               const v8::Local<v8::Value>* value) {
  auto result = false;

  object->Set(*context, index, *value).FromMaybe(&result);

  return result;
}

bool v8cxx__object_create_data_property(v8::Object* object,
                                        const v8::Local<v8::Context>* context,
                                        const v8::Local<v8::Name>* key,
                                        const v8::Local<v8::Value>* value) {
  auto result = false;

  object->CreateDataProperty(*context, *key, *value).FromMaybe(&result);

  return result;
}

bool v8cxx__object_create_data_property_indexed(
    v8::Object* object,
    const v8::Local<v8::Context>* context,
    uint32_t index,
    const v8::Local<v8::Value>* value) {
  auto result = false;

  object->CreateDataProperty(*context, index, *value).FromMaybe(&result);

  return result;
}

bool v8cxx__object_define_own_property(v8::Object* object,
                                       const v8::Local<v8::Context>* context,
                                       const v8::Local<v8::Name>* key,
                                       const v8::Local<v8::Value>* value,
                                       v8::PropertyAttribute attributes) {
  auto result = false;

  object->DefineOwnProperty(*context, *key, *value, attributes)
      .FromMaybe(&result);

  return result;
}

// TODO: v8cxx__object_define_property

void v8cxx__object_get(v8::MaybeLocal<v8::Value>* maybe_local_buf,
                       v8::Object* object,
                       const v8::Local<v8::Context>* context,
                       const v8::Local<v8::Value>* key,
                       const v8::MaybeLocal<v8::Object>* receiver) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Value>(object->Get(*context, *key, *receiver));
}

void v8cxx__object_get_indexed(v8::MaybeLocal<v8::Value>* maybe_local_buf,
                               v8::Object* object,
                               const v8::Local<v8::Context>* context,
                               uint32_t index) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Value>(object->Get(*context, index));
}
}

// v8::FixedArray
extern "C" {
int v8cxx__fixed_array_length(const v8::FixedArray* fixed_array) {
  return fixed_array->Length();
}

void v8cxx__fixed_array_get(v8::Local<v8::Data>* local_buf,
                            const v8::FixedArray* fixed_array,
                            const v8::Local<v8::Context>* context,
                            int i) {
  new (local_buf) v8::Local<v8::Data>(fixed_array->Get(*context, i));
}
}

// v8::Module
extern "C" {
v8::Module::Status v8cxx__module_get_status(const v8::Module* module) {
  return module->GetStatus();
}

void v8cxx__module_get_exception(v8::Local<v8::Value>* local_buf,
                                 const v8::Module* module) {
  new (local_buf) v8::Local<v8::Value>(module->GetException());
}

void v8cxx__module_get_module_requests(v8::Local<v8::FixedArray>* local_buf,
                                       const v8::Module* module) {
  new (local_buf) v8::Local<v8::FixedArray>(module->GetModuleRequests());
}

v8::Location v8cxx__module_source_offset_to_location(const v8::Module* module,
                                                     int offset) {
  return module->SourceOffsetToLocation(offset);
}

int v8cxx__module_get_identity_hash(const v8::Module* module) {
  return module->GetIdentityHash();
}

bool v8cxx__module_instantiate_module(
    v8::Module* module,
    const v8::Local<v8::Context>* context,
    v8::Module::ResolveModuleCallback module_callback
    // v8::Module::ResolveSourceCallback source_callback
) {
  auto result = false;

  module->InstantiateModule(*context, module_callback, nullptr)
      .FromMaybe(&result);

  return result;
}

void v8cxx__module_evaluate(v8::MaybeLocal<v8::Value>* maybe_local_buf,
                            v8::Module* module,
                            const v8::Local<v8::Context>* context) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Value>(module->Evaluate(*context));
}

void v8cxx__module_get_module_namespace(v8::Local<v8::Value>* local_buf,
                                        v8::Module* module) {
  new (local_buf) v8::Local<v8::Value>(module->GetModuleNamespace());
}

// TODO: add this method for v8::module::Module (Rust)
void v8cxx__module_get_unbound_module_script(
    v8::Local<v8::UnboundModuleScript>* local_buf,
    v8::Module* module) {
  new (local_buf)
      v8::Local<v8::UnboundModuleScript>(module->GetUnboundModuleScript());
}

int v8cxx__module_script_id(const v8::Module* module) {
  return module->ScriptId();
}

bool v8cxx__module_is_graph_async(const v8::Module* module) {
  return module->IsGraphAsync();
}

bool v8cxx__module_has_top_level_await(const v8::Module* module) {
  return module->HasTopLevelAwait();
}

bool v8cxx__module_is_source_text_module(const v8::Module* module) {
  return module->IsSourceTextModule();
}

bool v8cxx__module_is_synthetic_module(const v8::Module* module) {
  return module->IsSyntheticModule();
}

void v8cxx__module_create_synthetic_module(
    v8::Local<v8::Module>* local_buf,
    v8::Isolate* isolate,
    const v8::Local<v8::String>* module_name,
    const v8::Local<v8::String>* export_names,
    size_t export_names_length,
    v8::Module::SyntheticModuleEvaluationSteps evaluation_steps) {
  new (local_buf) v8::Local<v8::Module>(v8::Module::CreateSyntheticModule(
      isolate, *module_name,
      v8::MemorySpan<const v8::Local<v8::String>>(export_names,
                                                  export_names_length),
      evaluation_steps));
}

bool v8cxx__module_set_synthetic_module_export(
    v8::Module* module,
    v8::Isolate* isolate,
    const v8::Local<v8::String>* export_name,
    const v8::Local<v8::Value>* export_value) {
  auto result = false;

  module->SetSyntheticModuleExport(isolate, *export_name, *export_value)
      .FromMaybe(&result);

  return result;
}

// TODO: v8::Module::GetStalledTopLevelAwaitMessages
}

// v8::Local
extern "C" {
void v8cxx__local_empty(v8::Local<v8::Data>* local_buf) {
  new (local_buf) v8::Local<v8::Data>();
}
}

// v8::MaybeLocal
extern "C" {
bool v8cxx__maybe_local_is_empty(const v8::MaybeLocal<v8::Data>* maybe_local) {
  return maybe_local->IsEmpty();
}

bool v8cxx__maybe_local_to_local(const v8::MaybeLocal<v8::Data>* maybe_local,
                                 v8::Local<v8::Data>* out) {
  return maybe_local->ToLocal(out);
}

void v8cxx__maybe_local_to_local_checked(
    v8::Local<v8::Data>* local_buf,
    v8::MaybeLocal<v8::Data>* maybe_local) {
  new (local_buf) v8::Local<v8::Data>(maybe_local->ToLocalChecked());
}

void v8cxx__maybe_local_from_maybe(v8::Local<v8::Data>* local_buf,
                                   const v8::MaybeLocal<v8::Data>* maybe_local,
                                   const v8::Local<v8::Data>* default_value) {
  new (local_buf) v8::Local<v8::Data>(maybe_local->FromMaybe(*default_value));
}
}

// v8::ModuleRequest
extern "C" {
void v8cxx__module_request_get_specifier(
    v8::Local<v8::String>* local_buf,
    const v8::ModuleRequest* module_request) {
  new (local_buf) v8::Local<v8::String>(module_request->GetSpecifier());
}

v8::ModuleImportPhase v8cxx__module_request_get_phase(
    const v8::ModuleRequest* module_request) {
  return module_request->GetPhase();
}

void v8cxx__module_request_get_import_attributes(
    v8::Local<v8::FixedArray>* local_buf,
    const v8::ModuleRequest* module_request) {
  new (local_buf)
      v8::Local<v8::FixedArray>(module_request->GetImportAttributes());
}

int v8cxx__module_request_get_source_offset(
    const v8::ModuleRequest* module_request) {
  return module_request->GetSourceOffset();
}
}

// v8::PrimitiveArray
extern "C" {
void v8cxx__primitive_array_new(v8::Local<v8::PrimitiveArray>* local_buf,
                                v8::Isolate* isolate,
                                int length) {
  new (local_buf)
      v8::Local<v8::PrimitiveArray>(v8::PrimitiveArray::New(isolate, length));
}

int v8cxx__primitive_array_length(const v8::PrimitiveArray* primitive_array) {
  return primitive_array->Length();
}

void v8cxx__primitive_array_set(v8::PrimitiveArray* primitive_array,
                                v8::Isolate* isolate,
                                int index,
                                const v8::Local<v8::Primitive>* item) {
  primitive_array->Set(isolate, index, *item);
}

void v8cxx__primitive_array_get(v8::Local<v8::Primitive>* local_buf,
                                v8::PrimitiveArray* primitive_array,
                                v8::Isolate* isolate,
                                int index) {
  new (local_buf)
      v8::Local<v8::Primitive>(primitive_array->Get(isolate, index));
}
}

// v8::Private
extern "C" {
void v8cxx__private_name(v8::Local<v8::Value>* local_buf,
                         const v8::Private* private_) {
  new (local_buf) v8::Local<v8::Value>(private_->Name());
}

void v8cxx__private_new(v8::Local<v8::Private>* local_buf,
                        v8::Isolate* isolate,
                        const v8::Local<v8::String>* name) {
  new (local_buf) v8::Local<v8::Private>(v8::Private::New(isolate, *name));
}

void v8cxx__(v8::Local<v8::Private>* local_buf,
             v8::Isolate* isolate,
             const v8::Local<v8::String>* name) {
  new (local_buf) v8::Local<v8::Private>(v8::Private::ForApi(isolate, *name));
}
}

// v8::Signature
extern "C" {
void v8cxx__signature_new(v8::Local<v8::Signature>* local_buf,
                          v8::Isolate* isolate,
                          const v8::Local<v8::FunctionTemplate>* receiver) {
  new (local_buf)
      v8::Local<v8::Signature>(v8::Signature::New(isolate, *receiver));
}
}

// v8::Template
extern "C" {
void v8cxx__template_set(v8::Template* template_,
                         const v8::Local<v8::Name>* name,
                         const v8::Local<v8::Data>* value,
                         v8::PropertyAttribute attributes) {
  template_->Set(*name, *value, attributes);
}

void v8cxx__template_set_private(v8::Template* template_,
                                 const v8::Local<v8::Private>* name,
                                 const v8::Local<v8::Data>* value,
                                 v8::PropertyAttribute attributes) {
  template_->SetPrivate(*name, *value, attributes);
}

void v8cxx__template_set_with_isolate(v8::Template* template_,
                                      v8::Isolate* isolate,
                                      const char* name,
                                      const v8::Local<v8::Data>* value,
                                      v8::PropertyAttribute attributes) {
  template_->Set(isolate, name, *value, attributes);
}

void v8cxx__template_set_accessor_property(
    v8::Template* template_,
    const v8::Local<v8::Name>* name,
    const v8::Local<v8::FunctionTemplate>* getter,
    const v8::Local<v8::FunctionTemplate>* setter,
    v8::PropertyAttribute attribute) {
  template_->SetAccessorProperty(*name, *getter, *setter, attribute);
}
}

// v8::FunctionTemplate
extern "C" {
void v8cxx__function_template_new(
    v8::Local<v8::FunctionTemplate>* local_buf,
    v8::Isolate* isolate,
    v8::FunctionCallback fn_callback,
    const v8::Local<v8::Value>* data,
    const v8::Local<v8::Signature>* signature,
    int length,
    v8::ConstructorBehavior behavior,
    v8::SideEffectType side_effect_type
    /* TODO: add CFunction and rest of params */) {
  new (local_buf) v8::Local<v8::FunctionTemplate>(
      v8::FunctionTemplate::New(isolate, fn_callback, *data, *signature, length,
                                behavior, side_effect_type));
}

void v8cxx__function_template_get_function(
    v8::MaybeLocal<v8::Function>* maybe_local_buf,
    v8::FunctionTemplate* fn_template,
    const v8::Local<v8::Context>* context) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Function>(fn_template->GetFunction(*context));
}

void v8cxx__function_template_instance_template(
    v8::Local<v8::ObjectTemplate>* local_buf,
    v8::FunctionTemplate* fn_template) {
  new (local_buf)
      v8::Local<v8::ObjectTemplate>(fn_template->InstanceTemplate());
}

void v8cxx__function_template_inherit(
    v8::FunctionTemplate* fn_template,
    const v8::Local<v8::FunctionTemplate>* parent) {
  fn_template->Inherit(*parent);
}

void v8cxx__function_template_prototype_template(
    v8::Local<v8::ObjectTemplate>* local_buf,
    v8::FunctionTemplate* fn_template) {
  new (local_buf)
      v8::Local<v8::ObjectTemplate>(fn_template->PrototypeTemplate());
}

void v8cxx__function_template_set_prototype_provider_template(
    v8::FunctionTemplate* fn_template,
    const v8::Local<v8::FunctionTemplate>* prototype_provider) {
  fn_template->SetPrototypeProviderTemplate(*prototype_provider);
}

void v8cxx__function_template_set_class_name(
    v8::FunctionTemplate* fn_template,
    const v8::Local<v8::String>* name) {
  fn_template->SetClassName(*name);
}

void v8cxx__function_template_set_interface_name(
    v8::FunctionTemplate* fn_template,
    const v8::Local<v8::String>* name) {
  fn_template->SetInterfaceName(*name);
}

void v8cxx__function_template_read_only_prototype(
    v8::FunctionTemplate* fn_template) {
  fn_template->ReadOnlyPrototype();
}

void v8cxx__function_template_remove_prototype(
    v8::FunctionTemplate* fn_template) {
  fn_template->RemovePrototype();
}

bool v8cxx__funtion_template_has_instance(v8::FunctionTemplate* fn_template,
                                          const v8::Local<v8::Value>* object) {
  return fn_template->HasInstance(*object);
}
}

// v8::Function
extern "C" {
void v8cxx__function_new(v8::MaybeLocal<v8::Function>* maybe_local_buf,
                         const v8::Local<v8::Context>* context,
                         v8::FunctionCallback callback,
                         const v8::Local<v8::Value>* data,
                         int length,
                         v8::ConstructorBehavior behavior,
                         v8::SideEffectType side_effect_type) {
  new (maybe_local_buf) v8::MaybeLocal<v8::Function>(v8::Function::New(
      *context, callback, *data, length, behavior, side_effect_type));
}

void v8cxx__function_new_instance(v8::MaybeLocal<v8::Object>* maybe_local_buf,
                                  const v8::Function* function,
                                  const v8::Local<v8::Context>* context,
                                  int argc,
                                  v8::Local<v8::Value>* argv) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Object>(function->NewInstance(*context, argc, argv));
}

void v8cxx__function_new_instance_with_side_effect_type(
    v8::MaybeLocal<v8::Object>* maybe_local_buf,
    const v8::Function* function,
    const v8::Local<v8::Context>* context,
    int argc,
    v8::Local<v8::Value>* argv,
    v8::SideEffectType side_effect_type) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::Object>(function->NewInstanceWithSideEffectType(
          *context, argc, argv, side_effect_type));
}

void v8cxx__function_set_name(v8::Function* function,
                              const v8::Local<v8::String>* name) {
  function->SetName(*name);
}

void v8cxx__function_get_name(v8::Local<v8::Value>* local_buf,
                              const v8::Function* function) {
  new (local_buf) v8::Local<v8::Value>(function->GetName());
}

void v8cxx__function_get_inferred_name(v8::Local<v8::Value>* local_buf,
                                       const v8::Function* function) {
  new (local_buf) v8::Local<v8::Value>(function->GetInferredName());
}

int v8cxx__function_get_script_line_number(const v8::Function* function) {
  return function->GetScriptLineNumber();
}

int v8cxx__function_get_script_column_number(const v8::Function* function) {
  return function->GetScriptColumnNumber();
}

int v8cxx__function_script_id(const v8::Function* function) {
  return function->ScriptId();
}

void v8cxx__function_get_bound_function(v8::Local<v8::Value>* local_buf,
                                        const v8::Function* function) {
  new (local_buf) v8::Local<v8::Value>(function->GetBoundFunction());
}

void v8cxx__function_function_proto_to_string(
    v8::MaybeLocal<v8::String>* maybe_local_buf,
    v8::Function* function,
    const v8::Local<v8::Context>* context) {
  new (maybe_local_buf)
      v8::MaybeLocal<v8::String>(function->FunctionProtoToString(*context));
}

void v8cxx__function_get_script_origin(v8::ScriptOrigin* buf,
                                       const v8::Function* function) {
  new (buf) v8::ScriptOrigin(function->GetScriptOrigin());
}
}

// v8::FunctionCallbackInfo<T>
extern "C" {
int v8cxx__function_callback_info_length(
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  return fn_callback_info->Length();
}

void v8cxx__function_callback_info_at(
    v8::Local<v8::Value>* local_buf,
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info,
    int index) {
  new (local_buf) v8::Local<v8::Value>((*fn_callback_info)[index]);
}

void v8cxx__function_callback_info_this(
    v8::Local<v8::Object>* local_buf,
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  new (local_buf) v8::Local<v8::Object>(fn_callback_info->This());
}

void v8cxx__function_callback_info_new_target(
    v8::Local<v8::Value>* local_buf,
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  new (local_buf) v8::Local<v8::Value>(fn_callback_info->NewTarget());
}

bool v8cxx__function_callback_info_is_construct_call(
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  return fn_callback_info->IsConstructCall();
}

void v8cxx__function_callback_info_data(
    v8::Local<v8::Value>* local_buf,
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  new (local_buf) v8::Local<v8::Value>(fn_callback_info->Data());
}

v8::Isolate* v8cxx__function_callback_info_get_isolate(
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  return fn_callback_info->GetIsolate();
}

void v8cxx__function_callback_info_get_return_value(
    v8::ReturnValue<v8::Value>* buf,
    const v8::FunctionCallbackInfo<v8::Value>* fn_callback_info) {
  new (buf) v8::ReturnValue<v8::Value>(fn_callback_info->GetReturnValue());
}
}

// v8::ReturnValue<T>
extern "C" {
void v8cxx__return_value_set(v8::ReturnValue<v8::Value>* return_value,
                             const v8::Local<v8::Value>* value) {
  return_value->Set(*value);
}

void v8cxx__return_value_set_bool(v8::ReturnValue<v8::Value>* return_value,
                                  bool value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_f64(v8::ReturnValue<v8::Value>* return_value,
                                 double value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i16(v8::ReturnValue<v8::Value>* return_value,
                                 int16_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i32(v8::ReturnValue<v8::Value>* return_value,
                                 int32_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i64(v8::ReturnValue<v8::Value>* return_value,
                                 int64_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u16(v8::ReturnValue<v8::Value>* return_value,
                                 uint16_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u32(v8::ReturnValue<v8::Value>* return_value,
                                 uint32_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u64(v8::ReturnValue<v8::Value>* return_value,
                                 uint64_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_null(v8::ReturnValue<v8::Value>* return_value) {
  return_value->SetNull();
}

void v8cxx__return_value_set_undefined(
    v8::ReturnValue<v8::Value>* return_value) {
  return_value->SetUndefined();
}

void v8cxx__return_value_set_false(v8::ReturnValue<v8::Value>* return_value) {
  return_value->SetFalse();
}

void v8cxx__return_value_set_empty_string(
    v8::ReturnValue<v8::Value>* return_value) {
  return_value->SetEmptyString();
}

v8::Isolate* v8cxx__return_value_get_isolate(
    const v8::ReturnValue<v8::Value>* return_value) {
  return return_value->GetIsolate();
}

void v8cxx__return_value_get(v8::Local<v8::Value>* local_buf,
                             const v8::ReturnValue<v8::Value>* return_value) {
  new (local_buf) v8::Local<v8::Value>(return_value->Get());
}
}

// v8::ScriptOrigin
extern "C" {
void v8cxx__script_origin_new(v8::ScriptOrigin* buf,
                              const v8::Local<v8::Value>* resource_name,
                              int resource_line_offset,
                              int resource_column_offset,
                              bool resource_is_shared_cross_origin,
                              int script_id,
                              const v8::Local<v8::Value>* source_map_url,
                              bool resource_is_opaque,
                              bool is_wasm,
                              bool is_module,
                              const v8::Local<v8::Data>* host_defined_options) {
  new (buf) v8::ScriptOrigin(v8::ScriptOrigin(
      *resource_name, resource_line_offset, resource_column_offset,
      resource_is_shared_cross_origin, script_id, *source_map_url,
      resource_is_opaque, is_wasm, is_module, *host_defined_options));
}

void v8cxx__script_origin_resource_name(v8::Local<v8::Value>* local_buf,
                                        const v8::ScriptOrigin* script_origin) {
  new (local_buf) v8::Local<v8::Value>(script_origin->ResourceName());
}

int v8cxx__script_origin_line_offset(const v8::ScriptOrigin* script_origin) {
  return script_origin->LineOffset();
}

int v8cxx__script_origin_column_offset(const v8::ScriptOrigin* script_origin) {
  return script_origin->ColumnOffset();
}

int v8cxx__script_origin_script_id(const v8::ScriptOrigin* script_origin) {
  return script_origin->ScriptId();
}

void v8cxx__script_origin_source_map_url(
    v8::Local<v8::Value>* local_buf,
    const v8::ScriptOrigin* script_origin) {
  new (local_buf) v8::Local<v8::Value>(script_origin->SourceMapUrl());
}

void v8cxx__script_origin_get_host_defined_options(
    v8::Local<v8::Data>* local_buf,
    const v8::ScriptOrigin* script_origin) {
  new (local_buf) v8::Local<v8::Data>(script_origin->GetHostDefinedOptions());
}

bool v8cxx__script_origin_is_module(const v8::ScriptOrigin* script_origin) {
  return script_origin->Options().IsModule();
}

bool v8cxx__script_origin_is_opaque(const v8::ScriptOrigin* script_origin) {
  return script_origin->Options().IsOpaque();
}

bool v8cxx__script_origin_is_shared_cross_origin(
    const v8::ScriptOrigin* script_origin) {
  return script_origin->Options().IsSharedCrossOrigin();
}

bool v8cxx__script_origin_is_wasm(const v8::ScriptOrigin* script_origin) {
  return script_origin->Options().IsWasm();
}
}

// v8::ObjectTemplate
extern "C" {
void v8cxx__object_template_new(
    v8::Local<v8::ObjectTemplate>* local_buf,
    v8::Isolate* isolate,
    const v8::Local<v8::FunctionTemplate>* constructor) {
  new (local_buf) v8::Local<v8::ObjectTemplate>(
      v8::ObjectTemplate::New(isolate, *constructor));
}

void v8cxx__object_template_new_instance(
    v8::MaybeLocal<v8::Object>* local_buf,
    v8::ObjectTemplate* object_template,
    const v8::Local<v8::Context>* context) {
  new (local_buf)
      v8::MaybeLocal<v8::Object>(object_template->NewInstance(*context));
}

void v8cxx__object_template_set_named_property_handler(
    v8::ObjectTemplate* object_template,
    v8::NamedPropertyGetterCallback getter,
    v8::NamedPropertySetterCallback setter,
    v8::NamedPropertyQueryCallback query,
    v8::NamedPropertyDeleterCallback deleter,
    v8::NamedPropertyEnumeratorCallback enumerator,
    v8::NamedPropertyDefinerCallback definer,
    v8::NamedPropertyDescriptorCallback descriptor,
    const v8::Local<v8::Value>* data,
    v8::PropertyHandlerFlags flags) {
  object_template->SetHandler(v8::NamedPropertyHandlerConfiguration(
      getter, setter, query, deleter, enumerator, definer, descriptor, *data,
      flags));
}

void v8cxx__object_template_set_indexed_property_handler(
    v8::ObjectTemplate* object_template,
    v8::IndexedPropertyGetterCallbackV2 getter,
    v8::IndexedPropertySetterCallbackV2 setter,
    v8::IndexedPropertyQueryCallbackV2 query,
    v8::IndexedPropertyDeleterCallbackV2 deleter,
    v8::IndexedPropertyEnumeratorCallback enumerator,
    v8::IndexedPropertyDefinerCallbackV2 definer,
    v8::IndexedPropertyDescriptorCallbackV2 descriptor,
    const v8::Local<v8::Value>* data,
    v8::PropertyHandlerFlags flags) {
  object_template->SetHandler(v8::IndexedPropertyHandlerConfiguration(
      getter, setter, query, deleter, enumerator, definer, descriptor, *data,
      flags));
}

void v8cxx__object_template_set_call_as_function_handler(
    v8::ObjectTemplate* object_template,
    v8::FunctionCallback callback,
    const v8::Local<v8::Value>* data) {
  object_template->SetCallAsFunctionHandler(callback, *data);
}

void v8cxx__object_template_mark_as_undetectable(
    v8::ObjectTemplate* object_template) {
  object_template->MarkAsUndetectable();
}

// TODO: v8::ObjectTemplate::{
//   SetAccessCheckCallback,
//   SetAccessCheckCallbackAndHandler
// }

int v8cxx__object_template_internal_field_count(
    const v8::ObjectTemplate* object_template) {
  return object_template->InternalFieldCount();
}

void v8cxx__object_template_set_internal_field_count(
    v8::ObjectTemplate* object_template,
    int count) {
  object_template->SetInternalFieldCount(count);
}

bool v8cxx__object_template_is_immutable_proto(
    const v8::ObjectTemplate* object_template) {
  return object_template->IsImmutableProto();
}

// TODO: v8::ObjectTemplate::{SetCodeLike, IsCodeLike}
}

// v8::PropertyCallbackInfo<T>
extern "C" {
v8::Isolate* v8cxx__property_callback_info_get_isolate(
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  return pci->GetIsolate();
}

void v8cxx__property_callback_info_data(
    v8::Local<v8::Value>* local_buf,
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  new (local_buf) v8::Local<v8::Value>(pci->Data());
}

void v8cxx__property_callback_info_this(
    v8::Local<v8::Object>* local_buf,
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  new (local_buf) v8::Local<v8::Object>(pci->This());
}

void v8cxx__property_callback_info_holder(
    v8::Local<v8::Object>* local_buf,
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  new (local_buf) v8::Local<v8::Object>(pci->HolderV2());
}

void v8cxx__property_callback_info_get_return_value(
    v8::ReturnValue<v8::Value>* buf,
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  new (buf) v8::ReturnValue<v8::Value>(pci->GetReturnValue());
}

bool v8cxx__property_callback_info_should_throw_on_error(
    const v8::PropertyCallbackInfo<v8::Value>* pci) {
  return pci->ShouldThrowOnError();
}
}

// v8::Number
extern "C" {
void v8cxx__number_new(v8::Local<v8::Number>* local_buf,
                       v8::Isolate* isolate,
                       double value) {
  new (local_buf) v8::Local<v8::Number>(v8::Number::New(isolate, value));
}

double v8cxx__number_value(const v8::Number* number) {
  return number->Value();
}
}

// v8::Integer
extern "C" {
void v8cxx__integer_new(v8::Local<v8::Integer>* local_buf,
                        v8::Isolate* isolate,
                        int32_t value) {
  new (local_buf) v8::Local<v8::Integer>(v8::Integer::New(isolate, value));
}

void v8cxx__integer_new_from_unsigned(v8::Local<v8::Integer>* local_buf,
                                      v8::Isolate* isolate,
                                      uint32_t value) {
  new (local_buf)
      v8::Local<v8::Integer>(v8::Integer::NewFromUnsigned(isolate, value));
}

int64_t v8cxx__integer_value(const v8::Integer* integer) {
  return integer->Value();
}
}

// v8::Array
extern "C" {
uint32_t v8cxx__array_length(const v8::Array* array) {
  return array->Length();
}

void v8cxx__array_new(v8::Local<v8::Array>* local_buf,
                      v8::Isolate* isolate,
                      int length) {
  new (local_buf) v8::Local<v8::Array>(v8::Array::New(isolate, length));
}

void v8cxx__array_new_with_elements(v8::Local<v8::Array>* local_buf,
                                    v8::Isolate* isolate,
                                    const v8::Local<v8::Value>* elements,
                                    size_t length) {
  new (local_buf) v8::Local<v8::Array>(v8::Array::New(
      isolate, const_cast<v8::Local<v8::Value>*>(elements), length));
}
}

// v8::UnboundModuleScript
extern "C" {
void v8cxx__unbound_module_script_get_source_url(
    v8::Local<v8::Value>* local_buf,
    v8::UnboundModuleScript* ums) {
  new (local_buf) v8::Local<v8::Value>(ums->GetSourceURL());
}

void v8cxx__unbound_module_script_get_source_mapping_url(
    v8::Local<v8::Value>* local_buf,
    v8::UnboundModuleScript* ums) {
  new (local_buf) v8::Local<v8::Value>(ums->GetSourceMappingURL());
}
}

// v8::UnboundScript
extern "C" {
void v8cxx__unbound_script_bind_to_current_context(
    v8::Local<v8::Script>* local_buf,
    v8::UnboundScript* us) {
  new (local_buf) v8::Local<v8::Script>(us->BindToCurrentContext());
}

int v8cxx__unbound_script_get_id(const v8::UnboundScript* us) {
  return us->GetId();
}

void v8cxx__unbound_script_get_script_name(v8::Local<v8::Value>* local_buf,
                                           v8::UnboundScript* us) {
  new (local_buf) v8::Local<v8::Value>(us->GetScriptName());
}

void v8cxx__unbound_script_get_source_url(v8::Local<v8::Value>* local_buf,
                                          v8::UnboundScript* us) {
  new (local_buf) v8::Local<v8::Value>(us->GetSourceURL());
}

void v8cxx__unbound_script_get_source_mapping_url(
    v8::Local<v8::Value>* local_buf,
    v8::UnboundScript* us) {
  new (local_buf) v8::Local<v8::Value>(us->GetSourceMappingURL());
}

int v8cxx__unbound_script_get_line_number(v8::UnboundScript* us, int code_pos) {
  return us->GetLineNumber(code_pos);
}

int v8cxx__unbound_script_get_column_number(v8::UnboundScript* us,
                                            int code_pos) {
  return us->GetColumnNumber(code_pos);
}
}