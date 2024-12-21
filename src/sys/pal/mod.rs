cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    }
    else {
        mod unsupported;
        pub use self::unsupported::*;
    }
}

pub type RawOsError = i32;
