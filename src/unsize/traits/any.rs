use core::any::Any;
use core::any::TypeId;
use core::marker::PhantomData;

use crate::unsize::pointee::PointeeRep;
use crate::unsize::pointee::UnsizeRep;
use crate::unsize::vtable::DynPointeeRep;
use crate::unsize::vtable::VTableRep;

#[repr(C)]
pub struct AnyRep<This: ?Sized> {
    type_id: fn(*const PhantomData<This>) -> TypeId,
}
// U: 'static
#[cfg(not(unsize))]
mod sized {
    // TODO: we might as well not bother constructing our own vtable
    // and instead transmute the fat pointer directly (which will roundtrip).
    // Then type Rep = ();
    use super::*;
    impl<U: 'static> UnsizeRep<dyn Any> for U {
        type Rep = crate::unsize::vtable::VTable<dyn Any, Self>;
    }
    impl<'a, U: 'static> PointeeRep<'a, dyn Any> for U {
        type Meta = &'a crate::unsize::vtable::VTable<dyn Any, Self>;

        unsafe fn for_val(this: *const Self) -> Self::Rep {
            unsafe { crate::unsize::vtable::VTable::for_val(this) }
        }
        fn unsize_metadata(_this: *const Self, meta: &'a Self::Rep) -> Self::Meta {
            meta
        }
    }
    impl<U: 'static> VTableRep<dyn Any> for U {
        type VRep = AnyRep<U>;
    }
    impl<'a, U: 'static> DynPointeeRep<'a, dyn Any> for U {
        fn vrep_for_val(_this: *const Self) -> Self::VRep {
            AnyRep {
                type_id: |_| TypeId::of::<Self>(),
            }
        }

        fn drop_in_place(rep: *mut ()) {
            unsafe { core::ptr::drop_in_place(rep as *mut U) }
        }
    }
}
// str
impl UnsizeRep<dyn Any> for str {
    type Rep = crate::unsize::vtable::VTable<dyn Any, Self>;
}
impl<'a> PointeeRep<'a, dyn Any> for str {
    type Meta = &'a crate::unsize::vtable::VTable<dyn Any, Self>;

    unsafe fn for_val(this: *const Self) -> Self::Rep {
        unsafe { crate::unsize::vtable::VTable::for_val(this) }
    }
    fn unsize_metadata(_this: *const Self, meta: &'a Self::Rep) -> Self::Meta {
        meta
    }
}
impl VTableRep<dyn Any> for str {
    type VRep = AnyRep<str>;
}
impl<'a> DynPointeeRep<'a, dyn Any> for str {
    fn vrep_for_val(_this: *const Self) -> Self::VRep {
        AnyRep {
            type_id: |_| TypeId::of::<str>(),
        }
    }

    fn drop_in_place(_rep: *mut ()) {
        // str needs no drop
    }
}
// [T]
impl<T> UnsizeRep<dyn Any> for [T] {
    type Rep = crate::unsize::vtable::VTable<dyn Any, Self>;
}
impl<T> VTableRep<dyn Any> for [T] {
    type VRep = AnyRep<[T]>;
}
impl<'a, T: 'static + Copy> PointeeRep<'a, dyn Any> for [T] {
    type Meta = &'a crate::unsize::vtable::VTable<dyn Any, Self>;

    unsafe fn for_val(this: *const Self) -> Self::Rep {
        unsafe { crate::unsize::vtable::VTable::for_val(this) }
    }
    fn unsize_metadata(_this: *const Self, meta: &'a Self::Rep) -> Self::Meta {
        meta
    }
}
impl<'a, T: 'static + Copy> DynPointeeRep<'a, dyn Any> for [T] {
    fn vrep_for_val(_this: *const Self) -> Self::VRep {
        AnyRep {
            type_id: |_| TypeId::of::<[T]>(),
        }
    }

    fn drop_in_place(_rep: *mut ()) {
        // Copy types need no drop
    }
}
