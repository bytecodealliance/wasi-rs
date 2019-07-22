//! This file defines slightly more idiomatic interfaces to WASI functions.
//!
//! Pointer-length pairs are replaced by slice types, output parameters are
//! converted to normal return values, and safe interfaces are provided.
//!
//! TODO: Not all functions are covered yet; implement the rest of the API.

#![allow(non_camel_case_types)]

pub mod raw;

use core::ffi::c_void;
use core::mem::MaybeUninit;
use raw::*;

pub type advice_t = __wasi_advice_t;
pub type clockid_t = __wasi_clockid_t;
pub type device_t = __wasi_device_t;
pub type dircookie_t = __wasi_dircookie_t;
pub type errno_t = __wasi_errno_t;
pub type eventrwflags_t = __wasi_eventrwflags_t;
pub type eventtype_t = __wasi_eventtype_t;
pub type exitcode_t = __wasi_exitcode_t;
pub type fd_t = __wasi_fd_t;
pub type fdflags_t = __wasi_fdflags_t;
pub type filedelta_t = __wasi_filedelta_t;
pub type filesize_t = __wasi_filesize_t;
pub type filetype_t = __wasi_filetype_t;
pub type fstflags_t = __wasi_fstflags_t;
pub type inode_t = __wasi_inode_t;
pub type linkcount_t = __wasi_linkcount_t;
pub type lookupflags_t = __wasi_lookupflags_t;
pub type oflags_t = __wasi_oflags_t;
pub type preopentype_t = __wasi_preopentype_t;
pub type riflags_t = __wasi_riflags_t;
pub type rights_t = __wasi_rights_t;
pub type roflags_t = __wasi_roflags_t;
pub type sdflags_t = __wasi_sdflags_t;
pub type siflags_t = __wasi_siflags_t;
pub type signal_t = __wasi_signal_t;
pub type subclockflags_t = __wasi_subclockflags_t;
pub type timestamp_t = __wasi_timestamp_t;
pub type userdata_t = __wasi_userdata_t;
pub type whence_t = __wasi_whence_t;

pub type dirent_t = __wasi_dirent_t;
pub type event_u_fd_readwrite_t = __wasi_event_u_fd_readwrite_t;
pub type fdstat_t = __wasi_fdstat_t;
pub type filestat_t = __wasi_filestat_t;
pub type ciovec_t = __wasi_ciovec_t;
pub type iovec_t = __wasi_iovec_t;
pub type subscription_u_clock_t = __wasi_subscription_u_clock_t;
pub type subscription_u_fd_readwrite_t = __wasi_subscription_u_fd_readwrite_t;
pub type prestat_u_dir_t = __wasi_prestat_u_dir_t;
pub type subscription_t = __wasi_subscription_t;
pub type event_t = __wasi_event_t;
pub type event_u = __wasi_event_u;
pub type subscription_u = __wasi_subscription_u;
pub type prestat_t = __wasi_prestat_t;
pub type prestat_u = __wasi_prestat_u;

pub const ADVICE_NORMAL: u8 = __WASI_ADVICE_NORMAL;
pub const ADVICE_SEQUENTIAL: u8 = __WASI_ADVICE_SEQUENTIAL;
pub const ADVICE_RANDOM: u8 = __WASI_ADVICE_RANDOM;
pub const ADVICE_WILLNEED: u8 = __WASI_ADVICE_WILLNEED;
pub const ADVICE_DONTNEED: u8 = __WASI_ADVICE_DONTNEED;
pub const ADVICE_NOREUSE: u8 = __WASI_ADVICE_NOREUSE;
pub const CLOCK_REALTIME: u32 = __WASI_CLOCK_REALTIME;
pub const CLOCK_MONOTONIC: u32 = __WASI_CLOCK_MONOTONIC;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = __WASI_CLOCK_PROCESS_CPUTIME_ID;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = __WASI_CLOCK_THREAD_CPUTIME_ID;
pub const DIRCOOKIE_START: u64 = __WASI_DIRCOOKIE_START;
pub const ESUCCESS: u16 = __WASI_ESUCCESS;
pub const E2BIG: u16 = __WASI_E2BIG;
pub const EACCES: u16 = __WASI_EACCES;
pub const EADDRINUSE: u16 = __WASI_EADDRINUSE;
pub const EADDRNOTAVAIL: u16 = __WASI_EADDRNOTAVAIL;
pub const EAFNOSUPPORT: u16 = __WASI_EAFNOSUPPORT;
pub const EAGAIN: u16 = __WASI_EAGAIN;
pub const EALREADY: u16 = __WASI_EALREADY;
pub const EBADF: u16 = __WASI_EBADF;
pub const EBADMSG: u16 = __WASI_EBADMSG;
pub const EBUSY: u16 = __WASI_EBUSY;
pub const ECANCELED: u16 = __WASI_ECANCELED;
pub const ECHILD: u16 = __WASI_ECHILD;
pub const ECONNABORTED: u16 = __WASI_ECONNABORTED;
pub const ECONNREFUSED: u16 = __WASI_ECONNREFUSED;
pub const ECONNRESET: u16 = __WASI_ECONNRESET;
pub const EDEADLK: u16 = __WASI_EDEADLK;
pub const EDESTADDRREQ: u16 = __WASI_EDESTADDRREQ;
pub const EDOM: u16 = __WASI_EDOM;
pub const EDQUOT: u16 = __WASI_EDQUOT;
pub const EEXIST: u16 = __WASI_EEXIST;
pub const EFAULT: u16 = __WASI_EFAULT;
pub const EFBIG: u16 = __WASI_EFBIG;
pub const EHOSTUNREACH: u16 = __WASI_EHOSTUNREACH;
pub const EIDRM: u16 = __WASI_EIDRM;
pub const EILSEQ: u16 = __WASI_EILSEQ;
pub const EINPROGRESS: u16 = __WASI_EINPROGRESS;
pub const EINTR: u16 = __WASI_EINTR;
pub const EINVAL: u16 = __WASI_EINVAL;
pub const EIO: u16 = __WASI_EIO;
pub const EISCONN: u16 = __WASI_EISCONN;
pub const EISDIR: u16 = __WASI_EISDIR;
pub const ELOOP: u16 = __WASI_ELOOP;
pub const EMFILE: u16 = __WASI_EMFILE;
pub const EMLINK: u16 = __WASI_EMLINK;
pub const EMSGSIZE: u16 = __WASI_EMSGSIZE;
pub const EMULTIHOP: u16 = __WASI_EMULTIHOP;
pub const ENAMETOOLONG: u16 = __WASI_ENAMETOOLONG;
pub const ENETDOWN: u16 = __WASI_ENETDOWN;
pub const ENETRESET: u16 = __WASI_ENETRESET;
pub const ENETUNREACH: u16 = __WASI_ENETUNREACH;
pub const ENFILE: u16 = __WASI_ENFILE;
pub const ENOBUFS: u16 = __WASI_ENOBUFS;
pub const ENODEV: u16 = __WASI_ENODEV;
pub const ENOENT: u16 = __WASI_ENOENT;
pub const ENOEXEC: u16 = __WASI_ENOEXEC;
pub const ENOLCK: u16 = __WASI_ENOLCK;
pub const ENOLINK: u16 = __WASI_ENOLINK;
pub const ENOMEM: u16 = __WASI_ENOMEM;
pub const ENOMSG: u16 = __WASI_ENOMSG;
pub const ENOPROTOOPT: u16 = __WASI_ENOPROTOOPT;
pub const ENOSPC: u16 = __WASI_ENOSPC;
pub const ENOSYS: u16 = __WASI_ENOSYS;
pub const ENOTCONN: u16 = __WASI_ENOTCONN;
pub const ENOTDIR: u16 = __WASI_ENOTDIR;
pub const ENOTEMPTY: u16 = __WASI_ENOTEMPTY;
pub const ENOTRECOVERABLE: u16 = __WASI_ENOTRECOVERABLE;
pub const ENOTSOCK: u16 = __WASI_ENOTSOCK;
pub const ENOTSUP: u16 = __WASI_ENOTSUP;
pub const ENOTTY: u16 = __WASI_ENOTTY;
pub const ENXIO: u16 = __WASI_ENXIO;
pub const EOVERFLOW: u16 = __WASI_EOVERFLOW;
pub const EOWNERDEAD: u16 = __WASI_EOWNERDEAD;
pub const EPERM: u16 = __WASI_EPERM;
pub const EPIPE: u16 = __WASI_EPIPE;
pub const EPROTO: u16 = __WASI_EPROTO;
pub const EPROTONOSUPPORT: u16 = __WASI_EPROTONOSUPPORT;
pub const EPROTOTYPE: u16 = __WASI_EPROTOTYPE;
pub const ERANGE: u16 = __WASI_ERANGE;
pub const EROFS: u16 = __WASI_EROFS;
pub const ESPIPE: u16 = __WASI_ESPIPE;
pub const ESRCH: u16 = __WASI_ESRCH;
pub const ESTALE: u16 = __WASI_ESTALE;
pub const ETIMEDOUT: u16 = __WASI_ETIMEDOUT;
pub const ETXTBSY: u16 = __WASI_ETXTBSY;
pub const EXDEV: u16 = __WASI_EXDEV;
pub const ENOTCAPABLE: u16 = __WASI_ENOTCAPABLE;
pub const EVENT_FD_READWRITE_HANGUP: u16 = __WASI_EVENT_FD_READWRITE_HANGUP;
pub const EVENTTYPE_CLOCK: u8 = __WASI_EVENTTYPE_CLOCK;
pub const EVENTTYPE_FD_READ: u8 = __WASI_EVENTTYPE_FD_READ;
pub const EVENTTYPE_FD_WRITE: u8 = __WASI_EVENTTYPE_FD_WRITE;
pub const FDFLAG_APPEND: u16 = __WASI_FDFLAG_APPEND;
pub const FDFLAG_DSYNC: u16 = __WASI_FDFLAG_DSYNC;
pub const FDFLAG_NONBLOCK: u16 = __WASI_FDFLAG_NONBLOCK;
pub const FDFLAG_RSYNC: u16 = __WASI_FDFLAG_RSYNC;
pub const FDFLAG_SYNC: u16 = __WASI_FDFLAG_SYNC;
pub const FILETYPE_UNKNOWN: u8 = __WASI_FILETYPE_UNKNOWN;
pub const FILETYPE_BLOCK_DEVICE: u8 = __WASI_FILETYPE_BLOCK_DEVICE;
pub const FILETYPE_CHARACTER_DEVICE: u8 = __WASI_FILETYPE_CHARACTER_DEVICE;
pub const FILETYPE_DIRECTORY: u8 = __WASI_FILETYPE_DIRECTORY;
pub const FILETYPE_REGULAR_FILE: u8 = __WASI_FILETYPE_REGULAR_FILE;
pub const FILETYPE_SOCKET_DGRAM: u8 = __WASI_FILETYPE_SOCKET_DGRAM;
pub const FILETYPE_SOCKET_STREAM: u8 = __WASI_FILETYPE_SOCKET_STREAM;
pub const FILETYPE_SYMBOLIC_LINK: u8 = __WASI_FILETYPE_SYMBOLIC_LINK;
pub const FILESTAT_SET_ATIM: u16 = __WASI_FILESTAT_SET_ATIM;
pub const FILESTAT_SET_ATIM_NOW: u16 = __WASI_FILESTAT_SET_ATIM_NOW;
pub const FILESTAT_SET_MTIM: u16 = __WASI_FILESTAT_SET_MTIM;
pub const FILESTAT_SET_MTIM_NOW: u16 = __WASI_FILESTAT_SET_MTIM_NOW;
pub const LOOKUP_SYMLINK_FOLLOW: u32 = __WASI_LOOKUP_SYMLINK_FOLLOW;
pub const O_CREAT: u16 = __WASI_O_CREAT;
pub const O_DIRECTORY: u16 = __WASI_O_DIRECTORY;
pub const O_EXCL: u16 = __WASI_O_EXCL;
pub const O_TRUNC: u16 = __WASI_O_TRUNC;
pub const PREOPENTYPE_DIR: u8 = __WASI_PREOPENTYPE_DIR;
pub const SOCK_RECV_PEEK: u16 = __WASI_SOCK_RECV_PEEK;
pub const SOCK_RECV_WAITALL: u16 = __WASI_SOCK_RECV_WAITALL;
pub const RIGHT_FD_DATASYNC: u64 = __WASI_RIGHT_FD_DATASYNC;
pub const RIGHT_FD_READ: u64 = __WASI_RIGHT_FD_READ;
pub const RIGHT_FD_SEEK: u64 = __WASI_RIGHT_FD_SEEK;
pub const RIGHT_FD_FDSTAT_SET_FLAGS: u64 = __WASI_RIGHT_FD_FDSTAT_SET_FLAGS;
pub const RIGHT_FD_SYNC: u64 = __WASI_RIGHT_FD_SYNC;
pub const RIGHT_FD_TELL: u64 = __WASI_RIGHT_FD_TELL;
pub const RIGHT_FD_WRITE: u64 = __WASI_RIGHT_FD_WRITE;
pub const RIGHT_FD_ADVISE: u64 = __WASI_RIGHT_FD_ADVISE;
pub const RIGHT_FD_ALLOCATE: u64 = __WASI_RIGHT_FD_ALLOCATE;
pub const RIGHT_PATH_CREATE_DIRECTORY: u64 = __WASI_RIGHT_PATH_CREATE_DIRECTORY;
pub const RIGHT_PATH_CREATE_FILE: u64 = __WASI_RIGHT_PATH_CREATE_FILE;
pub const RIGHT_PATH_LINK_SOURCE: u64 = __WASI_RIGHT_PATH_LINK_SOURCE;
pub const RIGHT_PATH_LINK_TARGET: u64 = __WASI_RIGHT_PATH_LINK_TARGET;
pub const RIGHT_PATH_OPEN: u64 = __WASI_RIGHT_PATH_OPEN;
pub const RIGHT_FD_READDIR: u64 = __WASI_RIGHT_FD_READDIR;
pub const RIGHT_PATH_READLINK: u64 = __WASI_RIGHT_PATH_READLINK;
pub const RIGHT_PATH_RENAME_SOURCE: u64 = __WASI_RIGHT_PATH_RENAME_SOURCE;
pub const RIGHT_PATH_RENAME_TARGET: u64 = __WASI_RIGHT_PATH_RENAME_TARGET;
pub const RIGHT_PATH_FILESTAT_GET: u64 = __WASI_RIGHT_PATH_FILESTAT_GET;
pub const RIGHT_PATH_FILESTAT_SET_SIZE: u64 = __WASI_RIGHT_PATH_FILESTAT_SET_SIZE;
pub const RIGHT_PATH_FILESTAT_SET_TIMES: u64 = __WASI_RIGHT_PATH_FILESTAT_SET_TIMES;
pub const RIGHT_FD_FILESTAT_GET: u64 = __WASI_RIGHT_FD_FILESTAT_GET;
pub const RIGHT_FD_FILESTAT_SET_SIZE: u64 = __WASI_RIGHT_FD_FILESTAT_SET_SIZE;
pub const RIGHT_FD_FILESTAT_SET_TIMES: u64 = __WASI_RIGHT_FD_FILESTAT_SET_TIMES;
pub const RIGHT_PATH_SYMLINK: u64 = __WASI_RIGHT_PATH_SYMLINK;
pub const RIGHT_PATH_REMOVE_DIRECTORY: u64 = __WASI_RIGHT_PATH_REMOVE_DIRECTORY;
pub const RIGHT_PATH_UNLINK_FILE: u64 = __WASI_RIGHT_PATH_UNLINK_FILE;
pub const RIGHT_POLL_FD_READWRITE: u64 = __WASI_RIGHT_POLL_FD_READWRITE;
pub const RIGHT_SOCK_SHUTDOWN: u64 = __WASI_RIGHT_SOCK_SHUTDOWN;
pub const SOCK_RECV_DATA_TRUNCATED: u16 = __WASI_SOCK_RECV_DATA_TRUNCATED;
pub const SHUT_RD: u8 = __WASI_SHUT_RD;
pub const SHUT_WR: u8 = __WASI_SHUT_WR;
pub const SIGHUP: u8 = __WASI_SIGHUP;
pub const SIGINT: u8 = __WASI_SIGINT;
pub const SIGQUIT: u8 = __WASI_SIGQUIT;
pub const SIGILL: u8 = __WASI_SIGILL;
pub const SIGTRAP: u8 = __WASI_SIGTRAP;
pub const SIGABRT: u8 = __WASI_SIGABRT;
pub const SIGBUS: u8 = __WASI_SIGBUS;
pub const SIGFPE: u8 = __WASI_SIGFPE;
pub const SIGKILL: u8 = __WASI_SIGKILL;
pub const SIGUSR1: u8 = __WASI_SIGUSR1;
pub const SIGSEGV: u8 = __WASI_SIGSEGV;
pub const SIGUSR2: u8 = __WASI_SIGUSR2;
pub const SIGPIPE: u8 = __WASI_SIGPIPE;
pub const SIGALRM: u8 = __WASI_SIGALRM;
pub const SIGTERM: u8 = __WASI_SIGTERM;
pub const SIGCHLD: u8 = __WASI_SIGCHLD;
pub const SIGCONT: u8 = __WASI_SIGCONT;
pub const SIGSTOP: u8 = __WASI_SIGSTOP;
pub const SIGTSTP: u8 = __WASI_SIGTSTP;
pub const SIGTTIN: u8 = __WASI_SIGTTIN;
pub const SIGTTOU: u8 = __WASI_SIGTTOU;
pub const SIGURG: u8 = __WASI_SIGURG;
pub const SIGXCPU: u8 = __WASI_SIGXCPU;
pub const SIGXFSZ: u8 = __WASI_SIGXFSZ;
pub const SIGVTALRM: u8 = __WASI_SIGVTALRM;
pub const SIGPROF: u8 = __WASI_SIGPROF;
pub const SIGWINCH: u8 = __WASI_SIGWINCH;
pub const SIGPOLL: u8 = __WASI_SIGPOLL;
pub const SIGPWR: u8 = __WASI_SIGPWR;
pub const SIGSYS: u8 = __WASI_SIGSYS;
pub const SUBSCRIPTION_CLOCK_ABSTIME: u16 = __WASI_SUBSCRIPTION_CLOCK_ABSTIME;
pub const WHENCE_CUR: u8 = __WASI_WHENCE_CUR;
pub const WHENCE_END: u8 = __WASI_WHENCE_END;
pub const WHENCE_SET: u8 = __WASI_WHENCE_SET;

pub fn clock_res_get(clock_id: clockid_t) -> (errno_t, timestamp_t) {
    let mut resolution = MaybeUninit::<timestamp_t>::uninit();
    unsafe {
        (
            __wasi_clock_res_get(clock_id, resolution.as_mut_ptr()),
            resolution.assume_init(),
        )
    }
}

pub fn clock_time_get(clock_id: clockid_t, precision: timestamp_t) -> (errno_t, timestamp_t) {
    let mut time = MaybeUninit::<timestamp_t>::uninit();
    unsafe {
        (
            __wasi_clock_time_get(clock_id, precision, time.as_mut_ptr()),
            time.assume_init(),
        )
    }
}

pub fn fd_pread(fd: fd_t, iovs: &[iovec_t], offset: filesize_t) -> (errno_t, usize) {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset, nread.as_mut_ptr()),
            nread.assume_init(),
        )
    }
}

pub fn fd_pwrite(fd: fd_t, iovs: &[ciovec_t], offset: filesize_t) -> (__wasi_errno_t, usize) {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset, nwritten.as_mut_ptr()),
            nwritten.assume_init(),
        )
    }
}

pub fn random_get(buf: &mut [u8]) -> errno_t {
    unsafe { __wasi_random_get(buf.as_mut_ptr() as *mut c_void, buf.len()) }
}

pub fn fd_close(fd: fd_t) -> errno_t {
    unsafe { __wasi_fd_close(fd) }
}

pub fn fd_datasync(fd: fd_t) -> errno_t {
    unsafe { __wasi_fd_datasync(fd) }
}

pub fn fd_read(fd: fd_t, iovs: &[iovec_t]) -> (errno_t, usize) {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_read(fd, iovs.as_ptr(), iovs.len(), nread.as_mut_ptr()),
            nread.assume_init(),
        )
    }
}

pub fn fd_renumber(from: fd_t, to: fd_t) -> errno_t {
    unsafe { __wasi_fd_renumber(from, to) }
}

pub fn fd_seek(fd: fd_t, offset: filedelta_t, whence: whence_t) -> (errno_t, filesize_t) {
    let mut newoffset = MaybeUninit::<filesize_t>::uninit();
    unsafe {
        (
            __wasi_fd_seek(fd, offset, whence, newoffset.as_mut_ptr()),
            newoffset.assume_init(),
        )
    }
}

pub fn fd_tell(fd: fd_t) -> (errno_t, filesize_t) {
    let mut newoffset = MaybeUninit::<filesize_t>::uninit();
    unsafe {
        (
            __wasi_fd_tell(fd, newoffset.as_mut_ptr()),
            newoffset.assume_init(),
        )
    }
}

pub fn fd_fdstat_get(fd: fd_t) -> (errno_t, fdstat_t) {
    let mut buf = MaybeUninit::<fdstat_t>::uninit();
    unsafe {
        (
            __wasi_fd_fdstat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_fdstat_set_flags(fd: fd_t, flags: fdflags_t) -> errno_t {
    unsafe { __wasi_fd_fdstat_set_flags(fd, flags) }
}

pub fn fd_fdstat_set_rights(
    fd: fd_t,
    fs_rights_base: rights_t,
    fs_rights_inheriting: rights_t,
) -> errno_t {
    unsafe { __wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting) }
}

pub fn fd_sync(fd: fd_t) -> errno_t {
    unsafe { __wasi_fd_sync(fd) }
}

pub fn fd_write(fd: fd_t, iovs: &[ciovec_t]) -> (errno_t, usize) {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_write(fd, iovs.as_ptr(), iovs.len(), nwritten.as_mut_ptr()),
            nwritten.assume_init(),
        )
    }
}

pub fn fd_advise(fd: fd_t, offset: filesize_t, len: filesize_t, advice: advice_t) -> errno_t {
    unsafe { __wasi_fd_advise(fd, offset, len, advice) }
}

pub fn fd_allocate(fd: fd_t, offset: filesize_t, len: filesize_t) -> errno_t {
    unsafe { __wasi_fd_allocate(fd, offset, len) }
}

pub fn path_create_directory(fd: fd_t, path: &[u8]) -> errno_t {
    unsafe { __wasi_path_create_directory(fd, path.as_ptr(), path.len()) }
}

pub fn path_link(
    old_fd: fd_t,
    old_flags: lookupflags_t,
    old_path: &[u8],
    new_fd: fd_t,
    new_path: &[u8],
) -> errno_t {
    unsafe {
        __wasi_path_link(
            old_fd,
            old_flags,
            old_path.as_ptr(),
            old_path.len(),
            new_fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn path_open(
    dirfd: fd_t,
    dirflags: lookupflags_t,
    path: &[u8],
    oflags: oflags_t,
    fs_rights_base: rights_t,
    fs_rights_inheriting: rights_t,
    fs_flags: fdflags_t,
) -> (errno_t, fd_t) {
    let mut fd = MaybeUninit::<fd_t>::uninit();
    unsafe {
        (
            __wasi_path_open(
                dirfd,
                dirflags,
                path.as_ptr(),
                path.len(),
                oflags,
                fs_rights_base,
                fs_rights_inheriting,
                fs_flags,
                fd.as_mut_ptr(),
            ),
            fd.assume_init(),
        )
    }
}

pub fn fd_readdir(fd: fd_t, buf: &mut [u8], cookie: dircookie_t) -> (errno_t, usize) {
    let mut bufused = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_readdir(
                fd,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                cookie,
                bufused.as_mut_ptr(),
            ),
            bufused.assume_init(),
        )
    }
}

pub fn path_readlink(fd: fd_t, path: &[u8], buf: &mut [u8]) -> (errno_t, usize) {
    let mut bufused = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_path_readlink(
                fd,
                path.as_ptr(),
                path.len(),
                buf.as_mut_ptr(),
                buf.len(),
                bufused.as_mut_ptr(),
            ),
            bufused.assume_init(),
        )
    }
}

pub fn path_rename(old_fd: fd_t, old_path: &[u8], new_fd: fd_t, new_path: &[u8]) -> errno_t {
    unsafe {
        __wasi_path_rename(
            old_fd,
            old_path.as_ptr(),
            old_path.len(),
            new_fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn fd_filestat_get(fd: fd_t) -> (errno_t, filestat_t) {
    let mut buf = MaybeUninit::<filestat_t>::uninit();
    unsafe {
        (
            __wasi_fd_filestat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_filestat_set_times(
    fd: fd_t,
    st_atim: timestamp_t,
    st_mtim: timestamp_t,
    fstflags: fstflags_t,
) -> errno_t {
    unsafe { __wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fstflags) }
}

pub fn fd_filestat_set_size(fd: fd_t, st_size: filesize_t) -> errno_t {
    unsafe { __wasi_fd_filestat_set_size(fd, st_size) }
}

pub fn path_filestat_get(fd: fd_t, flags: lookupflags_t, path: &[u8]) -> (errno_t, filestat_t) {
    let mut buf = MaybeUninit::<filestat_t>::uninit();
    unsafe {
        (
            __wasi_path_filestat_get(fd, flags, path.as_ptr(), path.len(), buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn path_filestat_set_times(
    fd: fd_t,
    flags: lookupflags_t,
    path: &[u8],
    st_atim: timestamp_t,
    st_mtim: timestamp_t,
    fstflags: fstflags_t,
) -> errno_t {
    unsafe {
        __wasi_path_filestat_set_times(
            fd,
            flags,
            path.as_ptr(),
            path.len(),
            st_atim,
            st_mtim,
            fstflags,
        )
    }
}

pub fn path_symlink(old_path: &[u8], fd: fd_t, new_path: &[u8]) -> errno_t {
    unsafe {
        __wasi_path_symlink(
            old_path.as_ptr(),
            old_path.len(),
            fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn path_unlink_file(fd: fd_t, path: &[u8]) -> errno_t {
    unsafe { __wasi_path_unlink_file(fd, path.as_ptr(), path.len()) }
}

pub fn path_remove_directory(fd: fd_t, path: &[u8]) -> errno_t {
    unsafe { __wasi_path_remove_directory(fd, path.as_ptr(), path.len()) }
}

pub fn poll_oneoff(in_: &[subscription_t], out: &mut [event_t]) -> (errno_t, usize) {
    assert!(out.len() >= in_.len());
    let mut nevents = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_poll_oneoff(
                in_.as_ptr(),
                out.as_mut_ptr(),
                in_.len(),
                nevents.as_mut_ptr(),
            ),
            nevents.assume_init(),
        )
    }
}

pub fn proc_exit(rval: exitcode_t) {
    unsafe { __wasi_proc_exit(rval) }
}

pub fn proc_raise(sig: signal_t) -> errno_t {
    unsafe { __wasi_proc_raise(sig) }
}

pub fn sock_recv(
    sock: fd_t,
    ri_data: &[iovec_t],
    ri_flags: riflags_t,
) -> (errno_t, usize, roflags_t) {
    let mut ro_datalen = MaybeUninit::<usize>::uninit();
    let mut ro_flags = MaybeUninit::<roflags_t>::uninit();
    unsafe {
        (
            __wasi_sock_recv(
                sock,
                ri_data.as_ptr(),
                ri_data.len(),
                ri_flags,
                ro_datalen.as_mut_ptr(),
                ro_flags.as_mut_ptr(),
            ),
            ro_datalen.assume_init(),
            ro_flags.assume_init(),
        )
    }
}

pub fn sock_send(sock: fd_t, si_data: &[ciovec_t], si_flags: siflags_t) -> (errno_t, usize) {
    let mut so_datalen = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_sock_send(
                sock,
                si_data.as_ptr(),
                si_data.len(),
                si_flags,
                so_datalen.as_mut_ptr(),
            ),
            so_datalen.assume_init(),
        )
    }
}

pub fn sock_shutdown(sock: fd_t, how: sdflags_t) -> errno_t {
    unsafe { __wasi_sock_shutdown(sock, how) }
}

pub fn sched_yield() -> errno_t {
    unsafe { __wasi_sched_yield() }
}

pub fn fd_prestat_get(fd: fd_t) -> (errno_t, prestat_t) {
    let mut buf = MaybeUninit::<prestat_t>::uninit();
    unsafe {
        (
            __wasi_fd_prestat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_prestat_dir_name(fd: fd_t, path: &mut [u8]) -> errno_t {
    unsafe { __wasi_fd_prestat_dir_name(fd, path.as_mut_ptr(), path.len()) }
}

// TODO: Safe interfaces to the args and environ functions
/*
pub fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> errno_t {}
pub fn args_sizes_get(argc: *mut usize, argv_buf_size: *mut usize) -> errno_t {}
pub fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> errno_t {}
pub fn environ_sizes_get(environ_count: *mut usize, environ_buf_size: *mut usize) -> errno_t {}
*/
