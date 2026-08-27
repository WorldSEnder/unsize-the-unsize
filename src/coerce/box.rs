use crate::unsize::pointee::PointeeRep;

use super::UnsafeCoerceRep;
use alloc::boxed::Box;

impl<'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, Box<U>> for Box<T> {
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(this.as_ref()) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> Box<U> {
        let this = Box::into_raw(this);
        let this = UnsafeCoerceRep::<'_, *mut U>::assemble(this, rep);
        unsafe { Box::from_raw(this) }
    }
}
