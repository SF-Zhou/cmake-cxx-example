use crate::*;

#[::cxx::bridge]
mod ffi {
    extern "Rust" {
        type RustType;
        fn inc(&mut self);
        fn show(&self);
        fn create_a_rust_object() -> Box<RustType>;
        fn concat_two_strings(a: &str, b: &str) -> String;
    }
}

fn create_a_rust_object() -> Box<RustType> {
    Default::default()
}
