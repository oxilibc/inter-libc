pub const L_tmpnam: ::c_uint = 14;
pub const TMP_MAX: ::c_uint = 0x7fff;

// stdio file descriptor numbers
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

cfg_if! {
    if #[cfg(libc_align)] {
        mod align;
        pub use self::align::*;
    }
}
