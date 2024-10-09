#include <v8.h>

static uint32_t v8cxx__major_version = V8_MAJOR_VERSION;
static uint32_t v8cxx__minor_version = V8_MINOR_VERSION;
static uint32_t v8cxx__patch_level = V8_PATCH_LEVEL;
static size_t v8cxx__sizeof_isolate__createparams =
    sizeof(v8::Isolate::CreateParams);
static size_t v8cxx__sizeof_handlescope = sizeof(v8::HandleScope);
static size_t v8cxx__sizeof_scriptorigin = sizeof(v8::ScriptOrigin);
static size_t v8cxx__sizeof_functioncallbackinfo =
    sizeof(v8::FunctionCallbackInfo<void>);
static size_t v8cxx__sizeof_propertycallbackinfo =
    sizeof(v8::PropertyCallbackInfo<void>);
static size_t v8cxx__sizeof_scriptcompiler__source =
    sizeof(v8::ScriptCompiler::Source);