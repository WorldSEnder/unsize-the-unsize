use crate::unsize::pointee::PointeeRep;

use super::{CoerceRep, ThinRep, UnsafeCoerceRep, UnstableFatRep};

type Thin = *const ();
unsafe impl ThinRep for Thin {
    type Fat<P: ?Sized> = *const P;

    fn from_fat<'a, Dyn: ?Sized, This: ?Sized + PointeeRep<'a, Dyn>>(
        this: Self::Fat<This>,
        meta_rep: &'a This::Rep,
    ) -> (Self, This::Meta) {
        (this as *const (), This::unsize_metadata(this, meta_rep))
    }
}

pub type UnstablePtrRep<'a, Dyn, This> = UnstableFatRep<'a, Thin, Dyn, This>;

// ref -> ref coercions

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> UnsafeCoerceRep<'a, &'a U> for &'b T {
    type Rep = T::Rep;

    unsafe fn for_val(&this: &Self) -> Self::Rep {
        unsafe { T::for_val(this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> &'a U {
        unsafe { &*UnstablePtrRep::new(this, rep).coerce() }
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> CoerceRep<'a, &'a U> for &'b T {}

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> UnsafeCoerceRep<'a, &'a U>
    for &'b mut T
{
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(*&*this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> &'a U {
        unsafe { &*UnstablePtrRep::new(this, rep).coerce() }
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> CoerceRep<'a, &'a U>
    for &'b mut T
{
}

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> UnsafeCoerceRep<'a, &'a mut U>
    for &'b mut T
{
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(*this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> &'a mut U {
        unsafe { &mut *UnstablePtrRep::new(this, rep).coerce().cast_mut() }
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: ?Sized> CoerceRep<'a, &'a mut U>
    for &'b mut T
{
}

// ref -> pointer coercions

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *const U>
    for &'b T
{
    type Rep = T::Rep;

    unsafe fn for_val(&this: &Self) -> Self::Rep {
        unsafe { T::for_val(this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *const U {
        UnstablePtrRep::new(this, rep).coerce()
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> CoerceRep<'a, *const U>
    for &'b T
{
}

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *const U>
    for &'b mut T
{
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(*&*this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *const U {
        UnstablePtrRep::new(this, rep).coerce()
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> CoerceRep<'a, *const U>
    for &'b mut T
{
}

impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *mut U>
    for &'b mut T
{
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(*this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *mut U {
        UnstablePtrRep::new(this, rep).coerce().cast_mut()
    }
}
unsafe impl<'a, 'b: 'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> CoerceRep<'a, *mut U>
    for &'b mut T
{
}

// pointer -> pointer coercions
// these are unsafe: the metadata has a validity requirement, but this is not sufficient.

impl<'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *const U> for *const T {
    type Rep = T::Rep;

    unsafe fn for_val(&this: &Self) -> Self::Rep {
        unsafe { T::for_val(this) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *const U {
        UnstablePtrRep::new(this, rep).coerce()
    }
}

impl<'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *const U> for *mut T {
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(this.cast_const()) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *const U {
        UnstablePtrRep::new(this, rep).coerce()
    }
}

impl<'a, T: ?Sized + PointeeRep<'a, U>, U: 'a + ?Sized> UnsafeCoerceRep<'a, *mut U> for *mut T {
    type Rep = T::Rep;

    unsafe fn for_val(this: &Self) -> Self::Rep {
        unsafe { T::for_val(this.cast_const()) }
    }

    fn assemble(this: Self, rep: &'a mut Self::Rep) -> *mut U {
        UnstablePtrRep::new(this, rep).coerce().cast_mut()
    }
}
