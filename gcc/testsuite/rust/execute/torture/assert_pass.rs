// This is part of libstd
#![feature(rustc_attrs)]
#![feature(intrinsics)]

#[rustc_builtin_macro]
macro_rules! assert {
    ($cond:expr $(,)?) => {{ /* built-in */ }};
    ($cond:expr, $($arg:tt)+) => {{ /* built-in */ }};
}

mod intrinsics {
    extern "rust-intrinsic" {
        pub fn abort() -> !;
    }
}
//

pub fn jump (i: i32) {
    assert! (i == 33);
}

pub fn main () -> i32 {
    let mut i : i32 = 33;
    jump(i);
    0
}
