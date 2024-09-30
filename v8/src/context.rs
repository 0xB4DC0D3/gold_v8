use crate::{data::traits::Data, isolate::Isolate, local::Local, scope::HandleScope};

extern "C" {
    fn v8cxx__context_new(local_buf: *mut Local<Context>, isolate: *mut Isolate);
    fn v8cxx__context_enter(this: *mut Context);
    fn v8cxx__context_exit(this: *mut Context);
}

#[repr(C)]
pub struct Context([u8; 0]);

impl Context {
    #[inline(always)]
    pub fn new(handle_scope: &mut HandleScope) -> Local<Self> {
        let mut local_context = Local::<Self>::empty();

        unsafe {
            v8cxx__context_new(&mut local_context, handle_scope.get_isolate().unwrap());
        }

        local_context.enter();
        local_context
    }

    #[inline(always)]
    fn enter(&mut self) {
        unsafe { v8cxx__context_enter(self) };
    }

    #[inline(always)]
    fn exit(&mut self) {
        unsafe { v8cxx__context_exit(self) };
    }
}

impl Data for Context {}

impl Drop for Context {
    fn drop(&mut self) {
        self.exit();
    }
}
