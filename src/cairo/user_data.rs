use crate::cairo::ffi;
use std::marker::PhantomData;

pub struct UserDataKey<T> {
    pub(crate) ffi: ffi::cairo_user_data_key_t,
    marker: PhantomData<*const T>,
}

unsafe impl<T> Sync for UserDataKey<T> {}

impl<T> UserDataKey<T> {
    pub const fn new() -> Self {
        Self {
            ffi: ffi::cairo_user_data_key_t { unused: 0 },
            marker: PhantomData,
        }
    }
}

impl<T> Default for UserDataKey<T> {
    fn default() -> Self {
        Self::new()
    }
}
