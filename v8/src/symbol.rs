use crate::{
    data::traits::Data, isolate::Isolate, local::Local, name::traits::Name,
    primitive::traits::Primitive, string::String, value::traits::Value,
};

extern "C" {
    fn v8cxx__symbol_new(
        local_buf: *mut Local<Symbol>,
        isolate: *mut Isolate,
        name: *const Local<String>,
    );
    fn v8cxx__symbol_for(
        local_buf: *mut Local<Symbol>,
        isolate: *mut Isolate,
        description: *const Local<String>,
    );
    fn v8cxx__symbol_for_api(
        local_buf: *mut Local<Symbol>,
        isolate: *mut Isolate,
        description: *const Local<String>,
    );
    fn v8cxx__symbol_get_async_iterator(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_has_instance(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_is_concat_spreadable(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_iterator(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_match(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_replace(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_search(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_split(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_to_primitive(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_to_string_tag(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);
    fn v8cxx__symbol_get_unscopables(local_buf: *mut Local<Symbol>, isolate: *mut Isolate);

}

#[repr(C)]
pub struct Symbol([u8; 0]);

impl Symbol {
    #[inline(always)]
    pub fn new(isolate: &mut Isolate, name: &Local<String>) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_new(&mut local_symbol, isolate, name);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn for_symbol(isolate: &mut Isolate, description: &Local<String>) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_for(&mut local_symbol, isolate, description);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn for_api_symbol(isolate: &mut Isolate, description: &Local<String>) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_for_api(&mut local_symbol, isolate, description);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_async_iterator(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_async_iterator(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_has_instance(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_has_instance(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_is_concat_spreadable(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_is_concat_spreadable(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_iterator(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_iterator(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_match(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_match(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_replace(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_replace(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_search(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_search(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_split(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_split(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_to_primitive(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_to_primitive(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_to_string_tag(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_to_string_tag(&mut local_symbol, isolate);
        }

        local_symbol
    }

    #[inline(always)]
    pub fn get_unscopables(isolate: &mut Isolate) -> Local<Self> {
        let mut local_symbol = Local::<Self>::empty();

        unsafe {
            v8cxx__symbol_get_unscopables(&mut local_symbol, isolate);
        }

        local_symbol
    }
}

impl Data for Symbol {}
impl Value for Symbol {}
impl Primitive for Symbol {}
impl Name for Symbol {}
