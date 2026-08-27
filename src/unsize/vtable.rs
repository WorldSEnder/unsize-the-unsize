use super::pointee::*;

mod polyfill_layout_for_ptr {
    // Not yet stable. TODO: use the following on nightly/miri:
    // use core::mem::{align_of_val_raw, size_of_val_raw};
    pub const unsafe fn size_of_val_raw<T: ?Sized>(val: *const T) -> usize {
        size_of_val(unsafe { &*val })
    }
    pub const unsafe fn align_of_val_raw<T: ?Sized>(val: *const T) -> usize {
        align_of_val(unsafe { &*val })
    }
}
use polyfill_layout_for_ptr::{align_of_val_raw, size_of_val_raw};

pub trait VTableRep<Dyn: ?Sized>: UnsizeRep<Dyn, Rep = VTable<Dyn, Self>> {
    type VRep: Sized;
}
pub trait DynPointeeRep<'a, Dyn: 'a + ?Sized>: VTableRep<Dyn> + PointeeRep<'a, Dyn> {
    fn vrep_for_val(this: *const Self) -> Self::VRep;
    /// Drop the value behind the pointer.
    /// This does not receive metadata information at the moment
    fn drop_in_place(rep: *mut ());
}

/// This layout is incredibly unstable, and possibly incorrect.
/// SAFETY: compare to the internal representation in the rust compiler.
#[repr(C)]
pub struct VTable<Dyn: ?Sized, This: ?Sized + VTableRep<Dyn>> {
    drop: fn(*mut ()),
    size: usize,
    align: usize,
    tail: This::VRep,
}

impl<'a, Dyn: 'a + ?Sized, This: ?Sized + DynPointeeRep<'a, Dyn>> VTable<Dyn, This> {
    /// # Safety
    /// The metadata of the pointer must be valid to construct a layout for `This`.
    /// In particular, we must be able to call `size_of_val_raw(this)` and `align_of_val_raw(this)`.
    pub unsafe fn for_val(this: *const This) -> Self {
        Self {
            drop: This::drop_in_place,
            size: unsafe { size_of_val_raw(this) },
            align: unsafe { align_of_val_raw(this) },
            tail: This::vrep_for_val(this),
        }
    }
}
