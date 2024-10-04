use std::mem::MaybeUninit;

pub trait ClosureToFunction<T>
where
    Self: Sized,
{
    fn to_function(self) -> T;
    fn get() -> Self {
        unsafe { MaybeUninit::<Self>::uninit().assume_init() }
    }
}
