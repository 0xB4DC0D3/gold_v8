extern "C" {
    fn v8cxx__arraybuffer__allocator_new_default_allocator() -> *mut ArrayBufferAllocator;
}
#[repr(C)]
pub struct ArrayBufferAllocator([u8; 0]);

impl ArrayBufferAllocator {
    #[inline(always)]
    pub fn new<'a>() -> Option<&'a mut Self> {
        unsafe { v8cxx__arraybuffer__allocator_new_default_allocator().as_mut() }
    }
}
