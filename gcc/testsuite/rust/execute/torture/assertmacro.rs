#![feature(rustc_attrs)]
#![feature(intrinsics)]

extern "rust-intrinsic" {
    pub fn abort();
}

#[rustc_builtin_macro]
macro_rules! assert {
    ($cond:expr $(,)?) => {{ /* built-in */ }};
    ($cond:expr, $($arg:tt)+) => {{ /* built-in */ }};
}

fn something(i : u32) -> i32 {
    ((i / 45) >> 2 )as i32
}


fn main () -> i32 {
    let b = something(6*45);
   //unsafe {abort();};
   assert! (b != 3);
   0
}
