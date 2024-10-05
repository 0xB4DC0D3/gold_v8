use crate::{data::traits::Data, isolate::Isolate, local::Local};

extern "C" {
    fn v8cxx__signature_new(
        local_buf: *mut Local<Signature>,
        isolate: *mut Isolate,
        receiver: *const Local<FunctionTemplate>,
    );
}

#[repr(C)]
pub struct Signature([u8; 0]);

impl Signature {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, receiver: &Local<FunctionTemplate>) -> Local<Self> {
        let mut local_signature = Local::empty();

        unsafe { v8cxx__signature_new(&mut local_signature, isolate, receiver) };

        local_signature
    }
}

impl Data for Signature {}
