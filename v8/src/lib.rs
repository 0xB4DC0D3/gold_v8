use bindings::{v8cxx__major_version, v8cxx__minor_version, v8cxx__patch_level};

#[allow(non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod array_buffer;
pub mod bigint;
pub mod boolean;
pub mod context;
pub mod data;
pub mod isolate;
pub mod local;
pub mod microtask_queue;
pub mod number;
pub mod object;
pub mod object_template;
pub mod platform;
pub mod primitive;
pub mod scope;
pub mod script;
pub mod string;
pub mod symbol;
pub mod v8;
pub mod value;

pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        v8cxx__major_version, v8cxx__minor_version, v8cxx__patch_level
    )
}

#[repr(C)]
pub enum NewStringType {
    Normal,
    Internalized,
}
