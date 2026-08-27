use crate::host::Host;
use core::any::{Any, TypeId};

#[test]
fn coerce_static_any() {
    struct Simple;
    let mut host = Host::new();
    let as_any: &dyn Any = host.coerce(&Simple);
    assert_eq!(as_any.type_id(), TypeId::of::<Simple>());
}

#[test]
fn coerce_str_any() {
    let mut host = Host::new();
    let as_any: &dyn Any = host.coerce("foobar");
    assert_eq!(as_any.type_id(), TypeId::of::<str>());
}

#[test]
fn coerce_str_any_raw() {
    let mut host = Host::new();
    let as_any: *const dyn Any = unsafe { host.unsafe_coerce("foobar" as *const str) };
    assert_eq!(unsafe { &*as_any }.type_id(), TypeId::of::<str>());
}

#[cfg(feature = "alloc")]
#[test]
fn coerce_str_any_box() {
    use alloc::boxed::Box;
    let mut host = Host::new();
    let src: Box<str> = Box::from("foobar");
    let as_any: Box<dyn Any> = unsafe { host.unsafe_coerce(src) };
    assert_eq!((*as_any).type_id(), TypeId::of::<str>());
}

#[test]
fn coerce_slice_any() {
    let mut host = Host::new();
    let slice: &[u8] = &[0, 1, 2];
    let as_any: &dyn Any = host.coerce(slice);
    assert_eq!(as_any.type_id(), TypeId::of::<[u8]>());
}

// #[test]
// fn coerce_slice_any_drop() {
//     struct NeedsDrop;
//     impl Drop for NeedsDrop {
//         fn drop(&mut self) {}
//     }
//     let mut host = Host::new();
//     let slice: &[NeedsDrop] = &[NeedsDrop];
//     let as_any: &dyn Any = host.coerce(slice);
//     // ^~~ we could implement this, but can't run the drop
//     assert_eq!(as_any.type_id(), TypeId::of::<[u8]>());
// }
