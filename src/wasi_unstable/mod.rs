//! This module declares the Rust bindings to the `wasi_unstable` API.
//!
//! The raw bindings are in the `raw` submodule. They use raw pointers and
//! are unsafe. In the the top-level module, raw pointer-length pairs are
//! replaced by Rust slice types, output parameters are converted to normal
//! return values, names are translated to be more Rust-idiomatic, and the
//! functions are safe.
//!
//! TODO: Not all functions are covered yet; implement the rest of the API.

pub mod raw;

use core::ffi::c_void;
use core::mem::MaybeUninit;
use raw::*;

pub type Advice = __wasi_advice_t;
pub type Clockid = __wasi_clockid_t;
pub type Device = __wasi_device_t;
pub type Dircookie = __wasi_dircookie_t;
pub type Errno = __wasi_errno_t;
pub type Eventrwflags = __wasi_eventrwflags_t;
pub type Eventtype = __wasi_eventtype_t;
pub type Exitcode = __wasi_exitcode_t;
pub type Fd = __wasi_fd_t;
pub type Fdflags = __wasi_fdflags_t;
pub type Filedelta = __wasi_filedelta_t;
pub type Filesize = __wasi_filesize_t;
pub type Filetype = __wasi_filetype_t;
pub type Fstflags = __wasi_fstflags_t;
pub type Inode = __wasi_inode_t;
pub type Linkcount = __wasi_linkcount_t;
pub type Lookupflags = __wasi_lookupflags_t;
pub type Oflags = __wasi_oflags_t;
pub type Preopentype = __wasi_preopentype_t;
pub type Riflags = __wasi_riflags_t;
pub type Rights = __wasi_rights_t;
pub type Roflags = __wasi_roflags_t;
pub type Sdflags = __wasi_sdflags_t;
pub type Siflags = __wasi_siflags_t;
pub type Signal = __wasi_signal_t;
pub type Subclockflags = __wasi_subclockflags_t;
pub type Timestamp = __wasi_timestamp_t;
pub type Userdata = __wasi_userdata_t;
pub type Whence = __wasi_whence_t;
pub type Dirent = __wasi_dirent_t;
pub type Fdstat = __wasi_fdstat_t;
pub type Filestat = __wasi_filestat_t;
pub type Ciovec = __wasi_ciovec_t;
pub type Iovec = __wasi_iovec_t;
pub type Subscription = __wasi_subscription_t;
pub type Event = __wasi_event_t;
pub type Prestat = __wasi_prestat_t;

pub const ADVICE_NORMAL: Advice = __WASI_ADVICE_NORMAL;
pub const ADVICE_SEQUENTIAL: Advice = __WASI_ADVICE_SEQUENTIAL;
pub const ADVICE_RANDOM: Advice = __WASI_ADVICE_RANDOM;
pub const ADVICE_WILLNEED: Advice = __WASI_ADVICE_WILLNEED;
pub const ADVICE_DONTNEED: Advice = __WASI_ADVICE_DONTNEED;
pub const ADVICE_NOREUSE: Advice = __WASI_ADVICE_NOREUSE;
pub const CLOCK_REALTIME: Clockid = __WASI_CLOCK_REALTIME;
pub const CLOCK_MONOTONIC: Clockid = __WASI_CLOCK_MONOTONIC;
pub const CLOCK_PROCESS_CPUTIME_ID: Clockid = __WASI_CLOCK_PROCESS_CPUTIME_ID;
pub const CLOCK_THREAD_CPUTIME_ID: Clockid = __WASI_CLOCK_THREAD_CPUTIME_ID;
pub const DIRCOOKIE_START: Dircookie = __WASI_DIRCOOKIE_START;
pub const ESUCCESS: Errno = __WASI_ESUCCESS;
pub const E2BIG: Errno = __WASI_E2BIG;
pub const EACCES: Errno = __WASI_EACCES;
pub const EADDRINUSE: Errno = __WASI_EADDRINUSE;
pub const EADDRNOTAVAIL: Errno = __WASI_EADDRNOTAVAIL;
pub const EAFNOSUPPORT: Errno = __WASI_EAFNOSUPPORT;
pub const EAGAIN: Errno = __WASI_EAGAIN;
pub const EALREADY: Errno = __WASI_EALREADY;
pub const EBADF: Errno = __WASI_EBADF;
pub const EBADMSG: Errno = __WASI_EBADMSG;
pub const EBUSY: Errno = __WASI_EBUSY;
pub const ECANCELED: Errno = __WASI_ECANCELED;
pub const ECHILD: Errno = __WASI_ECHILD;
pub const ECONNABORTED: Errno = __WASI_ECONNABORTED;
pub const ECONNREFUSED: Errno = __WASI_ECONNREFUSED;
pub const ECONNRESET: Errno = __WASI_ECONNRESET;
pub const EDEADLK: Errno = __WASI_EDEADLK;
pub const EDESTADDRREQ: Errno = __WASI_EDESTADDRREQ;
pub const EDOM: Errno = __WASI_EDOM;
pub const EDQUOT: Errno = __WASI_EDQUOT;
pub const EEXIST: Errno = __WASI_EEXIST;
pub const EFAULT: Errno = __WASI_EFAULT;
pub const EFBIG: Errno = __WASI_EFBIG;
pub const EHOSTUNREACH: Errno = __WASI_EHOSTUNREACH;
pub const EIDRM: Errno = __WASI_EIDRM;
pub const EILSEQ: Errno = __WASI_EILSEQ;
pub const EINPROGRESS: Errno = __WASI_EINPROGRESS;
pub const EINTR: Errno = __WASI_EINTR;
pub const EINVAL: Errno = __WASI_EINVAL;
pub const EIO: Errno = __WASI_EIO;
pub const EISCONN: Errno = __WASI_EISCONN;
pub const EISDIR: Errno = __WASI_EISDIR;
pub const ELOOP: Errno = __WASI_ELOOP;
pub const EMFILE: Errno = __WASI_EMFILE;
pub const EMLINK: Errno = __WASI_EMLINK;
pub const EMSGSIZE: Errno = __WASI_EMSGSIZE;
pub const EMULTIHOP: Errno = __WASI_EMULTIHOP;
pub const ENAMETOOLONG: Errno = __WASI_ENAMETOOLONG;
pub const ENETDOWN: Errno = __WASI_ENETDOWN;
pub const ENETRESET: Errno = __WASI_ENETRESET;
pub const ENETUNREACH: Errno = __WASI_ENETUNREACH;
pub const ENFILE: Errno = __WASI_ENFILE;
pub const ENOBUFS: Errno = __WASI_ENOBUFS;
pub const ENODEV: Errno = __WASI_ENODEV;
pub const ENOENT: Errno = __WASI_ENOENT;
pub const ENOEXEC: Errno = __WASI_ENOEXEC;
pub const ENOLCK: Errno = __WASI_ENOLCK;
pub const ENOLINK: Errno = __WASI_ENOLINK;
pub const ENOMEM: Errno = __WASI_ENOMEM;
pub const ENOMSG: Errno = __WASI_ENOMSG;
pub const ENOPROTOOPT: Errno = __WASI_ENOPROTOOPT;
pub const ENOSPC: Errno = __WASI_ENOSPC;
pub const ENOSYS: Errno = __WASI_ENOSYS;
pub const ENOTCONN: Errno = __WASI_ENOTCONN;
pub const ENOTDIR: Errno = __WASI_ENOTDIR;
pub const ENOTEMPTY: Errno = __WASI_ENOTEMPTY;
pub const ENOTRECOVERABLE: Errno = __WASI_ENOTRECOVERABLE;
pub const ENOTSOCK: Errno = __WASI_ENOTSOCK;
pub const ENOTSUP: Errno = __WASI_ENOTSUP;
pub const ENOTTY: Errno = __WASI_ENOTTY;
pub const ENXIO: Errno = __WASI_ENXIO;
pub const EOVERFLOW: Errno = __WASI_EOVERFLOW;
pub const EOWNERDEAD: Errno = __WASI_EOWNERDEAD;
pub const EPERM: Errno = __WASI_EPERM;
pub const EPIPE: Errno = __WASI_EPIPE;
pub const EPROTO: Errno = __WASI_EPROTO;
pub const EPROTONOSUPPORT: Errno = __WASI_EPROTONOSUPPORT;
pub const EPROTOTYPE: Errno = __WASI_EPROTOTYPE;
pub const ERANGE: Errno = __WASI_ERANGE;
pub const EROFS: Errno = __WASI_EROFS;
pub const ESPIPE: Errno = __WASI_ESPIPE;
pub const ESRCH: Errno = __WASI_ESRCH;
pub const ESTALE: Errno = __WASI_ESTALE;
pub const ETIMEDOUT: Errno = __WASI_ETIMEDOUT;
pub const ETXTBSY: Errno = __WASI_ETXTBSY;
pub const EXDEV: Errno = __WASI_EXDEV;
pub const ENOTCAPABLE: Errno = __WASI_ENOTCAPABLE;
pub const EVENT_FD_READWRITE_HANGUP: Eventrwflags = __WASI_EVENT_FD_READWRITE_HANGUP;
pub const EVENTTYPE_CLOCK: Eventtype = __WASI_EVENTTYPE_CLOCK;
pub const EVENTTYPE_FD_READ: Eventtype = __WASI_EVENTTYPE_FD_READ;
pub const EVENTTYPE_FD_WRITE: Eventtype = __WASI_EVENTTYPE_FD_WRITE;
pub const FDFLAG_APPEND: Fdflags = __WASI_FDFLAG_APPEND;
pub const FDFLAG_DSYNC: Fdflags = __WASI_FDFLAG_DSYNC;
pub const FDFLAG_NONBLOCK: Fdflags = __WASI_FDFLAG_NONBLOCK;
pub const FDFLAG_RSYNC: Fdflags = __WASI_FDFLAG_RSYNC;
pub const FDFLAG_SYNC: Fdflags = __WASI_FDFLAG_SYNC;
pub const FILETYPE_UNKNOWN: Filetype = __WASI_FILETYPE_UNKNOWN;
pub const FILETYPE_BLOCK_DEVICE: Filetype = __WASI_FILETYPE_BLOCK_DEVICE;
pub const FILETYPE_CHARACTER_DEVICE: Filetype = __WASI_FILETYPE_CHARACTER_DEVICE;
pub const FILETYPE_DIRECTORY: Filetype = __WASI_FILETYPE_DIRECTORY;
pub const FILETYPE_REGULAR_FILE: Filetype = __WASI_FILETYPE_REGULAR_FILE;
pub const FILETYPE_SOCKET_DGRAM: Filetype = __WASI_FILETYPE_SOCKET_DGRAM;
pub const FILETYPE_SOCKET_STREAM: Filetype = __WASI_FILETYPE_SOCKET_STREAM;
pub const FILETYPE_SYMBOLIC_LINK: Filetype = __WASI_FILETYPE_SYMBOLIC_LINK;
pub const FILESTAT_SET_ATIM: Fstflags = __WASI_FILESTAT_SET_ATIM;
pub const FILESTAT_SET_ATIM_NOW: Fstflags = __WASI_FILESTAT_SET_ATIM_NOW;
pub const FILESTAT_SET_MTIM: Fstflags = __WASI_FILESTAT_SET_MTIM;
pub const FILESTAT_SET_MTIM_NOW: Fstflags = __WASI_FILESTAT_SET_MTIM_NOW;
pub const LOOKUP_SYMLINK_FOLLOW: Lookupflags = __WASI_LOOKUP_SYMLINK_FOLLOW;
pub const O_CREAT: Oflags = __WASI_O_CREAT;
pub const O_DIRECTORY: Oflags = __WASI_O_DIRECTORY;
pub const O_EXCL: Oflags = __WASI_O_EXCL;
pub const O_TRUNC: Oflags = __WASI_O_TRUNC;
pub const PREOPENTYPE_DIR: Preopentype = __WASI_PREOPENTYPE_DIR;
pub const SOCK_RECV_PEEK: Riflags = __WASI_SOCK_RECV_PEEK;
pub const SOCK_RECV_WAITALL: Riflags = __WASI_SOCK_RECV_WAITALL;
pub const RIGHT_FD_DATASYNC: Rights = __WASI_RIGHT_FD_DATASYNC;
pub const RIGHT_FD_READ: Rights = __WASI_RIGHT_FD_READ;
pub const RIGHT_FD_SEEK: Rights = __WASI_RIGHT_FD_SEEK;
pub const RIGHT_FD_FDSTAT_SET_FLAGS: Rights = __WASI_RIGHT_FD_FDSTAT_SET_FLAGS;
pub const RIGHT_FD_SYNC: Rights = __WASI_RIGHT_FD_SYNC;
pub const RIGHT_FD_TELL: Rights = __WASI_RIGHT_FD_TELL;
pub const RIGHT_FD_WRITE: Rights = __WASI_RIGHT_FD_WRITE;
pub const RIGHT_FD_ADVISE: Rights = __WASI_RIGHT_FD_ADVISE;
pub const RIGHT_FD_ALLOCATE: Rights = __WASI_RIGHT_FD_ALLOCATE;
pub const RIGHT_PATH_CREATE_DIRECTORY: Rights = __WASI_RIGHT_PATH_CREATE_DIRECTORY;
pub const RIGHT_PATH_CREATE_FILE: Rights = __WASI_RIGHT_PATH_CREATE_FILE;
pub const RIGHT_PATH_LINK_SOURCE: Rights = __WASI_RIGHT_PATH_LINK_SOURCE;
pub const RIGHT_PATH_LINK_TARGET: Rights = __WASI_RIGHT_PATH_LINK_TARGET;
pub const RIGHT_PATH_OPEN: Rights = __WASI_RIGHT_PATH_OPEN;
pub const RIGHT_FD_READDIR: Rights = __WASI_RIGHT_FD_READDIR;
pub const RIGHT_PATH_READLINK: Rights = __WASI_RIGHT_PATH_READLINK;
pub const RIGHT_PATH_RENAME_SOURCE: Rights = __WASI_RIGHT_PATH_RENAME_SOURCE;
pub const RIGHT_PATH_RENAME_TARGET: Rights = __WASI_RIGHT_PATH_RENAME_TARGET;
pub const RIGHT_PATH_FILESTAT_GET: Rights = __WASI_RIGHT_PATH_FILESTAT_GET;
pub const RIGHT_PATH_FILESTAT_SET_SIZE: Rights = __WASI_RIGHT_PATH_FILESTAT_SET_SIZE;
pub const RIGHT_PATH_FILESTAT_SET_TIMES: Rights = __WASI_RIGHT_PATH_FILESTAT_SET_TIMES;
pub const RIGHT_FD_FILESTAT_GET: Rights = __WASI_RIGHT_FD_FILESTAT_GET;
pub const RIGHT_FD_FILESTAT_SET_SIZE: Rights = __WASI_RIGHT_FD_FILESTAT_SET_SIZE;
pub const RIGHT_FD_FILESTAT_SET_TIMES: Rights = __WASI_RIGHT_FD_FILESTAT_SET_TIMES;
pub const RIGHT_PATH_SYMLINK: Rights = __WASI_RIGHT_PATH_SYMLINK;
pub const RIGHT_PATH_REMOVE_DIRECTORY: Rights = __WASI_RIGHT_PATH_REMOVE_DIRECTORY;
pub const RIGHT_PATH_UNLINK_FILE: Rights = __WASI_RIGHT_PATH_UNLINK_FILE;
pub const RIGHT_POLL_FD_READWRITE: Rights = __WASI_RIGHT_POLL_FD_READWRITE;
pub const RIGHT_SOCK_SHUTDOWN: Rights = __WASI_RIGHT_SOCK_SHUTDOWN;
pub const SOCK_RECV_DATA_TRUNCATED: Roflags = __WASI_SOCK_RECV_DATA_TRUNCATED;
pub const SHUT_RD: Sdflags = __WASI_SHUT_RD;
pub const SHUT_WR: Sdflags = __WASI_SHUT_WR;
pub const SIGHUP: Signal = __WASI_SIGHUP;
pub const SIGINT: Signal = __WASI_SIGINT;
pub const SIGQUIT: Signal = __WASI_SIGQUIT;
pub const SIGILL: Signal = __WASI_SIGILL;
pub const SIGTRAP: Signal = __WASI_SIGTRAP;
pub const SIGABRT: Signal = __WASI_SIGABRT;
pub const SIGBUS: Signal = __WASI_SIGBUS;
pub const SIGFPE: Signal = __WASI_SIGFPE;
pub const SIGKILL: Signal = __WASI_SIGKILL;
pub const SIGUSR1: Signal = __WASI_SIGUSR1;
pub const SIGSEGV: Signal = __WASI_SIGSEGV;
pub const SIGUSR2: Signal = __WASI_SIGUSR2;
pub const SIGPIPE: Signal = __WASI_SIGPIPE;
pub const SIGALRM: Signal = __WASI_SIGALRM;
pub const SIGTERM: Signal = __WASI_SIGTERM;
pub const SIGCHLD: Signal = __WASI_SIGCHLD;
pub const SIGCONT: Signal = __WASI_SIGCONT;
pub const SIGSTOP: Signal = __WASI_SIGSTOP;
pub const SIGTSTP: Signal = __WASI_SIGTSTP;
pub const SIGTTIN: Signal = __WASI_SIGTTIN;
pub const SIGTTOU: Signal = __WASI_SIGTTOU;
pub const SIGURG: Signal = __WASI_SIGURG;
pub const SIGXCPU: Signal = __WASI_SIGXCPU;
pub const SIGXFSZ: Signal = __WASI_SIGXFSZ;
pub const SIGVTALRM: Signal = __WASI_SIGVTALRM;
pub const SIGPROF: Signal = __WASI_SIGPROF;
pub const SIGWINCH: Signal = __WASI_SIGWINCH;
pub const SIGPOLL: Signal = __WASI_SIGPOLL;
pub const SIGPWR: Signal = __WASI_SIGPWR;
pub const SIGSYS: Signal = __WASI_SIGSYS;
pub const SUBSCRIPTION_CLOCK_ABSTIME: Subclockflags = __WASI_SUBSCRIPTION_CLOCK_ABSTIME;
pub const WHENCE_CUR: Whence = __WASI_WHENCE_CUR;
pub const WHENCE_END: Whence = __WASI_WHENCE_END;
pub const WHENCE_SET: Whence = __WASI_WHENCE_SET;

pub fn clock_res_get(clock_id: Clockid) -> (Errno, Timestamp) {
    let mut resolution = MaybeUninit::<Timestamp>::uninit();
    unsafe {
        (
            __wasi_clock_res_get(clock_id, resolution.as_mut_ptr()),
            resolution.assume_init(),
        )
    }
}

pub fn clock_time_get(clock_id: Clockid, precision: Timestamp) -> (Errno, Timestamp) {
    let mut time = MaybeUninit::<Timestamp>::uninit();
    unsafe {
        (
            __wasi_clock_time_get(clock_id, precision, time.as_mut_ptr()),
            time.assume_init(),
        )
    }
}

pub fn fd_pread(fd: Fd, iovs: &[Iovec], offset: Filesize) -> (Errno, usize) {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset, nread.as_mut_ptr()),
            nread.assume_init(),
        )
    }
}

pub fn fd_pwrite(fd: Fd, iovs: &[Ciovec], offset: Filesize) -> (Errno, usize) {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset, nwritten.as_mut_ptr()),
            nwritten.assume_init(),
        )
    }
}

pub fn random_get(buf: &mut [u8]) -> Errno {
    unsafe { __wasi_random_get(buf.as_mut_ptr() as *mut c_void, buf.len()) }
}

pub fn fd_close(fd: Fd) -> Errno {
    unsafe { __wasi_fd_close(fd) }
}

pub fn fd_datasync(fd: Fd) -> Errno {
    unsafe { __wasi_fd_datasync(fd) }
}

pub fn fd_read(fd: Fd, iovs: &[Iovec]) -> (Errno, usize) {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_read(fd, iovs.as_ptr(), iovs.len(), nread.as_mut_ptr()),
            nread.assume_init(),
        )
    }
}

pub fn fd_renumber(from: Fd, to: Fd) -> Errno {
    unsafe { __wasi_fd_renumber(from, to) }
}

pub fn fd_seek(fd: Fd, offset: Filedelta, whence: Whence) -> (Errno, Filesize) {
    let mut newoffset = MaybeUninit::<Filesize>::uninit();
    unsafe {
        (
            __wasi_fd_seek(fd, offset, whence, newoffset.as_mut_ptr()),
            newoffset.assume_init(),
        )
    }
}

pub fn fd_tell(fd: Fd) -> (Errno, Filesize) {
    let mut newoffset = MaybeUninit::<Filesize>::uninit();
    unsafe {
        (
            __wasi_fd_tell(fd, newoffset.as_mut_ptr()),
            newoffset.assume_init(),
        )
    }
}

pub fn fd_fdstat_get(fd: Fd) -> (Errno, Fdstat) {
    let mut buf = MaybeUninit::<Fdstat>::uninit();
    unsafe {
        (
            __wasi_fd_fdstat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_fdstat_set_flags(fd: Fd, flags: Fdflags) -> Errno {
    unsafe { __wasi_fd_fdstat_set_flags(fd, flags) }
}

pub fn fd_fdstat_set_rights(fd: Fd, fs_rights_base: Rights, fs_rights_inheriting: Rights) -> Errno {
    unsafe { __wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting) }
}

pub fn fd_sync(fd: Fd) -> Errno {
    unsafe { __wasi_fd_sync(fd) }
}

pub fn fd_write(fd: Fd, iovs: &[Ciovec]) -> (Errno, usize) {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe {
        (
            __wasi_fd_write(fd, iovs.as_ptr(), iovs.len(), nwritten.as_mut_ptr()),
            nwritten.assume_init(),
        )
    }
}

pub fn fd_advise(fd: Fd, offset: Filesize, len: Filesize, advice: Advice) -> Errno {
    unsafe { __wasi_fd_advise(fd, offset, len, advice) }
}

pub fn fd_allocate(fd: Fd, offset: Filesize, len: Filesize) -> Errno {
    unsafe { __wasi_fd_allocate(fd, offset, len) }
}

pub fn path_create_directory(fd: Fd, path: &[u8]) -> Errno {
    unsafe { __wasi_path_create_directory(fd, path.as_ptr(), path.len()) }
}

pub fn path_link(
    old_fd: Fd,
    old_flags: Lookupflags,
    old_path: &[u8],
    new_fd: Fd,
    new_path: &[u8],
) -> Errno {
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
    dirfd: Fd,
    dirflags: Lookupflags,
    path: &[u8],
    oflags: Oflags,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fs_flags: Fdflags,
) -> (Errno, Fd) {
    let mut fd = MaybeUninit::<Fd>::uninit();
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

pub fn fd_readdir(fd: Fd, buf: &mut [u8], cookie: Dircookie) -> (Errno, usize) {
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

pub fn path_readlink(fd: Fd, path: &[u8], buf: &mut [u8]) -> (Errno, usize) {
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

pub fn path_rename(old_fd: Fd, old_path: &[u8], new_fd: Fd, new_path: &[u8]) -> Errno {
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

pub fn fd_filestat_get(fd: Fd) -> (Errno, Filestat) {
    let mut buf = MaybeUninit::<Filestat>::uninit();
    unsafe {
        (
            __wasi_fd_filestat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_filestat_set_times(
    fd: Fd,
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: Fstflags,
) -> Errno {
    unsafe { __wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fstflags) }
}

pub fn fd_filestat_set_size(fd: Fd, st_size: Filesize) -> Errno {
    unsafe { __wasi_fd_filestat_set_size(fd, st_size) }
}

pub fn path_filestat_get(fd: Fd, flags: Lookupflags, path: &[u8]) -> (Errno, Filestat) {
    let mut buf = MaybeUninit::<Filestat>::uninit();
    unsafe {
        (
            __wasi_path_filestat_get(fd, flags, path.as_ptr(), path.len(), buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn path_filestat_set_times(
    fd: Fd,
    flags: Lookupflags,
    path: &[u8],
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: Fstflags,
) -> Errno {
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

pub fn path_symlink(old_path: &[u8], fd: Fd, new_path: &[u8]) -> Errno {
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

pub fn path_unlink_file(fd: Fd, path: &[u8]) -> Errno {
    unsafe { __wasi_path_unlink_file(fd, path.as_ptr(), path.len()) }
}

pub fn path_remove_directory(fd: Fd, path: &[u8]) -> Errno {
    unsafe { __wasi_path_remove_directory(fd, path.as_ptr(), path.len()) }
}

pub fn poll_oneoff(in_: &[Subscription], out: &mut [Event]) -> (Errno, usize) {
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

pub fn proc_exit(rval: Exitcode) {
    unsafe { __wasi_proc_exit(rval) }
}

pub fn proc_raise(sig: Signal) -> Errno {
    unsafe { __wasi_proc_raise(sig) }
}

pub fn sock_recv(sock: Fd, ri_data: &[Iovec], ri_flags: Riflags) -> (Errno, usize, Roflags) {
    let mut ro_datalen = MaybeUninit::<usize>::uninit();
    let mut ro_flags = MaybeUninit::<Roflags>::uninit();
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

pub fn sock_send(sock: Fd, si_data: &[Ciovec], si_flags: Siflags) -> (Errno, usize) {
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

pub fn sock_shutdown(sock: Fd, how: Sdflags) -> Errno {
    unsafe { __wasi_sock_shutdown(sock, how) }
}

pub fn sched_yield() -> Errno {
    unsafe { __wasi_sched_yield() }
}

pub fn fd_prestat_get(fd: Fd) -> (Errno, Prestat) {
    let mut buf = MaybeUninit::<Prestat>::uninit();
    unsafe {
        (
            __wasi_fd_prestat_get(fd, buf.as_mut_ptr()),
            buf.assume_init(),
        )
    }
}

pub fn fd_prestat_dir_name(fd: Fd, path: &mut [u8]) -> Errno {
    unsafe { __wasi_fd_prestat_dir_name(fd, path.as_mut_ptr(), path.len()) }
}

// TODO: Safe interfaces to the args and environ functions
/*
pub fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Errno {}
pub fn args_sizes_get(argc: *mut usize, argv_buf_size: *mut usize) -> Errno {}
pub fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> Errno {}
pub fn environ_sizes_get(environ_count: *mut usize, environ_buf_size: *mut usize) -> Errno {}
*/
