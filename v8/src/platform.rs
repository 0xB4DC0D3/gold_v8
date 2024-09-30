extern "C" {
    fn v8cxx__platform__new_default_platform(
        thread_pool_size: i32,
        idle_task_support: IdleTaskSupport,
        in_process_stack_dumping: InProcessStackDumping,
        priority_mode: PriorityMode,
    ) -> *mut Platform;

    fn v8cxx__platform_drop(this: *mut Platform);
}

#[repr(C)]
pub enum IdleTaskSupport {
    Disable,
    Enable,
}

#[repr(C)]
pub enum InProcessStackDumping {
    Disable,
    Enable,
}

#[repr(C)]
pub enum PriorityMode {
    DontApply,
    Apply,
}

#[repr(C)]
pub struct Platform([u8; 0]);

impl Platform {
    #[inline(always)]
    pub fn new<'a>(
        thread_pool_size: u32,
        idle_task_support: IdleTaskSupport,
        in_process_stack_dumping: InProcessStackDumping,
        priority_mode: PriorityMode,
    ) -> Option<&'a mut Self> {
        unsafe {
            v8cxx__platform__new_default_platform(
                thread_pool_size.min(16) as i32,
                idle_task_support,
                in_process_stack_dumping,
                priority_mode,
            )
            .as_mut()
        }
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        unsafe {
            v8cxx__platform_drop(self);
        }
    }
}
