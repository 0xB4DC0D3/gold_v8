use std::mem::MaybeUninit;

use crate::{
    bindings,
    context::Context,
    function::Function,
    isolate::Isolate,
    local::{Local, MaybeLocal},
    module::Module,
    object::Object,
    script::Script,
    script_origin::ScriptOrigin,
    string::String,
    unbound_script::UnboundScript,
};

extern "C" {
    fn v8cxx__script_compiler_compile_unbound_script(
        maybe_local_buf: *mut MaybeLocal<UnboundScript>,
        isolate: *mut Isolate,
        source: *mut Source,
    );
    fn v8cxx__script_compiler_compile(
        maybe_local_buf: *mut MaybeLocal<Script>,
        context: *const Local<Context>,
        source: *mut Source,
    );
    fn v8cxx__script_compiler_compile_module(
        maybe_local_buf: *mut MaybeLocal<Module>,
        isolate: *mut Isolate,
        source: *mut Source,
    );
    fn v8cxx__script_compiler_compile_function(
        maybe_local_buf: *mut MaybeLocal<Function>,
        context: *const Local<Context>,
        source: *mut Source,
        arguments_count: usize,
        arguments: *mut Local<String>,
        context_extension_count: usize,
        context_extensions: *mut Local<Object>,
    );
}

#[repr(C)]
pub struct ScriptCompiler([u8; 0]);

impl ScriptCompiler {
    #[inline(always)]
    pub fn compile_unbound_script(
        isolate: &mut Isolate,
        source: &mut Source,
    ) -> MaybeLocal<UnboundScript> {
        let mut maybe_local_unbound_script = MaybeLocal::empty();

        unsafe {
            v8cxx__script_compiler_compile_unbound_script(
                &mut maybe_local_unbound_script,
                isolate,
                source,
            )
        };

        maybe_local_unbound_script
    }

    #[inline(always)]
    pub fn compile(context: &Local<Context>, source: &mut Source) -> MaybeLocal<Script> {
        let mut maybe_local_script = MaybeLocal::empty();

        unsafe { v8cxx__script_compiler_compile(&mut maybe_local_script, context, source) };

        maybe_local_script
    }

    #[inline(always)]
    pub fn compile_module(isolate: &mut Isolate, source: &mut Source) -> MaybeLocal<Module> {
        let mut maybe_local_module = MaybeLocal::empty();

        unsafe { v8cxx__script_compiler_compile_module(&mut maybe_local_module, isolate, source) };

        maybe_local_module
    }

    #[inline(always)]
    pub fn compile_function(
        context: &Local<Context>,
        source: &mut Source,
        arguments: &mut Vec<Local<String>>,
        context_extensions: &mut Vec<Local<Object>>,
    ) -> MaybeLocal<Function> {
        let mut maybe_local_function = MaybeLocal::empty();

        unsafe {
            v8cxx__script_compiler_compile_function(
                &mut maybe_local_function,
                context,
                source,
                arguments.len(),
                arguments.as_mut_ptr(),
                context_extensions.len(),
                context_extensions.as_mut_ptr(),
            )
        };

        maybe_local_function
    }
}

extern "C" {
    fn v8cxx__script_compiler__source_new(
        buf: *mut Source,
        source_string: *const Local<String>,
        origin: *const ScriptOrigin,
    );
}

#[repr(C)]
pub struct Source([u8; bindings::v8cxx__sizeof_scriptcompiler__source]);

impl Source {
    #[inline(always)]
    pub fn new(source_string: &Local<String>, origin: &ScriptOrigin) -> Self {
        unsafe {
            let mut source = MaybeUninit::uninit();

            v8cxx__script_compiler__source_new(source.as_mut_ptr(), source_string, origin);

            source.assume_init()
        }
    }
}
