use crate::platform::Platform;

extern "C" {
    fn v8cxx__v8__initialize_platform(platform: *mut Platform);
    fn v8cxx__v8__initialize() -> bool;
    fn v8cxx__v8__dispose_platform();
    fn v8cxx__v8__dispose();
}

#[inline(always)]
pub fn initialize_platform(platform: &mut Platform) {
    unsafe { v8cxx__v8__initialize_platform(platform) };
}

#[inline(always)]
pub fn initialize() -> bool {
    unsafe { v8cxx__v8__initialize() }
}

#[inline(always)]
pub fn dispose_platform() {
    unsafe { v8cxx__v8__dispose_platform() };
}

#[inline(always)]
pub fn dispose() {
    unsafe { v8cxx__v8__dispose() };
}
