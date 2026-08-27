use core::mem::ManuallyDrop;
use core::mem::transmute_copy;

use crate::unsize::pointee::PointeeRep;

#[cfg(feature = "alloc")]
mod r#box;
mod ptr;

unsafe trait ThinRep: Sized {
    type Fat<P: ?Sized>;
    fn from_fat<'a, Dyn: ?Sized, This: ?Sized + PointeeRep<'a, Dyn>>(
        this: Self::Fat<This>,
        meta_rep: &'a This::Rep,
    ) -> (Self, This::Meta);
}
/// This layout is incredibly unstable, and possibly incorrect.
/// SAFETY: compare to the internal representation in the rust compiler.
#[repr(C)]
struct UnstableFatRep<'a, Thin, Dyn: ?Sized, This: ?Sized + PointeeRep<'a, Dyn>> {
    thin: Thin,
    meta: This::Meta,
}
impl<'a, Thin: ThinRep, Dyn: ?Sized, This: ?Sized + PointeeRep<'a, Dyn>>
    UnstableFatRep<'a, Thin, Dyn, This>
{
    fn new(this: Thin::Fat<This>, meta_rep: &'a This::Rep) -> Self {
        let (thin, meta) = Thin::from_fat(this, meta_rep);
        Self { thin, meta }
    }
    fn coerce(self) -> Thin::Fat<Dyn> {
        let this = ManuallyDrop::new(self);
        // SAFETY: from safety requirement on ThinRep trait
        unsafe { transmute_copy(&this) }
    }
}

/// Equivalent to `CoerceUnsized`
pub trait UnsafeCoerceRep<'a, U: ?Sized> {
    type Rep;
    unsafe fn for_val(this: &Self) -> Self::Rep;
    fn assemble(this: Self, rep: &'a mut Self::Rep) -> U;
}
pub unsafe trait CoerceRep<'a, U: ?Sized>: UnsafeCoerceRep<'a, U> {
    fn safe_for_val(this: &Self) -> Self::Rep {
        unsafe { Self::for_val(this) }
    }
}
//mod lifetime_aware;
//pub use lifetime_aware::LifetimeAware;
