#![no_std]
#![cfg_attr(unsize, feature(unsize, ptr_metadata))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod unsize {
    pub mod pointee;
    pub mod vtable;

    mod traits {
        mod any;
    }
}

mod coerce;
mod host;

pub use host::Host;

#[cfg(test)]
mod test;

mod impl_docs {
    //! A collection of throughts from implementing:
    //!
    //! ## Pointer coercion
    //!
    //! Some coercions are unsafe.
    //! ```no_run
    //! # fn coerce<T: ?Sized, U: ?Sized>(x: T) -> U { todo!() }
    //! struct WithTail<T: ?Sized> {
    //!     head: u32,
    //!     tail: T,
    //! }
    //! #fn foo() {
    //! let x: *const WithTail<[u32]> = todo!();
    //! let y: *const dyn Any = coerce(x);
    //! //                      ^~~ must use unsafe coercion.
    //! // We can assume that `x` contains valid pointer metadata,
    //! // i.e. a valid slice length but we can not assume that the derived
    //! // vtable for `impl Any for WithTail<[u32]>` is valid (its size
    //! // may overflow).
    //! let x: &WithTail<[u32]> = todo!();
    //! let y: *const dyn Any = coerce(x);
    //! //                      ^~~ this is safe.
    //! // We get an additional guarantee from holding a reference.
    //! }
    //! ```
    //!
    //! ## Lifetime issues
    //!
    //! Lifetime is not captured correctly.
    //! ```no_run
    //! # struct Host;
    //! # impl Host { fn new() -> Self { Self } }
    //! # fn coerce<T: ?Sized, U: ?Sized>(host: &mut Host, x: T) -> U { todo!() }
    //! let mut host = Host::new();
    //! let x: Box<u32> = todo!();
    //! let y: Box<dyn Any> = coerce(&mut host, x);
    //! //                    ^~~ must use unsafe coercion at the moment
    //! ```
    //!
    //! The coerced box must only be used for the lifetime of the `host`, since
    //! metadata is stored there.
    //!
    //! The additional lifetime requirement can not be attached to the `dyn` object.
    //! Indeed `dyn Any + 'a` implicitly requires `'a: 'static` since `trait Any: 'static`.
    //!
    //! This is at the moment captured by the same unsafe coercion as pointer coercion.
    //! A proposal would let us write `dyn<'a> Trait` to capture the lifetime. This aligns
    //! with the currently behaviour and implicit lifetimes, which is `dyn<'static> Trait`.
    //!
    //! ## Drop issues
    //!
    //! Vtable functions, and in particular the drop function, do not receive fat pointers
    //! when called.
    //!
    //! ```no_run
    //! # struct Host;
    //! # impl Host { fn new() -> Self { Self } }
    //! # fn coerce<T: ?Sized, U: ?Sized>(host: &mut Host, x: T) -> U { todo!() }
    //! let x: &[u32] = todo!();
    //! let y: &dyn Debug = coerce(&mut host, x);
    //! //                  ^~~ this is okay
    //! struct NeedsDrop;
    //! impl Drop for NeedsDrop {
    //!     fn drop(&mut self) { println!("Dropped") }
    //! }
    //! let x: &[NeedsDrop] = &[NeedsDrop];
    //! let y: &dyn Debug = coerce(&mut host, x);
    //! //                  ^~~ this would leak at best
    //! ```
    //!
    //! This makes it impossible to drop slices correctly when the type there-in needs to
    //! be dropped. Workaround is only implementing this for `Copy` types for now.
    //!
    //! See also `DispatchFromDyn`.
    //!
    //! ```no_run
    //! trait Foo {
    //!     fn call(self: Box<Self>);
    //! }
    //! struct Bar;
    //! impl Foo for Bar {
    //!     fn call(self: Box<Self>) {}
    //! }
    //! #fn foo() {
    //! let _: fn(Box<Foo>) = <Bar as Foo>::call;
    //! //  ^~~ the vtable stores this function pointer?
    //! let _: fn(Box<dyn Foo>) = <dyn Foo as Foo>::call;
    //! //  ^~~ this is implemented by the compiler
    //! #}
    //! ```
    //!
    //! "Unwrapping" from a `Box<dyn Foo>` to a `Box<Foo>` is done by the compiler.
    //! Right now, I believe this is done in a context where the specific receiver type
    //! is not know. This roughly looks like
    //! ```ignore
    //! impl Foo for dyn Foo {
    //!     fn call(self: Box<dyn Self>) {
    //!         // Box<dyn Self>: DispatchFromDyn<_> is the magic that guarantees that we
    //!         // can do the following:
    //!         let (thin, vtable) = DispatchFromDyn::to_raw_parts(self);
    //!         type ThinBox = <Box<dyn Self> as DispatchFromDyn<_>>::Thin;
    //!         let call: fn(ThinBox) = self.vtable.call;
    //!         call(thin)
    //!     }
    //! }
    //! ```
    //!
    //! Why not move this wrapper *into* the vtable function and forward the fat pointer
    //! directly?
    //!
}
