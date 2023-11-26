mod layer;
mod storage;

pub use layer::*;

#[macro_export]
macro_rules! log {
    ($f: literal) => {{
        if cfg!(debug_assertions) {
            print!("[{}] ", env!("CARGO_PKG_NAME"));
            println!($f);
        }
    }};
    ($f: literal, $($arg:tt)*) => {{
        if cfg!(debug_assertions) {
            print!("[{}] ", env!("CARGO_PKG_NAME"));
            println!($f, ($($arg)*));
        }
    }};
}
