use core::mem::MaybeUninit;

use crate::coerce::{CoerceRep, UnsafeCoerceRep};

pub struct Host<Rep> {
    meta_rep: MaybeUninit<Rep>,
}
impl<Rep> Host<Rep> {
    pub const fn new() -> Self {
        Self {
            meta_rep: MaybeUninit::uninit(),
        }
    }
    pub unsafe fn unsafe_coerce<'a, This, Dyn>(&'a mut self, this: This) -> Dyn
    where
        This: UnsafeCoerceRep<'a, Dyn, Rep = Rep>,
        This:, // Sized,
        Dyn:,  // Sized,
    {
        let meta_rep = self.meta_rep.write(unsafe { This::for_val(&this) });
        This::assemble(this, meta_rep)
    }
    /// # Safety
    /// Not really.
    pub fn coerce<'a, This, Dyn>(&'a mut self, this: This) -> Dyn
    where
        This: CoerceRep<'a, Dyn, Rep = Rep>,
        This:, // Sized,
        Dyn:,  // Sized,
    {
        let meta_rep = self.meta_rep.write(This::safe_for_val(&this));
        This::assemble(this, meta_rep)
    }
}
