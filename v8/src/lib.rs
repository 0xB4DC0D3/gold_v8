use bindings::{v8cxx__major_version, v8cxx__minor_version, v8cxx__patch_level};

#[allow(non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod array;
pub mod array_buffer;
pub mod bigint;
pub mod boolean;
pub mod context;
pub mod data;
pub mod fixed_array;
pub mod function;
pub mod function_callback_info;
pub mod function_template;
pub mod integer;
pub mod isolate;
pub mod local;
pub mod microtask_queue;
pub mod module;
pub mod module_request;
pub mod name;
pub mod number;
pub mod numeric;
pub mod object;
pub mod object_template;
pub mod platform;
pub mod primitive;
pub mod primitive_array;
pub mod private;
pub mod property_callback_info;
pub mod return_value;
pub mod scope;
pub mod script;
pub mod script_origin;
pub mod signature;
pub mod string;
pub mod symbol;
pub mod template;
pub mod unbound_module_script;
pub mod v8;
pub mod value;

pub(crate) mod c_support;

pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        v8cxx__major_version, v8cxx__minor_version, v8cxx__patch_level
    )
}
