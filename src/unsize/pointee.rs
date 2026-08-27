/// Roughly this means that we can emulate the `Self: Unsize<Dyn>` coercion.
pub trait UnsizeRep<Dyn: ?Sized> {
    /// The representation stored on the stack while the coercion is live
    type Rep: Sized;
}
pub trait PointeeRep<'a, Dyn: 'a + ?Sized>: UnsizeRep<Dyn> {
    /// The pointer metadata.
    type Meta: 'a;

    unsafe fn for_val(this: *const Self) -> Self::Rep;
    fn unsize_metadata(this: *const Self, meta: &'a Self::Rep) -> Self::Meta;
}

#[cfg(unsize)]
mod unsize {
    use super::*;
    // Note we restrict this impl specifically to T: Sized
    // Otherwise, core reserves the impl for unsized values in the future, which we
    // do not want
    impl<T, U: ?Sized> UnsizeRep<U> for T
    where
        T: core::marker::Unsize<U>,
    {
        type Rep = ();
    }

    impl<'a, T, U: 'a + ?Sized> PointeeRep<'a, U> for T
    where
        T: core::marker::Unsize<U>,
    {
        type Meta = <U as core::ptr::Pointee>::Metadata;

        unsafe fn for_val(_this: *const Self) -> Self::Rep {}
        fn unsize_metadata(this: *const Self, _meta: &'a Self::Rep) -> Self::Meta {
            core::ptr::metadata(this as *const U)
        }
    }
}
