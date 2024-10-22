#include <iostream>
#include <memory>

#include "libplatform/libplatform.h"
#include "v8.h"

using namespace v8;

// v8::platform
extern "C" {
Platform* v8cxx__platform__new_default_platform(
    int thread_pool_size,
    platform::IdleTaskSupport idle_task_support,
    platform::InProcessStackDumping in_process_stack_dumping,
    platform::PriorityMode priority_mode) {
  return platform::NewDefaultPlatform(thread_pool_size, idle_task_support,
                                      in_process_stack_dumping, nullptr,
                                      priority_mode)
      .release();
}
}

// v8::Platform
extern "C" {
void v8cxx__platform_drop(Platform* self) {
  delete self;
}
}

// v8::V8
extern "C" {
void v8cxx__v8__initialize_platform(Platform* platform) {
  V8::InitializePlatform(platform);
}

bool v8cxx__v8__initialize() {
  return V8::Initialize();
}

void v8cxx__v8__dispose_platform() {
  V8::DisposePlatform();
}

bool v8cxx__v8__dispose() {
  return V8::Dispose();
}
}

// v8::ArrayBuffer
extern "C" {
size_t v8cxx__array_buffer_byte_length(const ArrayBuffer* ab) {
  return ab->ByteLength();
}

size_t v8cxx__array_buffer_max_byte_length(const ArrayBuffer* ab) {
  return ab->MaxByteLength();
}

void v8cxx__array_buffer_new(Local<ArrayBuffer>* local_buf,
                             Isolate* isolate,
                             size_t byte_length,
                             BackingStoreInitializationMode init_mode) {
  new (local_buf)
      Local<ArrayBuffer>(ArrayBuffer::New(isolate, byte_length, init_mode));
}

bool v8cxx__array_buffer_is_detachable(const ArrayBuffer* ab) {
  return ab->IsDetachable();
}

bool v8cxx__array_buffer_was_detached(const ArrayBuffer* ab) {
  return ab->WasDetached();
}

bool v8cxx__array_buffer_detach(ArrayBuffer* ab, const Local<Value>* key) {
  auto result = false;

  return ab->Detach(*key).FromMaybe(&result);
}

void v8cxx__array_buffer_set_detach_key(ArrayBuffer* ab,
                                        const Local<Value>* key) {
  ab->SetDetachKey(*key);
}

bool v8cxx__array_buffer_is_resizable_by_user_javascript(
    const ArrayBuffer* ab) {
  return ab->IsResizableByUserJavaScript();
}

void* v8cxx__array_buffer_data(const ArrayBuffer* ab) {
  return ab->Data();
}
}

// v8::ArrayBuffer::Allocator
extern "C" {
ArrayBuffer::Allocator* v8cxx__array_buffer__allocator_new_default_allocator() {
  return ArrayBuffer::Allocator::NewDefaultAllocator();
}
}

// v8::Isolate
extern "C" {
Isolate::CreateParams v8cxx__isolate__createparams(
    ArrayBuffer::Allocator* array_buffer_allocator) {
  Isolate::CreateParams create_params;
  create_params.array_buffer_allocator = array_buffer_allocator;

  return create_params;
}

Isolate* v8cxx__isolate_new(const Isolate::CreateParams& create_params) {
  return Isolate::New(create_params);
}

Isolate* v8cxx__isolate_get_current() {
  return Isolate::GetCurrent();
}

void v8cxx__isolate_get_current_context(Local<Context>* local_buf,
                                        Isolate* isolate) {
  new (local_buf) Local<Context>(isolate->GetCurrentContext());
}

void v8cxx__isolate_enter(Isolate* isolate) {
  isolate->Enter();
}

void v8cxx__isolate_exit(Isolate* isolate) {
  isolate->Exit();
}
}

// v8::HandleScope
extern "C" {
class HandleScope {
 public:
  HandleScope(Isolate* isolate) : handle_scope(isolate) {}
  ~HandleScope() {}

  const v8::HandleScope& get() const { return this->handle_scope; }

 private:
  v8::HandleScope handle_scope;
};

void v8cxx__handlescope_new(::HandleScope* buf, Isolate* isolate) {
  new (buf)::HandleScope(isolate);
}

void v8cxx__handlescope_drop(::HandleScope* handle_scope) {
  handle_scope->~HandleScope();
}

Isolate* v8cxx__handlescope_get_isolate(const ::HandleScope* handle_scope) {
  return handle_scope->get().GetIsolate();
}
}

// v8::Context
extern "C" {
// TODO: extend functionality of Context::new
void v8cxx__context_new(Local<Context>* local_buf,
                        Isolate* isolate,
                        const Local<ObjectTemplate>* global_template,
                        const Local<Value>* global_object,
                        MicrotaskQueue* microtask_queue) {
  new (local_buf) Local<Context>(v8::Context::New(
      isolate, nullptr,
      global_template == nullptr ? MaybeLocal<ObjectTemplate>()
                                 : *global_template,
      global_object == nullptr ? MaybeLocal<Value>() : *global_object,
      DeserializeInternalFieldsCallback(), microtask_queue));
}

void v8cxx__context_enter(Context* context) {
  context->Enter();
}

void v8cxx__context_exit(Context* context) {
  context->Exit();
}

void v8cxx__context_global(Local<Object>* local_buf, Context* context) {
  new (local_buf) Local<Object>(context->Global());
}

void v8cxx__context_detach_global(Context* context) {
  context->DetachGlobal();
}

Isolate* v8cxx__context_get_isolate(Context* context) {
  return context->GetIsolate();
}

MicrotaskQueue* v8cxx__context_get_microtask_queue(Context* context) {
  return context->GetMicrotaskQueue();
}

void v8cxx__context_set_microtask_queue(Context* context,
                                        MicrotaskQueue* microtask_queue) {
  context->SetMicrotaskQueue(microtask_queue);
}

void v8cxx__context_set_security_token(Context* context,
                                       const Local<Value>* token) {
  context->SetSecurityToken(*token);
}

void v8cxx__context_use_default_security_token(Context* context) {
  context->UseDefaultSecurityToken();
}

uint32_t v8cxx__context_get_number_of_embedder_data_fields(Context* context) {
  return context->GetNumberOfEmbedderDataFields();
}

void v8cxx__context_get_embedder_data(Local<Value>* local_buf,
                                      Context* context,
                                      int index) {
  new (local_buf) Local<Value>(context->GetEmbedderData(index));
}

void v8cxx__context_get_extras_binding_object(Local<Object>* local_buf,
                                              Context* context) {
  new (local_buf) Local<Object>(context->GetExtrasBindingObject());
}

void v8cxx__context_set_embedder_data(Context* context,
                                      int index,
                                      const Local<Value>* value) {
  context->SetEmbedderData(index, *value);
}

void* v8cxx__context_get_aligned_pointer_from_embedder_data(Context* context,
                                                            Isolate* isolate,
                                                            int index) {
  return context->GetAlignedPointerFromEmbedderData(isolate, index);
}

void v8cxx__context_set_aligned_pointer_in_embedder_data(Context* context,
                                                         int index,
                                                         void* value) {
  context->SetAlignedPointerInEmbedderData(index, value);
}

void v8cxx__context_allow_code_generation_from_strings(Context* context,
                                                       bool allow) {
  context->AllowCodeGenerationFromStrings(allow);
}

bool v8cxx__context_is_code_generation_from_string_allowed(
    const Context* context) {
  return context->IsCodeGenerationFromStringsAllowed();
}

void v8cxx__context_set_error_message_for_code_generation_from_strings(
    Context* context,
    const Local<String>* message) {
  context->SetErrorMessageForCodeGenerationFromStrings(*message);
}

void v8cxx__context_set_error_message_for_wasm_code_generation(
    Context* context,
    const Local<String>* message) {
  context->SetErrorMessageForWasmCodeGeneration(*message);
}

void v8cxx__context_set_abort_script_execution(
    Context* context,
    Context::AbortScriptExecutionCallback callback) {
  context->SetAbortScriptExecution(callback);
}

void v8cxx__context_set_promise_hooks(Context* context,
                                      const Local<Function>* init_hook,
                                      const Local<Function>* before_hook,
                                      const Local<Function>* after_hook,
                                      const Local<Function>* resolve_hook) {
  context->SetPromiseHooks(*init_hook, *before_hook, *after_hook,
                           *resolve_hook);
}

bool v8cxx__context_has_template_literal_object(Context* context,
                                                const Local<Value>* object) {
  return context->HasTemplateLiteralObject(*object);
}
}

// v8::Data
extern "C" {
bool v8cxx__data_is_context(const Data& data) {
  return data.IsContext();
}

bool v8cxx__data_is_fixed_array(const Data& data) {
  return data.IsFixedArray();
}

bool v8cxx__data_is_function_template(const Data& data) {
  return data.IsFunctionTemplate();
}

bool v8cxx__data_is_module(const Data& data) {
  return data.IsModule();
}

bool v8cxx__data_is_object_template(const Data& data) {
  return data.IsObjectTemplate();
}

bool v8cxx__data_is_private(const Data& data) {
  return data.IsPrivate();
}

bool v8cxx__data_is_value(const Data& data) {
  return data.IsValue();
}
}

// v8::Value
extern "C" {
bool v8cxx__value_is_undefined(const Value& value) {
  return value.IsUndefined();
}

bool v8cxx__value_is_null(const Value& value) {
  return value.IsNull();
}

bool v8cxx__value_is_null_or_undefined(const Value& value) {
  return value.IsNullOrUndefined();
}

bool v8cxx__value_is_true(const Value& value) {
  return value.IsTrue();
}

bool v8cxx__value_is_false(const Value& value) {
  return value.IsFalse();
}

bool v8cxx__value_is_name(const Value& value) {
  return value.IsName();
}

bool v8cxx__value_is_string(const Value& value) {
  return value.IsString();
}

bool v8cxx__value_is_symbol(const Value& value) {
  return value.IsSymbol();
}

bool v8cxx__value_is_function(const Value& value) {
  return value.IsFunction();
}

bool v8cxx__value_is_array(const Value& value) {
  return value.IsArray();
}

bool v8cxx__value_is_object(const Value& value) {
  return value.IsObject();
}

bool v8cxx__value_is_bigint(const Value& value) {
  return value.IsBigInt();
}

bool v8cxx__value_is_boolean(const Value& value) {
  return value.IsBoolean();
}

bool v8cxx__value_is_number(const Value& value) {
  return value.IsNumber();
}

bool v8cxx__value_is_external(const Value& value) {
  return value.IsExternal();
}

bool v8cxx__value_is_int32(const Value& value) {
  return value.IsInt32();
}

bool v8cxx__value_is_uint32(const Value& value) {
  return value.IsUint32();
}

bool v8cxx__value_is_date(const Value& value) {
  return value.IsDate();
}

bool v8cxx__value_is_arguments_object(const Value& value) {
  return value.IsArgumentsObject();
}

bool v8cxx__value_is_bigint_object(const Value& value) {
  return value.IsBigIntObject();
}

bool v8cxx__value_is_boolean_object(const Value& value) {
  return value.IsBooleanObject();
}

bool v8cxx__value_is_number_object(const Value& value) {
  return value.IsNumberObject();
}

bool v8cxx__value_is_string_object(const Value& value) {
  return value.IsStringObject();
}

bool v8cxx__value_is_symbol_object(const Value& value) {
  return value.IsSymbolObject();
}

bool v8cxx__value_is_native_error(const Value& value) {
  return value.IsNativeError();
}

bool v8cxx__value_is_regexp(const Value& value) {
  return value.IsRegExp();
}

bool v8cxx__value_is_async_function(const Value& value) {
  return value.IsAsyncFunction();
}

bool v8cxx__value_is_generator_function(const Value& value) {
  return value.IsGeneratorFunction();
}

bool v8cxx__value_is_generator_object(const Value& value) {
  return value.IsGeneratorObject();
}

bool v8cxx__value_is_promise(const Value& value) {
  return value.IsPromise();
}

bool v8cxx__value_is_map(const Value& value) {
  return value.IsMap();
}

bool v8cxx__value_is_set(const Value& value) {
  return value.IsSet();
}

bool v8cxx__value_is_map_iterator(const Value& value) {
  return value.IsMapIterator();
}

bool v8cxx__value_is_set_iterator(const Value& value) {
  return value.IsSetIterator();
}

bool v8cxx__value_is_weak_map(const Value& value) {
  return value.IsWeakMap();
}

bool v8cxx__value_is_weak_set(const Value& value) {
  return value.IsWeakSet();
}

bool v8cxx__value_is_weak_ref(const Value& value) {
  return value.IsWeakRef();
}

bool v8cxx__value_is_array_buffer(const Value& value) {
  return value.IsArrayBuffer();
}

bool v8cxx__value_is_array_buffer_view(const Value& value) {
  return value.IsArrayBufferView();
}

bool v8cxx__value_is_typed_array(const Value& value) {
  return value.IsTypedArray();
}

bool v8cxx__value_is_uint8_array(const Value& value) {
  return value.IsUint8Array();
}

bool v8cxx__value_is_uint8_clamped_array(const Value& value) {
  return value.IsUint8ClampedArray();
}

bool v8cxx__value_is_int8_array(const Value& value) {
  return value.IsInt8Array();
}

bool v8cxx__value_is_uint16_array(const Value& value) {
  return value.IsUint16Array();
}

bool v8cxx__value_is_int16_array(const Value& value) {
  return value.IsInt16Array();
}

bool v8cxx__value_is_uint32_array(const Value& value) {
  return value.IsUint32Array();
}

bool v8cxx__value_is_int32_array(const Value& value) {
  return value.IsInt32Array();
}

bool v8cxx__value_is_float16_array(const Value& value) {
  return value.IsFloat16Array();
}

bool v8cxx__value_is_float32_array(const Value& value) {
  return value.IsFloat32Array();
}

bool v8cxx__value_is_float64_array(const Value& value) {
  return value.IsFloat64Array();
}

bool v8cxx__value_is_bigint64_array(const Value& value) {
  return value.IsBigInt64Array();
}

bool v8cxx__value_is_biguint64_array(const Value& value) {
  return value.IsBigUint64Array();
}

bool v8cxx__value_is_data_view(const Value& value) {
  return value.IsDataView();
}

bool v8cxx__value_is_shared_array_buffer(const Value& value) {
  return value.IsSharedArrayBuffer();
}

bool v8cxx__value_is_proxy(const Value& value) {
  return value.IsProxy();
}

bool v8cxx__value_is_wasm_memory_object(const Value& value) {
  return value.IsWasmMemoryObject();
}

bool v8cxx__value_is_wasm_module_object(const Value& value) {
  return value.IsWasmModuleObject();
}

bool v8cxx__value_is_wasm_null(const Value& value) {
  return value.IsWasmNull();
}

bool v8cxx__value_is_module_namespace_object(const Value& value) {
  return value.IsModuleNamespaceObject();
}

void v8cxx__value_to_primitive(MaybeLocal<Primitive>* maybe_local_buf,
                               const Value& value,
                               const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<Primitive>(value.ToPrimitive(*context));
}

void v8cxx__value_to_bigint(MaybeLocal<BigInt>* maybe_local_buf,
                            const Value& value,
                            const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<BigInt>(value.ToBigInt(*context));
}

void v8cxx__value_to_number(MaybeLocal<Number>* maybe_local_buf,
                            const Value& value,
                            const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<Number>(value.ToNumber(*context));
}

void v8cxx__value_to_string(MaybeLocal<String>* maybe_local_buf,
                            const Value& value,
                            const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<String>(value.ToString(*context));
}

void v8cxx__value_to_object(MaybeLocal<Object>* maybe_local_buf,
                            const Value& value,
                            const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<Object>(value.ToObject(*context));
}

void v8cxx__value_to_boolean(Local<Boolean>* local_buf,
                             const Value& value,
                             Isolate* isolate) {
  new (local_buf) Local<Boolean>(value.ToBoolean(isolate));
}

void v8cxx__value_typeof(Local<String>* local_buf,
                         Value& value,
                         Isolate* isolate) {
  new (local_buf) Local<String>(value.TypeOf(isolate));
}

bool v8cxx__value_instanceof(Value& value,
                             const Local<Context>* context,
                             const Local<Object>* object) {
  return value.InstanceOf(*context, *object).ToChecked();
}
}

// v8::Primitive
extern "C" {
void v8cxx__primitive_undefined(Local<Primitive>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Primitive>(v8::Undefined(isolate));
}

void v8cxx__primitive_null(Local<Primitive>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Primitive>(v8::Null(isolate));
}
}

// v8::Boolean
extern "C" {
void v8cxx__boolean_new(Local<Boolean>* local_buf,
                        Isolate* isolate,
                        bool value) {
  new (local_buf)
      Local<Boolean>(value ? v8::True(isolate) : v8::False(isolate));
}

bool v8cxx__boolean_value(const Boolean* boolean) {
  return boolean->Value();
}
}

// v8::String
extern "C" {
void v8cxx__string_new_from_utf8(MaybeLocal<String>* maybe_local_buf,
                                 Isolate* isolate,
                                 const char* value,
                                 NewStringType type,
                                 int length) {
  new (maybe_local_buf)
      MaybeLocal<String>(String::NewFromUtf8(isolate, value, type, length));
}

void v8cxx__string_new_from_onebyte(MaybeLocal<String>* maybe_local_buf,
                                    Isolate* isolate,
                                    const uint8_t* value,
                                    NewStringType type,
                                    int length) {
  new (maybe_local_buf)
      MaybeLocal<String>(String::NewFromOneByte(isolate, value, type, length));
}

void v8cxx__string_new_from_twobyte(MaybeLocal<String>* maybe_local_buf,
                                    Isolate* isolate,
                                    const uint16_t* value,
                                    NewStringType type,
                                    int length) {
  new (maybe_local_buf)
      MaybeLocal<String>(String::NewFromTwoByte(isolate, value, type, length));
}

int v8cxx__string_length(const String& string) {
  return string.Length();
}

int v8cxx__string_utf8length(const String& string, Isolate* isolate) {
  return string.Utf8Length(isolate);
}

bool v8cxx__string_is_onebyte(const String& string) {
  return string.IsOneByte();
}

bool v8cxx__string_contains_only_onebyte(const String& string) {
  return string.ContainsOnlyOneByte();
}

bool v8cxx__string_is_external(const String& string) {
  return string.IsExternal();
}

bool v8cxx__string_is_external_twobyte(const String& string) {
  return string.IsExternalTwoByte();
}

bool v8cxx__string_is_external_onebyte(const String& string) {
  return string.IsExternalOneByte();
}

void v8cxx__string_internalize_string(Local<String>* local_buf,
                                      String& string,
                                      Isolate* isolate) {
  new (local_buf) Local<String>(string.InternalizeString(isolate));
}

const char* v8cxx__string_view(const String* string, Isolate* isolate) {
  return *String::Utf8Value(
      isolate, string->ToString(isolate->GetCurrentContext()).ToLocalChecked());
}
}

// v8::Symbol
extern "C" {
void v8cxx__symbol_new(Local<Symbol>* local_buf,
                       Isolate* isolate,
                       const Local<String>* name) {
  new (local_buf) Local<Symbol>(v8::Symbol::New(isolate, *name));
}

void v8cxx__symbol_for(Local<Symbol>* local_buf,
                       Isolate* isolate,
                       const Local<String>* description) {
  new (local_buf) Local<Symbol>(v8::Symbol::For(isolate, *description));
}

void v8cxx__symbol_for_api(Local<Symbol>* local_buf,
                           Isolate* isolate,
                           const Local<String>* description) {
  new (local_buf) Local<Symbol>(v8::Symbol::For(isolate, *description));
}

void v8cxx__symbol_get_async_iterator(Local<Symbol>* local_buf,
                                      Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetAsyncIterator(isolate));
}

void v8cxx__symbol_get_has_instance(Local<Symbol>* local_buf,
                                    Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetHasInstance(isolate));
}

void v8cxx__symbol_get_is_concat_spreadable(Local<Symbol>* local_buf,
                                            Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetIsConcatSpreadable(isolate));
}

void v8cxx__symbol_get_iterator(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetIterator(isolate));
}

void v8cxx__symbol_get_match(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetMatch(isolate));
}

void v8cxx__symbol_get_replace(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetReplace(isolate));
}

void v8cxx__symbol_get_search(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetSearch(isolate));
}

void v8cxx__symbol_get_split(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetSplit(isolate));
}

void v8cxx__symbol_get_to_primitive(Local<Symbol>* local_buf,
                                    Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetToPrimitive(isolate));
}

void v8cxx__symbol_get_to_string_tag(Local<Symbol>* local_buf,
                                     Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetToStringTag(isolate));
}

void v8cxx__symbol_get_unscopables(Local<Symbol>* local_buf, Isolate* isolate) {
  new (local_buf) Local<Symbol>(v8::Symbol::GetUnscopables(isolate));
}
}

// v8::BigInt
extern "C" {
void v8cxx__bigint_new(Local<BigInt>* local_buf,
                       Isolate* isolate,
                       int64_t value) {
  new (local_buf) Local<BigInt>(v8::BigInt::New(isolate, value));
}

void v8cxx__bigint_new_from_unsigned(Local<BigInt>* local_buf,
                                     Isolate* isolate,
                                     uint64_t value) {
  new (local_buf) Local<BigInt>(v8::BigInt::NewFromUnsigned(isolate, value));
}

uint64_t v8cxx__bigint_uint64_value(const BigInt* big_int, bool* lossless) {
  return big_int->Uint64Value(lossless);
}

int64_t v8cxx__bigint_int64_value(const BigInt* big_int, bool* lossless) {
  return big_int->Int64Value(lossless);
}
}

// v8::Script
extern "C" {
void v8cxx__script_compile(MaybeLocal<Script>* maybe_local_buf,
                           const Local<Context>* context,
                           const Local<String>* source) {
  new (maybe_local_buf)
      MaybeLocal<Script>(v8::Script::Compile(*context, *source));
}

void v8cxx__script_run(MaybeLocal<Value>* maybe_local_buf,
                       Script* script,
                       const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<Value>(script->Run(*context));
}

void v8cxx__script_get_unbound_script(Local<UnboundScript>* local_buf,
                                      Script* script) {
  new (local_buf) Local<UnboundScript>(script->GetUnboundScript());
}

void v8cxx__script_get_resource_name(Local<Value>* local_buf, Script* script) {
  new (local_buf) Local<Value>(script->GetResourceName());
}

// TODO: add this to script::Script (Rust)
void v8cxx__script_get_compile_hints_collector(
    Local<CompileHintsCollector>* local_buf,
    const Script* script) {
  new (local_buf)
      Local<CompileHintsCollector>(script->GetCompileHintsCollector());
}
}

// v8::Name
extern "C" {
int v8cxx__name_get_identity_hash(Name* name) {
  return name->GetIdentityHash();
}
}

// v8::Object
extern "C" {
void v8cxx__object_new(Local<Object>* local_buf, v8::Isolate* isolate) {
  new (local_buf) Local<Object>(v8::Object::New(isolate));
}

bool v8cxx__object_set(Object* object,
                       const Local<Context>* context,
                       const Local<Value>* key,
                       const Local<Value>* value,
                       const MaybeLocal<Object>* receiver) {
  auto result = false;

  return object->Set(*context, *key, *value, *receiver).FromMaybe(&result);
}

bool v8cxx__object_set_indexed(Object* object,
                               const Local<Context>* context,
                               uint32_t index,
                               const Local<Value>* value) {
  auto result = false;

  return object->Set(*context, index, *value).FromMaybe(&result);
}

bool v8cxx__object_create_data_property(Object* object,
                                        const Local<Context>* context,
                                        const Local<Name>* key,
                                        const Local<Value>* value) {
  auto result = false;

  return object->CreateDataProperty(*context, *key, *value).FromMaybe(&result);
}

bool v8cxx__object_create_data_property_indexed(Object* object,
                                                const Local<Context>* context,
                                                uint32_t index,
                                                const Local<Value>* value) {
  auto result = false;

  return object->CreateDataProperty(*context, index, *value).FromMaybe(&result);
}

bool v8cxx__object_define_own_property(Object* object,
                                       const Local<Context>* context,
                                       const Local<Name>* key,
                                       const Local<Value>* value,
                                       PropertyAttribute attributes) {
  auto result = false;

  return object->DefineOwnProperty(*context, *key, *value, attributes)
      .FromMaybe(&result);
}

// TODO: v8cxx__object_define_property

void v8cxx__object_get(MaybeLocal<Value>* maybe_local_buf,
                       Object* object,
                       const Local<Context>* context,
                       const Local<Value>* key,
                       const MaybeLocal<Object>* receiver) {
  new (maybe_local_buf)
      MaybeLocal<Value>(object->Get(*context, *key, *receiver));
}

void v8cxx__object_get_indexed(MaybeLocal<Value>* maybe_local_buf,
                               Object* object,
                               const Local<Context>* context,
                               uint32_t index) {
  new (maybe_local_buf) MaybeLocal<Value>(object->Get(*context, index));
}
}

// v8::FixedArray
extern "C" {
int v8cxx__fixed_array_length(const FixedArray* fixed_array) {
  return fixed_array->Length();
}

void v8cxx__fixed_array_get(Local<Data>* local_buf,
                            const FixedArray* fixed_array,
                            const Local<Context>* context,
                            int i) {
  new (local_buf) Local<Data>(fixed_array->Get(*context, i));
}
}

// v8::Module
extern "C" {
Module::Status v8cxx__module_get_status(const Module* module) {
  return module->GetStatus();
}

void v8cxx__module_get_exception(Local<Value>* local_buf,
                                 const Module* module) {
  new (local_buf) Local<Value>(module->GetException());
}

void v8cxx__module_get_module_requests(Local<FixedArray>* local_buf,
                                       const Module* module) {
  new (local_buf) Local<FixedArray>(module->GetModuleRequests());
}

Location v8cxx__module_source_offset_to_location(const Module* module,
                                                 int offset) {
  return module->SourceOffsetToLocation(offset);
}

int v8cxx__module_get_identity_hash(const Module* module) {
  return module->GetIdentityHash();
}

bool v8cxx__module_instantiate_module(
    Module* module,
    const Local<Context>* context,
    Module::ResolveModuleCallback module_callback) {
  auto result = false;

  return module->InstantiateModule(*context, module_callback)
      .FromMaybe(&result);
}

void v8cxx__module_evaluate(MaybeLocal<Value>* maybe_local_buf,
                            Module* module,
                            const Local<Context>* context) {
  new (maybe_local_buf) MaybeLocal<Value>(module->Evaluate(*context));
}

void v8cxx__module_get_module_namespace(Local<Value>* local_buf,
                                        Module* module) {
  new (local_buf) Local<Value>(module->GetModuleNamespace());
}

// TODO: add this method for module::Module (Rust)
void v8cxx__module_get_unbound_module_script(
    Local<UnboundModuleScript>* local_buf,
    Module* module) {
  new (local_buf) Local<UnboundModuleScript>(module->GetUnboundModuleScript());
}

int v8cxx__module_script_id(const Module* module) {
  return module->ScriptId();
}

bool v8cxx__module_is_graph_async(const Module* module) {
  return module->IsGraphAsync();
}

bool v8cxx__module_has_top_level_await(const Module* module) {
  return module->HasTopLevelAwait();
}

bool v8cxx__module_is_source_text_module(const Module* module) {
  return module->IsSourceTextModule();
}

bool v8cxx__module_is_synthetic_module(const Module* module) {
  return module->IsSyntheticModule();
}

void v8cxx__module_create_synthetic_module(
    Local<Module>* local_buf,
    Isolate* isolate,
    const Local<String>* module_name,
    const Local<String>* export_names,
    size_t export_names_length,
    Module::SyntheticModuleEvaluationSteps evaluation_steps) {
  new (local_buf) Local<Module>(v8::Module::CreateSyntheticModule(
      isolate, *module_name,
      MemorySpan<const Local<v8::String>>(export_names, export_names_length),
      evaluation_steps));
}

bool v8cxx__module_set_synthetic_module_export(
    Module* module,
    Isolate* isolate,
    const Local<String>* export_name,
    const Local<Value>* export_value) {
  auto result = false;

  return module->SetSyntheticModuleExport(isolate, *export_name, *export_value)
      .FromMaybe(&result);
}

// TODO: Module::GetStalledTopLevelAwaitMessages
}

// v8::Local
extern "C" {
void v8cxx__local_empty(Local<Data>* local_buf) {
  new (local_buf) Local<Data>();
}

bool v8cxx__local_is_empty(const Local<Data>* local) {
  return local->IsEmpty();
}
}

// v8::MaybeLocal
extern "C" {
void v8cxx__maybe_local_empty(MaybeLocal<Data>* maybe_local_buf) {
  new (maybe_local_buf) MaybeLocal<Data>(v8::MaybeLocal<v8::Data>());
}

bool v8cxx__maybe_local_is_empty(const MaybeLocal<Data>* maybe_local) {
  return maybe_local->IsEmpty();
}

bool v8cxx__maybe_local_to_local(const MaybeLocal<Data>* maybe_local,
                                 Local<Data>* out) {
  return maybe_local->ToLocal(out);
}

void v8cxx__maybe_local_to_local_checked(Local<Data>* local_buf,
                                         MaybeLocal<Data>* maybe_local) {
  new (local_buf) Local<Data>(maybe_local->ToLocalChecked());
}

void v8cxx__maybe_local_from_maybe(Local<Data>* local_buf,
                                   const MaybeLocal<Data>* maybe_local,
                                   const Local<Data>* default_value) {
  new (local_buf) Local<Data>(maybe_local->FromMaybe(*default_value));
}
}

// v8::ModuleRequest
extern "C" {
void v8cxx__module_request_get_specifier(Local<String>* local_buf,
                                         const ModuleRequest* module_request) {
  new (local_buf) Local<String>(module_request->GetSpecifier());
}

ModuleImportPhase v8cxx__module_request_get_phase(
    const ModuleRequest* module_request) {
  return module_request->GetPhase();
}

void v8cxx__module_request_get_import_attributes(
    Local<FixedArray>* local_buf,
    const ModuleRequest* module_request) {
  new (local_buf) Local<FixedArray>(module_request->GetImportAttributes());
}

int v8cxx__module_request_get_source_offset(
    const ModuleRequest* module_request) {
  return module_request->GetSourceOffset();
}
}

// v8::PrimitiveArray
extern "C" {
void v8cxx__primitive_array_new(Local<PrimitiveArray>* local_buf,
                                Isolate* isolate,
                                int length) {
  new (local_buf)
      Local<PrimitiveArray>(v8::PrimitiveArray::New(isolate, length));
}

int v8cxx__primitive_array_length(const PrimitiveArray* primitive_array) {
  return primitive_array->Length();
}

void v8cxx__primitive_array_set(PrimitiveArray* primitive_array,
                                Isolate* isolate,
                                int index,
                                const Local<Primitive>* item) {
  primitive_array->Set(isolate, index, *item);
}

void v8cxx__primitive_array_get(Local<Primitive>* local_buf,
                                PrimitiveArray* primitive_array,
                                Isolate* isolate,
                                int index) {
  new (local_buf) Local<Primitive>(primitive_array->Get(isolate, index));
}
}

// v8::Private
extern "C" {
void v8cxx__private_name(Local<Value>* local_buf, const Private* private_) {
  new (local_buf) Local<Value>(private_->Name());
}

void v8cxx__private_new(Local<Private>* local_buf,
                        Isolate* isolate,
                        const Local<String>* name) {
  new (local_buf) Local<Private>(v8::Private::New(isolate, *name));
}

void v8cxx__(Local<Private>* local_buf,
             Isolate* isolate,
             const Local<String>* name) {
  new (local_buf) Local<Private>(v8::Private::ForApi(isolate, *name));
}
}

// v8::Signature
extern "C" {
void v8cxx__signature_new(Local<Signature>* local_buf,
                          Isolate* isolate,
                          const Local<FunctionTemplate>* receiver) {
  new (local_buf) Local<Signature>(v8::Signature::New(isolate, *receiver));
}
}

// v8::Template
extern "C" {
void v8cxx__template_set(Template* template_,
                         const Local<Name>* name,
                         const Local<Data>* value,
                         PropertyAttribute attributes) {
  template_->Set(*name, *value, attributes);
}

void v8cxx__template_set_private(Template* template_,
                                 const Local<Private>* name,
                                 const Local<Data>* value,
                                 PropertyAttribute attributes) {
  template_->SetPrivate(*name, *value, attributes);
}

void v8cxx__template_set_with_isolate(Template* template_,
                                      Isolate* isolate,
                                      const char* name,
                                      const Local<Data>* value,
                                      PropertyAttribute attributes) {
  template_->Set(isolate, name, *value, attributes);
}

void v8cxx__template_set_accessor_property(
    Template* template_,
    const Local<Name>* name,
    const Local<FunctionTemplate>* getter,
    const Local<FunctionTemplate>* setter,
    PropertyAttribute attribute) {
  template_->SetAccessorProperty(*name, *getter, *setter, attribute);
}
}

// v8::FunctionTemplate
extern "C" {
void v8cxx__function_template_new(
    Local<FunctionTemplate>* local_buf,
    Isolate* isolate,
    FunctionCallback fn_callback,
    const Local<Value>* data,
    const Local<Signature>* signature,
    int length,
    ConstructorBehavior behavior,
    SideEffectType side_effect_type
    /* TODO: add CFunction and rest of params */) {
  new (local_buf) Local<FunctionTemplate>(
      FunctionTemplate::New(isolate, fn_callback, *data, *signature, length,
                            behavior, side_effect_type));
}

void v8cxx__function_template_get_function(
    MaybeLocal<Function>* maybe_local_buf,
    FunctionTemplate* fn_template,
    const Local<Context>* context) {
  new (maybe_local_buf)
      MaybeLocal<Function>(fn_template->GetFunction(*context));
}

void v8cxx__function_template_instance_template(
    Local<ObjectTemplate>* local_buf,
    FunctionTemplate* fn_template) {
  new (local_buf) Local<ObjectTemplate>(fn_template->InstanceTemplate());
}

void v8cxx__function_template_inherit(FunctionTemplate* fn_template,
                                      const Local<FunctionTemplate>* parent) {
  fn_template->Inherit(*parent);
}

void v8cxx__function_template_prototype_template(
    Local<ObjectTemplate>* local_buf,
    FunctionTemplate* fn_template) {
  new (local_buf) Local<ObjectTemplate>(fn_template->PrototypeTemplate());
}

void v8cxx__function_template_set_prototype_provider_template(
    FunctionTemplate* fn_template,
    const Local<FunctionTemplate>* prototype_provider) {
  fn_template->SetPrototypeProviderTemplate(*prototype_provider);
}

void v8cxx__function_template_set_class_name(FunctionTemplate* fn_template,
                                             const Local<String>* name) {
  fn_template->SetClassName(*name);
}

void v8cxx__function_template_set_interface_name(FunctionTemplate* fn_template,
                                                 const Local<String>* name) {
  fn_template->SetInterfaceName(*name);
}

void v8cxx__function_template_read_only_prototype(
    FunctionTemplate* fn_template) {
  fn_template->ReadOnlyPrototype();
}

void v8cxx__function_template_remove_prototype(FunctionTemplate* fn_template) {
  fn_template->RemovePrototype();
}

bool v8cxx__function_template_has_instance(FunctionTemplate* fn_template,
                                           const Local<Value>* object) {
  return fn_template->HasInstance(*object);
}

void v8cxx__function_template_set_exception_context(
    FunctionTemplate* fn_template,
    ExceptionContext context) {
  fn_template->SetExceptionContext(context);
}

void v8cxx__function_template_set_accept_any_receiver(
    FunctionTemplate* fn_template,
    bool accept) {
  fn_template->SetAcceptAnyReceiver(accept);
}

bool v8cxx__function_template_is_leaf_template_for_api_object(
    const FunctionTemplate* fn_template,
    const Local<Value>* value) {
  return fn_template->IsLeafTemplateForApiObject(*value);
}
}

// v8::Function
extern "C" {
void v8cxx__function_new(MaybeLocal<Function>* maybe_local_buf,
                         const Local<Context>* context,
                         FunctionCallback callback,
                         const Local<Value>* data,
                         int length,
                         ConstructorBehavior behavior,
                         SideEffectType side_effect_type) {
  new (maybe_local_buf) MaybeLocal<Function>(v8::Function::New(
      *context, callback, *data, length, behavior, side_effect_type));
}

void v8cxx__function_new_instance(MaybeLocal<Object>* maybe_local_buf,
                                  const Function* function,
                                  const Local<Context>* context,
                                  int argc,
                                  Local<Value>* argv) {
  new (maybe_local_buf)
      MaybeLocal<Object>(function->NewInstance(*context, argc, argv));
}

void v8cxx__function_new_instance_with_side_effect_type(
    MaybeLocal<Object>* maybe_local_buf,
    const Function* function,
    const Local<Context>* context,
    int argc,
    Local<Value>* argv,
    SideEffectType side_effect_type) {
  new (maybe_local_buf)
      MaybeLocal<Object>(function->NewInstanceWithSideEffectType(
          *context, argc, argv, side_effect_type));
}

void v8cxx__function_set_name(Function* function, const Local<String>* name) {
  function->SetName(*name);
}

void v8cxx__function_get_name(Local<Value>* local_buf,
                              const Function* function) {
  new (local_buf) Local<Value>(function->GetName());
}

void v8cxx__function_get_inferred_name(Local<Value>* local_buf,
                                       const Function* function) {
  new (local_buf) Local<Value>(function->GetInferredName());
}

int v8cxx__function_get_script_line_number(const Function* function) {
  return function->GetScriptLineNumber();
}

int v8cxx__function_get_script_column_number(const Function* function) {
  return function->GetScriptColumnNumber();
}

int v8cxx__function_script_id(const Function* function) {
  return function->ScriptId();
}

void v8cxx__function_get_bound_function(Local<Value>* local_buf,
                                        const Function* function) {
  new (local_buf) Local<Value>(function->GetBoundFunction());
}

void v8cxx__function_function_proto_to_string(
    MaybeLocal<String>* maybe_local_buf,
    Function* function,
    const Local<Context>* context) {
  new (maybe_local_buf)
      MaybeLocal<String>(function->FunctionProtoToString(*context));
}

void v8cxx__function_get_script_origin(ScriptOrigin* buf,
                                       const Function* function) {
  new (buf) ScriptOrigin(function->GetScriptOrigin());
}
}

// v8::FunctionCallbackInfo<T>
extern "C" {
int v8cxx__function_callback_info_length(
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  return fn_callback_info->Length();
}

void v8cxx__function_callback_info_at(
    Local<Value>* local_buf,
    const FunctionCallbackInfo<Value>* fn_callback_info,
    int index) {
  new (local_buf) Local<Value>((*fn_callback_info)[index]);
}

void v8cxx__function_callback_info_this(
    Local<Object>* local_buf,
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  new (local_buf) Local<Object>(fn_callback_info->This());
}

void v8cxx__function_callback_info_new_target(
    Local<Value>* local_buf,
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  new (local_buf) Local<Value>(fn_callback_info->NewTarget());
}

bool v8cxx__function_callback_info_is_construct_call(
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  return fn_callback_info->IsConstructCall();
}

void v8cxx__function_callback_info_data(
    Local<Value>* local_buf,
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  new (local_buf) Local<Value>(fn_callback_info->Data());
}

Isolate* v8cxx__function_callback_info_get_isolate(
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  return fn_callback_info->GetIsolate();
}

void v8cxx__function_callback_info_get_return_value(
    ReturnValue<Value>* buf,
    const FunctionCallbackInfo<Value>* fn_callback_info) {
  new (buf) ReturnValue<Value>(fn_callback_info->GetReturnValue());
}
}

// v8::ReturnValue<T>
extern "C" {
void v8cxx__return_value_set(ReturnValue<Value>* return_value,
                             const Local<Value>* value) {
  return_value->Set(*value);
}

void v8cxx__return_value_set_bool(ReturnValue<Value>* return_value,
                                  bool value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_f64(ReturnValue<Value>* return_value,
                                 double value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i16(ReturnValue<Value>* return_value,
                                 int16_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i32(ReturnValue<Value>* return_value,
                                 int32_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_i64(ReturnValue<Value>* return_value,
                                 int64_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u16(ReturnValue<Value>* return_value,
                                 uint16_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u32(ReturnValue<Value>* return_value,
                                 uint32_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_u64(ReturnValue<Value>* return_value,
                                 uint64_t value) {
  return_value->Set(value);
}

void v8cxx__return_value_set_null(ReturnValue<Value>* return_value) {
  return_value->SetNull();
}

void v8cxx__return_value_set_undefined(ReturnValue<Value>* return_value) {
  return_value->SetUndefined();
}

void v8cxx__return_value_set_false(ReturnValue<Value>* return_value) {
  return_value->SetFalse();
}

void v8cxx__return_value_set_empty_string(ReturnValue<Value>* return_value) {
  return_value->SetEmptyString();
}

Isolate* v8cxx__return_value_get_isolate(
    const ReturnValue<Value>* return_value) {
  return return_value->GetIsolate();
}

void v8cxx__return_value_get(Local<Value>* local_buf,
                             const ReturnValue<Value>* return_value) {
  new (local_buf) Local<Value>(return_value->Get());
}
}

// v8::ScriptOrigin
extern "C" {
void v8cxx__script_origin_new(ScriptOrigin* buf,
                              const Local<Value>* resource_name,
                              int resource_line_offset,
                              int resource_column_offset,
                              bool resource_is_shared_cross_origin,
                              int script_id,
                              const Local<Value>* source_map_url,
                              bool resource_is_opaque,
                              bool is_wasm,
                              bool is_module,
                              const Local<Data>* host_defined_options) {
  new (buf) ScriptOrigin(ScriptOrigin(
      *resource_name, resource_line_offset, resource_column_offset,
      resource_is_shared_cross_origin, script_id, *source_map_url,
      resource_is_opaque, is_wasm, is_module, *host_defined_options));
}

void v8cxx__script_origin_resource_name(Local<Value>* local_buf,
                                        const ScriptOrigin* script_origin) {
  new (local_buf) Local<Value>(script_origin->ResourceName());
}

int v8cxx__script_origin_line_offset(const ScriptOrigin* script_origin) {
  return script_origin->LineOffset();
}

int v8cxx__script_origin_column_offset(const ScriptOrigin* script_origin) {
  return script_origin->ColumnOffset();
}

int v8cxx__script_origin_script_id(const ScriptOrigin* script_origin) {
  return script_origin->ScriptId();
}

void v8cxx__script_origin_source_map_url(Local<Value>* local_buf,
                                         const ScriptOrigin* script_origin) {
  new (local_buf) Local<Value>(script_origin->SourceMapUrl());
}

void v8cxx__script_origin_get_host_defined_options(
    Local<Data>* local_buf,
    const ScriptOrigin* script_origin) {
  new (local_buf) Local<Data>(script_origin->GetHostDefinedOptions());
}

bool v8cxx__script_origin_is_module(const ScriptOrigin* script_origin) {
  return script_origin->Options().IsModule();
}

bool v8cxx__script_origin_is_opaque(const ScriptOrigin* script_origin) {
  return script_origin->Options().IsOpaque();
}

bool v8cxx__script_origin_is_shared_cross_origin(
    const ScriptOrigin* script_origin) {
  return script_origin->Options().IsSharedCrossOrigin();
}

bool v8cxx__script_origin_is_wasm(const ScriptOrigin* script_origin) {
  return script_origin->Options().IsWasm();
}
}

// v8::ObjectTemplate
extern "C" {
void v8cxx__object_template_new(Local<ObjectTemplate>* local_buf,
                                Isolate* isolate,
                                const Local<FunctionTemplate>* constructor) {
  new (local_buf)
      Local<ObjectTemplate>(ObjectTemplate::New(isolate, *constructor));
}

void v8cxx__object_template_new_instance(MaybeLocal<Object>* local_buf,
                                         ObjectTemplate* object_template,
                                         const Local<Context>* context) {
  new (local_buf) MaybeLocal<Object>(object_template->NewInstance(*context));
}

void v8cxx__object_template_set_named_property_handler(
    ObjectTemplate* object_template,
    NamedPropertyGetterCallback getter,
    NamedPropertySetterCallback setter,
    NamedPropertyQueryCallback query,
    NamedPropertyDeleterCallback deleter,
    NamedPropertyEnumeratorCallback enumerator,
    NamedPropertyDefinerCallback definer,
    NamedPropertyDescriptorCallback descriptor,
    const Local<Value>* data,
    PropertyHandlerFlags flags) {
  object_template->SetHandler(NamedPropertyHandlerConfiguration(
      getter, setter, query, deleter, enumerator, definer, descriptor, *data,
      flags));
}

void v8cxx__object_template_set_indexed_property_handler(
    ObjectTemplate* object_template,
    IndexedPropertyGetterCallbackV2 getter,
    IndexedPropertySetterCallbackV2 setter,
    IndexedPropertyQueryCallbackV2 query,
    IndexedPropertyDeleterCallbackV2 deleter,
    IndexedPropertyEnumeratorCallback enumerator,
    IndexedPropertyDefinerCallbackV2 definer,
    IndexedPropertyDescriptorCallbackV2 descriptor,
    const Local<Value>* data,
    PropertyHandlerFlags flags) {
  object_template->SetHandler(IndexedPropertyHandlerConfiguration(
      getter, setter, query, deleter, enumerator, definer, descriptor, *data,
      flags));
}

void v8cxx__object_template_set_call_as_function_handler(
    ObjectTemplate* object_template,
    FunctionCallback callback,
    const Local<Value>* data) {
  object_template->SetCallAsFunctionHandler(callback, *data);
}

void v8cxx__object_template_mark_as_undetectable(
    ObjectTemplate* object_template) {
  object_template->MarkAsUndetectable();
}

// TODO: ObjectTemplate::{
//   SetAccessCheckCallback,
//   SetAccessCheckCallbackAndHandler
// }

int v8cxx__object_template_internal_field_count(
    const ObjectTemplate* object_template) {
  return object_template->InternalFieldCount();
}

void v8cxx__object_template_set_internal_field_count(
    ObjectTemplate* object_template,
    int count) {
  object_template->SetInternalFieldCount(count);
}

bool v8cxx__object_template_is_immutable_proto(
    const ObjectTemplate* object_template) {
  return object_template->IsImmutableProto();
}

// TODO: ObjectTemplate::{SetCodeLike, IsCodeLike}
}

// v8::PropertyCallbackInfo<T>
extern "C" {
Isolate* v8cxx__property_callback_info_get_isolate(
    const PropertyCallbackInfo<Value>* pci) {
  return pci->GetIsolate();
}

void v8cxx__property_callback_info_data(
    Local<Value>* local_buf,
    const PropertyCallbackInfo<Value>* pci) {
  new (local_buf) Local<Value>(pci->Data());
}

void v8cxx__property_callback_info_this(
    Local<Object>* local_buf,
    const PropertyCallbackInfo<Value>* pci) {
  new (local_buf) Local<Object>(pci->This());
}

void v8cxx__property_callback_info_holder(
    Local<Object>* local_buf,
    const PropertyCallbackInfo<Value>* pci) {
  new (local_buf) Local<Object>(pci->HolderV2());
}

void v8cxx__property_callback_info_get_return_value(
    ReturnValue<Value>* buf,
    const PropertyCallbackInfo<Value>* pci) {
  new (buf) ReturnValue<Value>(pci->GetReturnValue());
}

bool v8cxx__property_callback_info_should_throw_on_error(
    const PropertyCallbackInfo<Value>* pci) {
  return pci->ShouldThrowOnError();
}
}

// v8::Number
extern "C" {
void v8cxx__number_new(Local<Number>* local_buf,
                       Isolate* isolate,
                       double value) {
  new (local_buf) Local<Number>(v8::Number::New(isolate, value));
}

double v8cxx__number_value(const Number* number) {
  return number->Value();
}
}

// v8::Integer
extern "C" {
void v8cxx__integer_new(Local<Integer>* local_buf,
                        Isolate* isolate,
                        int32_t value) {
  new (local_buf) Local<Integer>(v8::Integer::New(isolate, value));
}

void v8cxx__integer_new_from_unsigned(Local<Integer>* local_buf,
                                      Isolate* isolate,
                                      uint32_t value) {
  new (local_buf) Local<Integer>(v8::Integer::NewFromUnsigned(isolate, value));
}

int64_t v8cxx__integer_value(const Integer* integer) {
  return integer->Value();
}
}

// v8::Array
extern "C" {
uint32_t v8cxx__array_length(const Array* array) {
  return array->Length();
}

void v8cxx__array_new(Local<Array>* local_buf, Isolate* isolate, int length) {
  new (local_buf) Local<Array>(v8::Array::New(isolate, length));
}

void v8cxx__array_new_with_elements(Local<Array>* local_buf,
                                    Isolate* isolate,
                                    const Local<Value>* elements,
                                    size_t length) {
  new (local_buf) Local<Array>(
      v8::Array::New(isolate, const_cast<Local<Value>*>(elements), length));
}
}

// v8::UnboundModuleScript
extern "C" {
void v8cxx__unbound_module_script_get_source_url(Local<Value>* local_buf,
                                                 UnboundModuleScript* ums) {
  new (local_buf) Local<Value>(ums->GetSourceURL());
}

void v8cxx__unbound_module_script_get_source_mapping_url(
    Local<Value>* local_buf,
    UnboundModuleScript* ums) {
  new (local_buf) Local<Value>(ums->GetSourceMappingURL());
}
}

// v8::UnboundScript
extern "C" {
void v8cxx__unbound_script_bind_to_current_context(Local<Script>* local_buf,
                                                   UnboundScript* us) {
  new (local_buf) Local<Script>(us->BindToCurrentContext());
}

int v8cxx__unbound_script_get_id(const UnboundScript* us) {
  return us->GetId();
}

void v8cxx__unbound_script_get_script_name(Local<Value>* local_buf,
                                           UnboundScript* us) {
  new (local_buf) Local<Value>(us->GetScriptName());
}

void v8cxx__unbound_script_get_source_url(Local<Value>* local_buf,
                                          UnboundScript* us) {
  new (local_buf) Local<Value>(us->GetSourceURL());
}

void v8cxx__unbound_script_get_source_mapping_url(Local<Value>* local_buf,
                                                  UnboundScript* us) {
  new (local_buf) Local<Value>(us->GetSourceMappingURL());
}

int v8cxx__unbound_script_get_line_number(UnboundScript* us, int code_pos) {
  return us->GetLineNumber(code_pos);
}

int v8cxx__unbound_script_get_column_number(UnboundScript* us, int code_pos) {
  return us->GetColumnNumber(code_pos);
}
}

// v8::ScriptCompiler
extern "C" {
void v8cxx__script_compiler_compile_unbound_script(
    MaybeLocal<UnboundScript>* maybe_local_buf,
    Isolate* isolate,
    ScriptCompiler::Source* source) {
  new (maybe_local_buf) MaybeLocal<UnboundScript>(
      ScriptCompiler::CompileUnboundScript(isolate, source));
}

void v8cxx__script_compiler_compile(MaybeLocal<Script>* maybe_local_buf,
                                    const Local<Context>* context,
                                    ScriptCompiler::Source* source) {
  new (maybe_local_buf)
      MaybeLocal<Script>(v8::ScriptCompiler::Compile(*context, source));
}

void v8cxx__script_compiler_compile_module(MaybeLocal<Module>* maybe_local_buf,
                                           Isolate* isolate,
                                           ScriptCompiler::Source* source) {
  new (maybe_local_buf)
      MaybeLocal<Module>(ScriptCompiler::CompileModule(isolate, source));
}

void v8cxx__script_compiler_compile_function(
    MaybeLocal<Function>* maybe_local_buf,
    const Local<Context>* context,
    ScriptCompiler::Source* source,
    size_t arguments_count,
    Local<String>* arguments,
    size_t context_extension_count,
    Local<Object>* context_extensions) {
  new (maybe_local_buf)
      MaybeLocal<Function>(v8::ScriptCompiler::CompileFunction(
          *context, source, arguments_count, arguments, context_extension_count,
          context_extensions));
}

// TODO: implement other methods if needed
}

// v8::ScriptCompiler::Source
extern "C" {
void v8cxx__script_compiler__source_new(ScriptCompiler::Source* buf,
                                        const Local<String>* source_string,
                                        const ScriptOrigin* origin) {
  new (buf) ScriptCompiler::Source(*source_string, *origin);
}
}

// v8::Promise
extern "C" {
void v8cxx__promise_catch(MaybeLocal<Promise>* maybe_local_buf,
                          Promise* promise,
                          const Local<Context>* context,
                          const Local<Function>* handler) {
  new (maybe_local_buf) MaybeLocal<Promise>(promise->Catch(*context, *handler));
}

void v8cxx__promise_then(MaybeLocal<Promise>* maybe_local_buf,
                         Promise* promise,
                         const Local<Context>* context,
                         const Local<Function>* handler) {
  new (maybe_local_buf) MaybeLocal<Promise>(promise->Then(*context, *handler));
}

bool v8cxx__promise_has_handler(const Promise* promise) {
  return promise->HasHandler();
}

void v8cxx__promise_result(Local<Value>* local_buf, Promise* promise) {
  new (local_buf) Local<Value>(promise->Result());
}

Promise::PromiseState v8cxx__promise_state(Promise* promise) {
  return promise->State();
}

void v8cxx__promise_mark_as_handled(Promise* promise) {
  promise->MarkAsHandled();
}

void v8cxx__promise_mark_as_silent(Promise* promise) {
  promise->MarkAsSilent();
}
}

// v8::Promise::Resolver
extern "C" {
void v8cxx__promise__resolver_new(
    MaybeLocal<Promise::Resolver>* maybe_local_buf,
    const Local<Context>* context) {
  new (maybe_local_buf)
      MaybeLocal<Promise::Resolver>(Promise::Resolver::New(*context));
}

void v8cxx__promise__resolver_get_promise(Local<Promise>* local_buf,
                                          Promise::Resolver* pr) {
  new (local_buf) Local<Promise>(pr->GetPromise());
}

bool v8cxx__promise__resolver_resolve(Promise::Resolver* pr,
                                      const Local<Context>* context,
                                      const Local<Value>* value) {
  auto result = false;

  return pr->Resolve(*context, *value).FromMaybe(&result);
}

bool v8cxx__promise__resolver_reject(Promise::Resolver* pr,
                                     const Local<Context>* context,
                                     const Local<Value>* value) {
  auto result = false;

  return pr->Reject(*context, *value).FromMaybe(&result);
}
}

// v8::TryCatch
extern "C" {
class TryCatch {
 public:
  TryCatch(Isolate* isolate) : try_catch(isolate) {}
  ~TryCatch() {}

  v8::TryCatch& get() { return this->try_catch; }
  const v8::TryCatch& get() const { return this->try_catch; }

 private:
  v8::TryCatch try_catch;
};

void v8cxx__try_catch_new(::TryCatch* buf, Isolate* isolate) {
  new (buf)::TryCatch(isolate);
}

void v8cxx__try_catch_drop(::TryCatch* try_catch) {
  try_catch->~TryCatch();
}

bool v8cxx__try_catch_has_caught(const ::TryCatch* try_catch) {
  return try_catch->get().HasCaught();
}

bool v8cxx__try_catch_can_continue(const ::TryCatch* try_catch) {
  return try_catch->get().CanContinue();
}

bool v8cxx__try_catch_has_terminated(const ::TryCatch* try_catch) {
  return try_catch->get().HasTerminated();
}

void v8cxx__try_catch_rethrow(Local<Value>* local_buf, ::TryCatch* try_catch) {
  new (local_buf) Local<Value>(try_catch->get().ReThrow());
}

void v8cxx__try_catch_exception(Local<Value>* local_buf,
                                const ::TryCatch* try_catch) {
  new (local_buf) Local<Value>(try_catch->get().Exception());
}

void v8cxx__try_catch_stack_trace(MaybeLocal<Value>* maybe_local_buf,
                                  const Local<Context>* context,
                                  const Local<Value>* exception) {
  new (maybe_local_buf)
      MaybeLocal<Value>(v8::TryCatch::StackTrace(*context, *exception));
}

void v8cxx__try_catch_message(Local<Message>* local_buf,
                              const ::TryCatch* try_catch) {
  new (local_buf) Local<Message>(try_catch->get().Message());
}

void v8cxx__try_catch_reset(::TryCatch* try_catch) {
  try_catch->get().Reset();
}

void v8cxx__try_catch_set_verbose(::TryCatch* try_catch, bool value) {
  try_catch->get().SetVerbose(value);
}

bool v8cxx__try_catch_is_verbose(const ::TryCatch* try_catch) {
  return try_catch->get().IsVerbose();
}

void v8cxx__try_catch_set_capture_message(::TryCatch* try_catch, bool value) {
  try_catch->get().SetCaptureMessage(value);
}
}

// v8::MicrotaskQueue
extern "C" {
MicrotaskQueue* v8cxx__microtask_queue_new(Isolate* isolate,
                                           MicrotasksPolicy policy) {
  return MicrotaskQueue::New(isolate, policy).release();
}

void v8cxx__microtask_queue_drop(MicrotaskQueue* self) {
  self->~MicrotaskQueue();
}

void v8cxx__microtask_queue_enqueue_microtask(
    MicrotaskQueue* self,
    Isolate* isolate,
    const Local<Function>* microtask) {
  self->EnqueueMicrotask(isolate, *microtask);
}

void v8cxx__microtask_queue_enqueue_microtask_with_callback(
    MicrotaskQueue* self,
    Isolate* isolate,
    MicrotaskCallback callback,
    void* data) {
  self->EnqueueMicrotask(isolate, callback, data);
}

void v8cxx__microtask_queue_add_microtasks_completed_callback(
    MicrotaskQueue* self,
    MicrotasksCompletedCallbackWithData callback,
    void* data) {
  self->AddMicrotasksCompletedCallback(callback, data);
}

void v8cxx__microtask_queue_remove_microtasks_completed_callback(
    MicrotaskQueue* self,
    MicrotasksCompletedCallbackWithData callback,
    void* data) {
  self->RemoveMicrotasksCompletedCallback(callback, data);
}

void v8cxx__microtask_queue_perform_checkpoint(MicrotaskQueue* self,
                                               Isolate* isolate) {
  self->PerformCheckpoint(isolate);
}

bool v8cxx__microtask_queue_is_running_microtasks(const MicrotaskQueue* self) {
  return self->IsRunningMicrotasks();
}

int v8cxx__microtask_queue_get_microtasks_scope_depth(
    const MicrotaskQueue* self) {
  return self->GetMicrotasksScopeDepth();
}
}