//extern crate autocfg;

const PROBE_UNSIZE: &str = r#"
#![no_std]
#![feature(unsize)]
#![feature(ptr_metadata)]
pub trait TestUnsize: core::marker::Unsize<()> {}
pub trait TestPointee: core::ptr::Pointee {}
"#;

fn main() {
    let ac = autocfg::new();
    autocfg::emit_possibility("unsize");

    if ac.probe_raw(PROBE_UNSIZE).is_ok() {
        autocfg::emit("unsize");
    };
    ac.emit_trait_cfg("core::marker::Unsize", "unsize");

    // (optional) We don't need to rerun for anything external.
    autocfg::rerun_path("build.rs");
}
