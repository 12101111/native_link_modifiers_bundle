#![feature(native_link_modifiers)]
#![feature(native_link_modifiers_bundle)]
#![feature(static_nobundle)]

#[link(name = "foo", kind = "static", modifiers = "-bundle")]
extern "C" {
    fn bar() -> i32;
}

pub fn rust_bar() -> i32 {
    unsafe { bar() }
}
