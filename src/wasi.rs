use super::{Send, Sync};

pub use ffi::c_void;

pub type c_char = i8;
pub type c_uchar = u8;
pub type c_schar = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type off_t = i64;
pub type pid_t = i32;
pub type clock_t = c_longlong;
pub type time_t = c_longlong;
pub type c_double = f64;
pub type c_float = f32;
pub type ino_t = u64;
pub type sigset_t = c_uchar;
pub type suseconds_t = c_longlong;
pub type mode_t = u32;
pub type dev_t = u64;
pub type uid_t = u32;
pub type gid_t = u32;
pub type nlink_t = u64;
pub type blksize_t = c_long;
pub type blkcnt_t = i64;
pub type nfds_t = c_ulong;
pub type wchar_t = i32;
pub type nl_item = c_int;
pub type __wasi_rights_t = u64;

s_no_extra_traits! {
    #[repr(align(16))]
    #[allow(missing_debug_implementations)]
    pub struct max_align_t {
        priv_: [f64; 4]
    }
}

#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum FILE {}
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum DIR {}
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum __locale_struct {}

pub type locale_t = *mut __locale_struct;

s_paren! {
    // in wasi-libc clockid_t is const struct __clockid* (where __clockid is an opaque struct),
    // but that's an implementation detail that we don't want to have to deal with
    #[repr(transparent)]
    pub struct clockid_t(*const u8);
}

unsafe impl Send for clockid_t {}
unsafe impl Sync for clockid_t {}

s! {
    #[repr(align(8))]
    pub struct fpos_t {
        data: [u8; 16],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub __tm_gmtoff: c_int,
        pub __tm_zone: *const c_char,
        pub __tm_nsec: c_int,
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct tms {
        pub tms_utime: clock_t,
        pub tms_stime: clock_t,
        pub tms_cutime: clock_t,
        pub tms_cstime: clock_t,
    }

    pub struct itimerspec {
        pub it_interval: timespec,
        pub it_value: timespec,
    }

    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: size_t,
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    pub struct pollfd {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
    }

    pub struct rusage {
        pub ru_utime: timeval,
        pub ru_stime: timeval,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_nlink: nlink_t,
        pub st_mode: mode_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        __pad0: c_uint,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        __reserved: [c_longlong; 3],
    }
}

// Declare dirent outside of s! so that it doesn't implement Copy, Eq, Hash,
// etc., since it contains a flexible array member with a dynamic size.
#[repr(C)]
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_type: c_uchar,
    /// d_name is declared in WASI libc as a flexible array member, which
    /// can't be directly expressed in Rust. As an imperfect workaround,
    /// declare it as a zero-length array instead.
    pub d_name: [c_char; 0],
}

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IONBF: c_int = 2;
pub const _IOLBF: c_int = 1;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const FD_CLOEXEC: c_int = 1;
pub const FD_SETSIZE: size_t = 1024;
pub const O_APPEND: c_int = 0x0001;
pub const O_DSYNC: c_int = 0x0002;
pub const O_NONBLOCK: c_int = 0x0004;
pub const O_RSYNC: c_int = 0x0008;
pub const O_SYNC: c_int = 0x0010;
pub const O_CREAT: c_int = 0x0001 << 12;
pub const O_DIRECTORY: c_int = 0x0002 << 12;
pub const O_EXCL: c_int = 0x0004 << 12;
pub const O_TRUNC: c_int = 0x0008 << 12;
pub const O_NOFOLLOW: c_int = 0x01000000;
pub const O_EXEC: c_int = 0x02000000;
pub const O_RDONLY: c_int = 0x04000000;
pub const O_SEARCH: c_int = 0x08000000;
pub const O_WRONLY: c_int = 0x10000000;
pub const O_CLOEXEC: c_int = 0x0;
pub const O_RDWR: c_int = O_WRONLY | O_RDONLY;
pub const O_ACCMODE: c_int = O_EXEC | O_RDWR | O_SEARCH;
pub const O_NOCTTY: c_int = 0x0;
pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;
pub const POSIX_FADV_NORMAL: c_int = 0;
pub const POSIX_FADV_RANDOM: c_int = 2;
pub const POSIX_FADV_SEQUENTIAL: c_int = 1;
pub const POSIX_FADV_WILLNEED: c_int = 3;
pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACCESS: c_int = 0x0;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x1;
pub const AT_SYMLINK_FOLLOW: c_int = 0x2;
pub const AT_REMOVEDIR: c_int = 0x4;
pub const UTIME_OMIT: c_long = 0xfffffffe;
pub const UTIME_NOW: c_long = 0xffffffff;
pub const S_IFIFO: mode_t = 49152;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 57344;
pub const S_IRWXO: mode_t = 0x7;
pub const S_IXOTH: mode_t = 0x1;
pub const S_IWOTH: mode_t = 0x2;
pub const S_IROTH: mode_t = 0x4;
pub const S_IRWXG: mode_t = 0x38;
pub const S_IXGRP: mode_t = 0x8;
pub const S_IWGRP: mode_t = 0x10;
pub const S_IRGRP: mode_t = 0x20;
pub const S_IRWXU: mode_t = 0x1c0;
pub const S_IXUSR: mode_t = 0x40;
pub const S_IWUSR: mode_t = 0x80;
pub const S_IRUSR: mode_t = 0x100;
pub const S_ISVTX: mode_t = 0x200;
pub const S_ISGID: mode_t = 0x400;
pub const S_ISUID: mode_t = 0x800;
pub const DT_UNKNOWN: u8 = 0;
pub const DT_BLK: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 3;
pub const DT_REG: u8 = 4;
pub const DT_LNK: u8 = 7;
pub const FIONREAD: c_int = 1;
pub const FIONBIO: c_int = 2;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const POLLIN: ::c_short = 0x1;
pub const POLLOUT: ::c_short = 0x2;
pub const POLLERR: ::c_short = 0x1000;
pub const POLLHUP: ::c_short = 0x2000;
pub const POLLNVAL: ::c_short = 0x4000;
pub const POLLRDNORM: ::c_short = 0x1;
pub const POLLWRNORM: ::c_short = 0x2;

pub const E2BIG: c_int = 1;
pub const EACCES: c_int = 2;
pub const EADDRINUSE: c_int = 3;
pub const EADDRNOTAVAIL: c_int = 4;
pub const EAFNOSUPPORT: c_int = 5;
pub const EAGAIN: c_int = 6;
pub const EALREADY: c_int = 7;
pub const EBADF: c_int = 8;
pub const EBADMSG: c_int = 9;
pub const EBUSY: c_int = 10;
pub const ECANCELED: c_int = 11;
pub const ECHILD: c_int = 12;
pub const ECONNABORTED: c_int = 13;
pub const ECONNREFUSED: c_int = 14;
pub const ECONNRESET: c_int = 15;
pub const EDEADLK: c_int = 16;
pub const EDESTADDRREQ: c_int = 17;
pub const EDOM: c_int = 18;
pub const EDQUOT: c_int = 19;
pub const EEXIST: c_int = 20;
pub const EFAULT: c_int = 21;
pub const EFBIG: c_int = 22;
pub const EHOSTUNREACH: c_int = 23;
pub const EIDRM: c_int = 24;
pub const EILSEQ: c_int = 25;
pub const EINPROGRESS: c_int = 26;
pub const EINTR: c_int = 27;
pub const EINVAL: c_int = 28;
pub const EIO: c_int = 29;
pub const EISCONN: c_int = 30;
pub const EISDIR: c_int = 31;
pub const ELOOP: c_int = 32;
pub const EMFILE: c_int = 33;
pub const EMLINK: c_int = 34;
pub const EMSGSIZE: c_int = 35;
pub const EMULTIHOP: c_int = 36;
pub const ENAMETOOLONG: c_int = 37;
pub const ENETDOWN: c_int = 38;
pub const ENETRESET: c_int = 39;
pub const ENETUNREACH: c_int = 40;
pub const ENFILE: c_int = 41;
pub const ENOBUFS: c_int = 42;
pub const ENODEV: c_int = 43;
pub const ENOENT: c_int = 44;
pub const ENOEXEC: c_int = 45;
pub const ENOLCK: c_int = 46;
pub const ENOLINK: c_int = 47;
pub const ENOMEM: c_int = 48;
pub const ENOMSG: c_int = 49;
pub const ENOPROTOOPT: c_int = 50;
pub const ENOSPC: c_int = 51;
pub const ENOSYS: c_int = 52;
pub const ENOTCONN: c_int = 53;
pub const ENOTDIR: c_int = 54;
pub const ENOTEMPTY: c_int = 55;
pub const ENOTRECOVERABLE: c_int = 56;
pub const ENOTSOCK: c_int = 57;
pub const ENOTSUP: c_int = 58;
pub const ENOTTY: c_int = 59;
pub const ENXIO: c_int = 60;
pub const EOVERFLOW: c_int = 61;
pub const EOWNERDEAD: c_int = 62;
pub const EPERM: c_int = 63;
pub const EPIPE: c_int = 64;
pub const EPROTO: c_int = 65;
pub const EPROTONOSUPPORT: c_int = 66;
pub const EPROTOTYPE: c_int = 67;
pub const ERANGE: c_int = 68;
pub const EROFS: c_int = 69;
pub const ESPIPE: c_int = 70;
pub const ESRCH: c_int = 71;
pub const ESTALE: c_int = 72;
pub const ETIMEDOUT: c_int = 73;
pub const ETXTBSY: c_int = 74;
pub const EXDEV: c_int = 75;
pub const ENOTCAPABLE: c_int = 76;
pub const EOPNOTSUPP: c_int = ENOTSUP;
pub const EWOULDBLOCK: c_int = EAGAIN;

pub const _SC_PAGESIZE: c_int = 30;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_IOV_MAX: c_int = 60;
pub const _SC_SYMLOOP_MAX: c_int = 173;

pub static CLOCK_MONOTONIC: clockid_t = unsafe { clockid_t(ptr_addr_of!(_CLOCK_MONOTONIC)) };
pub static CLOCK_PROCESS_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_PROCESS_CPUTIME_ID)) };
pub static CLOCK_REALTIME: clockid_t = unsafe { clockid_t(ptr_addr_of!(_CLOCK_REALTIME)) };
pub static CLOCK_THREAD_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_THREAD_CPUTIME_ID)) };

pub const ABDAY_1: ::nl_item = 0x20000;
pub const ABDAY_2: ::nl_item = 0x20001;
pub const ABDAY_3: ::nl_item = 0x20002;
pub const ABDAY_4: ::nl_item = 0x20003;
pub const ABDAY_5: ::nl_item = 0x20004;
pub const ABDAY_6: ::nl_item = 0x20005;
pub const ABDAY_7: ::nl_item = 0x20006;

pub const DAY_1: ::nl_item = 0x20007;
pub const DAY_2: ::nl_item = 0x20008;
pub const DAY_3: ::nl_item = 0x20009;
pub const DAY_4: ::nl_item = 0x2000A;
pub const DAY_5: ::nl_item = 0x2000B;
pub const DAY_6: ::nl_item = 0x2000C;
pub const DAY_7: ::nl_item = 0x2000D;

pub const ABMON_1: ::nl_item = 0x2000E;
pub const ABMON_2: ::nl_item = 0x2000F;
pub const ABMON_3: ::nl_item = 0x20010;
pub const ABMON_4: ::nl_item = 0x20011;
pub const ABMON_5: ::nl_item = 0x20012;
pub const ABMON_6: ::nl_item = 0x20013;
pub const ABMON_7: ::nl_item = 0x20014;
pub const ABMON_8: ::nl_item = 0x20015;
pub const ABMON_9: ::nl_item = 0x20016;
pub const ABMON_10: ::nl_item = 0x20017;
pub const ABMON_11: ::nl_item = 0x20018;
pub const ABMON_12: ::nl_item = 0x20019;

pub const MON_1: ::nl_item = 0x2001A;
pub const MON_2: ::nl_item = 0x2001B;
pub const MON_3: ::nl_item = 0x2001C;
pub const MON_4: ::nl_item = 0x2001D;
pub const MON_5: ::nl_item = 0x2001E;
pub const MON_6: ::nl_item = 0x2001F;
pub const MON_7: ::nl_item = 0x20020;
pub const MON_8: ::nl_item = 0x20021;
pub const MON_9: ::nl_item = 0x20022;
pub const MON_10: ::nl_item = 0x20023;
pub const MON_11: ::nl_item = 0x20024;
pub const MON_12: ::nl_item = 0x20025;

pub const AM_STR: ::nl_item = 0x20026;
pub const PM_STR: ::nl_item = 0x20027;

pub const D_T_FMT: ::nl_item = 0x20028;
pub const D_FMT: ::nl_item = 0x20029;
pub const T_FMT: ::nl_item = 0x2002A;
pub const T_FMT_AMPM: ::nl_item = 0x2002B;

pub const ERA: ::nl_item = 0x2002C;
pub const ERA_D_FMT: ::nl_item = 0x2002E;
pub const ALT_DIGITS: ::nl_item = 0x2002F;
pub const ERA_D_T_FMT: ::nl_item = 0x20030;
pub const ERA_T_FMT: ::nl_item = 0x20031;

pub const CODESET: ::nl_item = 14;
pub const CRNCYSTR: ::nl_item = 0x4000F;
pub const RADIXCHAR: ::nl_item = 0x10000;
pub const THOUSEP: ::nl_item = 0x10001;
pub const YESEXPR: ::nl_item = 0x50000;
pub const NOEXPR: ::nl_item = 0x50001;
pub const YESSTR: ::nl_item = 0x50002;
pub const NOSTR: ::nl_item = 0x50003;

#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(
        name = "c",
        kind = "static",
        modifiers = "-bundle",
        cfg(target_feature = "crt-static")
    )
)]
#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(name = "c", cfg(not(target_feature = "crt-static")))
)]
extern "C" {
    static _CLOCK_MONOTONIC: u8;
    static _CLOCK_PROCESS_CPUTIME_ID: u8;
    static _CLOCK_REALTIME: u8;
    static _CLOCK_THREAD_CPUTIME_ID: u8;
}
