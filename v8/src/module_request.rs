use crate::{fixed_array::FixedArray, local::Local, string::String};

extern "C" {
    fn v8cxx__module_request_get_specifier(
        local_buf: *mut Local<String>,
        module_request: *const ModuleRequest,
    );
    fn v8cxx__module_request_get_phase(module_request: *const ModuleRequest) -> ModuleImportPhase;
    fn v8cxx__module_request_get_import_attributes(
        local_buf: *mut Local<FixedArray>,
        module_request: *const ModuleRequest,
    );
    fn v8cxx__module_request_get_source_offset(module_request: *const ModuleRequest) -> i32;
}

#[repr(C)]
pub struct ModuleRequest([u8; 0]);

impl ModuleRequest {
    #[inline(always)]
    pub fn get_specifier(&self) -> Local<String> {
        let mut local_string = Local::empty();

        unsafe { v8cxx__module_request_get_specifier(&mut local_string, self) };

        local_string
    }

    #[inline(always)]
    pub fn get_phase(&self) -> ModuleImportPhase {
        unsafe { v8cxx__module_request_get_phase(self) }
    }

    #[inline(always)]
    pub fn get_import_attributes(&self) -> Local<FixedArray> {
        let mut local_fixed_array = Local::empty();

        unsafe {
            v8cxx__module_request_get_import_attributes(&mut local_fixed_array, self);
        }

        local_fixed_array
    }

    #[inline(always)]
    pub fn get_source_offset(&self) -> i32 {
        unsafe { v8cxx__module_request_get_source_offset(self) }
    }
}

#[repr(C)]
pub enum ModuleImportPhase {
    Source,
    Evaluation,
}
