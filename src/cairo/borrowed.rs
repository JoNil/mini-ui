use std::mem;

/// Wrapper around values representing borrowed C memory.
///
/// This is returned by `from_glib_borrow()` and ensures that the wrapped value
/// is never dropped when going out of scope.
///
/// Borrowed values must never be passed by value or mutable reference to safe Rust code and must
/// not leave the C scope in which they are valid.
#[derive(Debug)]
pub struct Borrowed<T>(mem::ManuallyDrop<T>);

impl<T> Borrowed<T> {
    /// Creates a new borrowed value.
    #[inline]
    pub fn new(val: T) -> Self {
        Self(mem::ManuallyDrop::new(val))
    }

    /// Extracts the contained value.
    ///
    /// The returned value must never be dropped and instead has to be passed to `mem::forget()` or
    /// be directly wrapped in `mem::ManuallyDrop` or another `Borrowed` wrapper.
    #[inline]
    pub unsafe fn into_inner(self) -> T {
        mem::ManuallyDrop::into_inner(self.0)
    }
}

impl<T> AsRef<T> for Borrowed<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::Deref for Borrowed<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}
