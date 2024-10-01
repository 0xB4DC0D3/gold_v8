use crate::{array_buffer::ArrayBufferAllocator, bindings};

extern "C" {
    fn v8cxx__isolate__createparams(
        array_buffer_allocator: *mut ArrayBufferAllocator,
    ) -> IsolateCreateParams;

    fn v8cxx__isolate_new(create_params: *const IsolateCreateParams) -> *mut Isolate;
    fn v8cxx__isolate_enter(this: *mut Isolate);
    fn v8cxx__isolate_exit(this: *mut Isolate);
    fn v8cxx__isolate_get_current(this: *mut Isolate) -> *mut Isolate;
}

#[repr(C)]
pub struct IsolateCreateParams([u8; bindings::v8cxx__sizeof_isolate__createparams]);

impl IsolateCreateParams {
    #[inline(always)]
    pub fn new(array_buffer_allocator: &mut ArrayBufferAllocator) -> Self {
        unsafe { v8cxx__isolate__createparams(array_buffer_allocator) }
    }
}

#[repr(C)]
pub struct Isolate([u8; 0]);

impl Isolate {
    #[inline(always)]
    pub fn new(create_params: &IsolateCreateParams) -> Option<&mut Self> {
        unsafe {
            let mut isolate = v8cxx__isolate_new(create_params).as_mut();

            isolate.as_mut().unwrap().enter();
            isolate
        }
    }

    #[inline(always)]
    fn enter(&mut self) {
        unsafe { v8cxx__isolate_enter(self) };
    }

    #[inline(always)]
    fn exit(&mut self) {
        unsafe { v8cxx__isolate_exit(self) };
    }

    #[inline(always)]
    pub fn get_current(&mut self) -> Option<&mut Self> {
        unsafe { v8cxx__isolate_get_current(self).as_mut() }
    }
}

impl Drop for Isolate {
    fn drop(&mut self) {
        println!("Isolate::drop");
        self.exit();
    }
}
