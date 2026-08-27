//! Consider `Box<dyn Trait + 'a>`.
//! This signals a (hidden) type `T: 'a`. It is represented "as" `(Box<T>, Metadata)`.
//! No lifetime is attached to the metdata content of the fat object.
use core::marker::PhantomData;

use super::{CoerceRep, UnsafeCoerceRep};

/// Wrapper struct, signaling that pointer metadata contained in `T` is only valid for the lifetime `'a`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct LifetimeAware<'a, T: 'a + ?Sized> {
    _phantom: PhantomData<&'a ()>,
    inner: T,
}
impl<'a, T> LifetimeAware<'a, T> {
    pub const fn new(inner: T) -> Self {
        Self {
            _phantom: PhantomData,
            inner,
        }
    }
    /// Forget about the lifetime.
    // TODO: It's not 100% clear if this should be `unsafe`. It depends whether pointer metadata
    // has to be valid. E.g. a dyn-object contains a reference to the vtable. If this reference is
    // allowed to dangle, this method is safe to call. If the reference must never dangle, this
    // method is unsafe.
    pub unsafe fn into_inner(self) -> T {
        self.inner
    }
}

impl<'a, 'b: 'a, T: UnsafeCoerceRep<'a, U>, U> UnsafeCoerceRep<'a, LifetimeAware<'a, U>>
    for LifetimeAware<'b, T>
{
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(&this.inner) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> LifetimeAware<'a, U> {
        LifetimeAware::new(T::assemble(unsafe { this.into_inner() }, rep))
    }
}
unsafe impl<'a, 'b: 'a, T: CoerceRep<'a, U>, U> CoerceRep<'a, LifetimeAware<'a, U>>
    for LifetimeAware<'b, T>
{
}
