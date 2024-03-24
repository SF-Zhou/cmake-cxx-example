use super::*;

#[::cxx::bridge]
mod ffi {
    #[namespace = "hello"]
    extern "Rust" {
        fn add(left: usize, right: usize) -> usize;
    }
}
