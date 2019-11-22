//! This module declares the Rust bindings to the `wasi_snapshot_preview1` API.
//!
//! The raw bindings are in the `raw` submodule. They use raw pointers and
//! are unsafe. In the the top-level module, raw pointer-length pairs are
//! replaced by Rust slice types, output parameters are converted to normal
//! return values, names are translated to be more Rust-idiomatic, and the
//! functions are safe.
//!
//! TODO: Not all functions are covered yet; implement the rest of the API.

pub mod raw;

use core::mem::MaybeUninit;
use core::num::NonZeroU16;
use raw::*;

pub type Advice = __wasi_advice_t;
pub type ClockId = __wasi_clockid_t;
pub type Device = __wasi_device_t;
pub type DirCookie = __wasi_dircookie_t;
pub type Errno = __wasi_errno_t;
pub type Error = NonZeroU16;
pub type EventRwFlags = __wasi_eventrwflags_t;
pub type EventType = __wasi_eventtype_t;
pub type ExitCode = __wasi_exitcode_t;
pub type Fd = __wasi_fd_t;
pub type FdFlags = __wasi_fdflags_t;
pub type FileDelta = __wasi_filedelta_t;
pub type FileSize = __wasi_filesize_t;
pub type FileType = __wasi_filetype_t;
pub type FstFlags = __wasi_fstflags_t;
pub type Inode = __wasi_inode_t;
pub type LinkCount = __wasi_linkcount_t;
pub type LookupFlags = __wasi_lookupflags_t;
pub type OFlags = __wasi_oflags_t;
pub type PreopenType = __wasi_preopentype_t;
pub type RiFlags = __wasi_riflags_t;
pub type Rights = __wasi_rights_t;
pub type RoFlags = __wasi_roflags_t;
pub type SdFlags = __wasi_sdflags_t;
pub type SiFlags = __wasi_siflags_t;
pub type Signal = __wasi_signal_t;
pub type SubclockFlags = __wasi_subclockflags_t;
pub type Timestamp = __wasi_timestamp_t;
pub type Userdata = __wasi_userdata_t;
pub type Whence = __wasi_whence_t;
pub type Dirent = __wasi_dirent_t;
pub type FdStat = __wasi_fdstat_t;
pub type FileStat = __wasi_filestat_t;
pub type CIoVec = __wasi_ciovec_t;
pub type IoVec = __wasi_iovec_t;
pub type Subscription = __wasi_subscription_t;
pub type Event = __wasi_event_t;
pub type Prestat = __wasi_prestat_t;

// Assert that `__WASI_ERRNO_SUCCESS` equals to 0
const _ASSERT1: [(); 0] = [(); __WASI_ERRNO_SUCCESS as usize];

pub const ADVICE_NORMAL: Advice = __WASI_ADVICE_NORMAL;
pub const ADVICE_SEQUENTIAL: Advice = __WASI_ADVICE_SEQUENTIAL;
pub const ADVICE_RANDOM: Advice = __WASI_ADVICE_RANDOM;
pub const ADVICE_WILLNEED: Advice = __WASI_ADVICE_WILLNEED;
pub const ADVICE_DONTNEED: Advice = __WASI_ADVICE_DONTNEED;
pub const ADVICE_NOREUSE: Advice = __WASI_ADVICE_NOREUSE;
pub const CLOCKID_REALTIME: ClockId = __WASI_CLOCKID_REALTIME;
pub const CLOCKID_MONOTONIC: ClockId = __WASI_CLOCKID_MONOTONIC;
pub const CLOCKID_PROCESS_CPUTIME_ID: ClockId = __WASI_CLOCKID_PROCESS_CPUTIME_ID;
pub const CLOCKID_THREAD_CPUTIME_ID: ClockId = __WASI_CLOCKID_THREAD_CPUTIME_ID;
pub const DIRCOOKIE_START: DirCookie = 0;

pub const STDIN_FD: Fd = 0;
pub const STDOUT_FD: Fd = 1;
pub const STDERR_FD: Fd = 2;

macro_rules! errno_set {
    {$($safe_const:ident = $raw_const:ident;)*} => {
        $(
            pub const $safe_const: Error = unsafe {
                NonZeroU16::new_unchecked($raw_const)
            };
        )*
    };
}

errno_set! {
    ERRNO_2BIG = __WASI_ERRNO_2BIG;
    ERRNO_ACCES = __WASI_ERRNO_ACCES;
    ERRNO_ADDRINUSE = __WASI_ERRNO_ADDRINUSE;
    ERRNO_ADDRNOTAVAIL = __WASI_ERRNO_ADDRNOTAVAIL;
    ERRNO_AFNOSUPPORT = __WASI_ERRNO_AFNOSUPPORT;
    ERRNO_AGAIN = __WASI_ERRNO_AGAIN;
    ERRNO_ALREADY = __WASI_ERRNO_ALREADY;
    ERRNO_BADF = __WASI_ERRNO_BADF;
    ERRNO_BADMSG = __WASI_ERRNO_BADMSG;
    ERRNO_BUSY = __WASI_ERRNO_BUSY;
    ERRNO_CANCELED = __WASI_ERRNO_CANCELED;
    ERRNO_CHILD = __WASI_ERRNO_CHILD;
    ERRNO_CONNABORTED = __WASI_ERRNO_CONNABORTED;
    ERRNO_CONNREFUSED = __WASI_ERRNO_CONNREFUSED;
    ERRNO_CONNRESET = __WASI_ERRNO_CONNRESET;
    ERRNO_DEADLK = __WASI_ERRNO_DEADLK;
    ERRNO_DESTADDRREQ = __WASI_ERRNO_DESTADDRREQ;
    ERRNO_DOM = __WASI_ERRNO_DOM;
    ERRNO_DQUOT = __WASI_ERRNO_DQUOT;
    ERRNO_EXIST = __WASI_ERRNO_EXIST;
    ERRNO_FAULT = __WASI_ERRNO_FAULT;
    ERRNO_FBIG = __WASI_ERRNO_FBIG;
    ERRNO_HOSTUNREACH = __WASI_ERRNO_HOSTUNREACH;
    ERRNO_IDRM = __WASI_ERRNO_IDRM;
    ERRNO_ILSEQ = __WASI_ERRNO_ILSEQ;
    ERRNO_INPROGRESS = __WASI_ERRNO_INPROGRESS;
    ERRNO_INTR = __WASI_ERRNO_INTR;
    ERRNO_INVAL = __WASI_ERRNO_INVAL;
    ERRNO_IO = __WASI_ERRNO_IO;
    ERRNO_ISCONN = __WASI_ERRNO_ISCONN;
    ERRNO_ISDIR = __WASI_ERRNO_ISDIR;
    ERRNO_LOOP = __WASI_ERRNO_LOOP;
    ERRNO_MFILE = __WASI_ERRNO_MFILE;
    ERRNO_MLINK = __WASI_ERRNO_MLINK;
    ERRNO_MSGSIZE = __WASI_ERRNO_MSGSIZE;
    ERRNO_MULTIHOP = __WASI_ERRNO_MULTIHOP;
    ERRNO_NAMETOOLONG = __WASI_ERRNO_NAMETOOLONG;
    ERRNO_NETDOWN = __WASI_ERRNO_NETDOWN;
    ERRNO_NETRESET = __WASI_ERRNO_NETRESET;
    ERRNO_NETUNREACH = __WASI_ERRNO_NETUNREACH;
    ERRNO_NFILE = __WASI_ERRNO_NFILE;
    ERRNO_NOBUFS = __WASI_ERRNO_NOBUFS;
    ERRNO_NODEV = __WASI_ERRNO_NODEV;
    ERRNO_NOENT = __WASI_ERRNO_NOENT;
    ERRNO_NOEXEC = __WASI_ERRNO_NOEXEC;
    ERRNO_NOLCK = __WASI_ERRNO_NOLCK;
    ERRNO_NOLINK = __WASI_ERRNO_NOLINK;
    ERRNO_NOMEM = __WASI_ERRNO_NOMEM;
    ERRNO_NOMSG = __WASI_ERRNO_NOMSG;
    ERRNO_NOPROTOOPT = __WASI_ERRNO_NOPROTOOPT;
    ERRNO_NOSPC = __WASI_ERRNO_NOSPC;
    ERRNO_NOSYS = __WASI_ERRNO_NOSYS;
    ERRNO_NOTCONN = __WASI_ERRNO_NOTCONN;
    ERRNO_NOTDIR = __WASI_ERRNO_NOTDIR;
    ERRNO_NOTEMPTY = __WASI_ERRNO_NOTEMPTY;
    ERRNO_NOTRECOVERABLE = __WASI_ERRNO_NOTRECOVERABLE;
    ERRNO_NOTSOCK = __WASI_ERRNO_NOTSOCK;
    ERRNO_NOTSUP = __WASI_ERRNO_NOTSUP;
    ERRNO_NOTTY = __WASI_ERRNO_NOTTY;
    ERRNO_NXIO = __WASI_ERRNO_NXIO;
    ERRNO_OVERFLOW = __WASI_ERRNO_OVERFLOW;
    ERRNO_OWNERDEAD = __WASI_ERRNO_OWNERDEAD;
    ERRNO_PERM = __WASI_ERRNO_PERM;
    ERRNO_PIPE = __WASI_ERRNO_PIPE;
    ERRNO_PROTO = __WASI_ERRNO_PROTO;
    ERRNO_PROTONOSUPPORT = __WASI_ERRNO_PROTONOSUPPORT;
    ERRNO_PROTOTYPE = __WASI_ERRNO_PROTOTYPE;
    ERRNO_RANGE = __WASI_ERRNO_RANGE;
    ERRNO_ROFS = __WASI_ERRNO_ROFS;
    ERRNO_SPIPE = __WASI_ERRNO_SPIPE;
    ERRNO_SRCH = __WASI_ERRNO_SRCH;
    ERRNO_STALE = __WASI_ERRNO_STALE;
    ERRNO_TIMEDOUT = __WASI_ERRNO_TIMEDOUT;
    ERRNO_TXTBSY = __WASI_ERRNO_TXTBSY;
    ERRNO_XDEV = __WASI_ERRNO_XDEV;
    ERRNO_NOTCAPABLE = __WASI_ERRNO_NOTCAPABLE;
}

pub const EVENTRWFLAGS_FD_READWRITE_HANGUP: EventRwFlags = __WASI_EVENTRWFLAGS_FD_READWRITE_HANGUP;
pub const EVENTTYPE_CLOCK: EventType = __WASI_EVENTTYPE_CLOCK;
pub const EVENTTYPE_FD_READ: EventType = __WASI_EVENTTYPE_FD_READ;
pub const EVENTTYPE_FD_WRITE: EventType = __WASI_EVENTTYPE_FD_WRITE;
pub const FDFLAGS_APPEND: FdFlags = __WASI_FDFLAGS_APPEND;
pub const FDFLAGS_DSYNC: FdFlags = __WASI_FDFLAGS_DSYNC;
pub const FDFLAGS_NONBLOCK: FdFlags = __WASI_FDFLAGS_NONBLOCK;
pub const FDFLAGS_RSYNC: FdFlags = __WASI_FDFLAGS_RSYNC;
pub const FDFLAGS_SYNC: FdFlags = __WASI_FDFLAGS_SYNC;
pub const FILETYPE_UNKNOWN: FileType = __WASI_FILETYPE_UNKNOWN;
pub const FILETYPE_BLOCK_DEVICE: FileType = __WASI_FILETYPE_BLOCK_DEVICE;
pub const FILETYPE_CHARACTER_DEVICE: FileType = __WASI_FILETYPE_CHARACTER_DEVICE;
pub const FILETYPE_DIRECTORY: FileType = __WASI_FILETYPE_DIRECTORY;
pub const FILETYPE_REGULAR_FILE: FileType = __WASI_FILETYPE_REGULAR_FILE;
pub const FILETYPE_SOCKET_DGRAM: FileType = __WASI_FILETYPE_SOCKET_DGRAM;
pub const FILETYPE_SOCKET_STREAM: FileType = __WASI_FILETYPE_SOCKET_STREAM;
pub const FILETYPE_SYMBOLIC_LINK: FileType = __WASI_FILETYPE_SYMBOLIC_LINK;
pub const FSTFLAGS_ATIM: FstFlags = __WASI_FSTFLAGS_ATIM;
pub const FSTFLAGS_ATIM_NOW: FstFlags = __WASI_FSTFLAGS_ATIM_NOW;
pub const FSTFLAGS_MTIM: FstFlags = __WASI_FSTFLAGS_MTIM;
pub const FSTFLAGS_MTIM_NOW: FstFlags = __WASI_FSTFLAGS_MTIM_NOW;
pub const LOOKUPFLAGS_SYMLINK_FOLLOW: LookupFlags = __WASI_LOOKUPFLAGS_SYMLINK_FOLLOW;
pub const OFLAGS_CREAT: OFlags = __WASI_OFLAGS_CREAT;
pub const OFLAGS_DIRECTORY: OFlags = __WASI_OFLAGS_DIRECTORY;
pub const OFLAGS_EXCL: OFlags = __WASI_OFLAGS_EXCL;
pub const OFLAGS_TRUNC: OFlags = __WASI_OFLAGS_TRUNC;
pub const PREOPENTYPE_DIR: PreopenType = __WASI_PREOPENTYPE_DIR;
pub const RIFLAGS_RECV_PEEK: RiFlags = __WASI_RIFLAGS_RECV_PEEK;
pub const RIFLAGS_RECV_WAITALL: RiFlags = __WASI_RIFLAGS_RECV_WAITALL;
pub const RIGHTS_FD_DATASYNC: Rights = __WASI_RIGHTS_FD_DATASYNC;
pub const RIGHTS_FD_READ: Rights = __WASI_RIGHTS_FD_READ;
pub const RIGHTS_FD_SEEK: Rights = __WASI_RIGHTS_FD_SEEK;
pub const RIGHTS_FD_FDSTAT_SET_FLAGS: Rights = __WASI_RIGHTS_FD_FDSTAT_SET_FLAGS;
pub const RIGHTS_FD_SYNC: Rights = __WASI_RIGHTS_FD_SYNC;
pub const RIGHTS_FD_TELL: Rights = __WASI_RIGHTS_FD_TELL;
pub const RIGHTS_FD_WRITE: Rights = __WASI_RIGHTS_FD_WRITE;
pub const RIGHTS_FD_ADVISE: Rights = __WASI_RIGHTS_FD_ADVISE;
pub const RIGHTS_FD_ALLOCATE: Rights = __WASI_RIGHTS_FD_ALLOCATE;
pub const RIGHTS_PATH_CREATE_DIRECTORY: Rights = __WASI_RIGHTS_PATH_CREATE_DIRECTORY;
pub const RIGHTS_PATH_CREATE_FILE: Rights = __WASI_RIGHTS_PATH_CREATE_FILE;
pub const RIGHTS_PATH_LINK_SOURCE: Rights = __WASI_RIGHTS_PATH_LINK_SOURCE;
pub const RIGHTS_PATH_LINK_TARGET: Rights = __WASI_RIGHTS_PATH_LINK_TARGET;
pub const RIGHTS_PATH_OPEN: Rights = __WASI_RIGHTS_PATH_OPEN;
pub const RIGHTS_FD_READDIR: Rights = __WASI_RIGHTS_FD_READDIR;
pub const RIGHTS_PATH_READLINK: Rights = __WASI_RIGHTS_PATH_READLINK;
pub const RIGHTS_PATH_RENAME_SOURCE: Rights = __WASI_RIGHTS_PATH_RENAME_SOURCE;
pub const RIGHTS_PATH_RENAME_TARGET: Rights = __WASI_RIGHTS_PATH_RENAME_TARGET;
pub const RIGHTS_PATH_FILESTAT_GET: Rights = __WASI_RIGHTS_PATH_FILESTAT_GET;
pub const RIGHTS_PATH_FILESTAT_SET_SIZE: Rights = __WASI_RIGHTS_PATH_FILESTAT_SET_SIZE;
pub const RIGHTS_PATH_FILESTAT_SET_TIMES: Rights = __WASI_RIGHTS_PATH_FILESTAT_SET_TIMES;
pub const RIGHTS_FD_FILESTAT_GET: Rights = __WASI_RIGHTS_FD_FILESTAT_GET;
pub const RIGHTS_FD_FILESTAT_SET_SIZE: Rights = __WASI_RIGHTS_FD_FILESTAT_SET_SIZE;
pub const RIGHTS_FD_FILESTAT_SET_TIMES: Rights = __WASI_RIGHTS_FD_FILESTAT_SET_TIMES;
pub const RIGHTS_PATH_SYMLINK: Rights = __WASI_RIGHTS_PATH_SYMLINK;
pub const RIGHTS_PATH_REMOVE_DIRECTORY: Rights = __WASI_RIGHTS_PATH_REMOVE_DIRECTORY;
pub const RIGHTS_PATH_UNLINK_FILE: Rights = __WASI_RIGHTS_PATH_UNLINK_FILE;
pub const RIGHTS_POLL_FD_READWRITE: Rights = __WASI_RIGHTS_POLL_FD_READWRITE;
pub const RIGHTS_SOCK_SHUTDOWN: Rights = __WASI_RIGHTS_SOCK_SHUTDOWN;
pub const ROFLAGS_RECV_DATA_TRUNCATED: RoFlags = __WASI_ROFLAGS_RECV_DATA_TRUNCATED;
pub const SDFLAGS_RD: SdFlags = __WASI_SDFLAGS_RD;
pub const SDFLAGS_WR: SdFlags = __WASI_SDFLAGS_WR;
pub const SIGNAL_HUP: Signal = __WASI_SIGNAL_HUP;
pub const SIGNAL_INT: Signal = __WASI_SIGNAL_INT;
pub const SIGNAL_QUIT: Signal = __WASI_SIGNAL_QUIT;
pub const SIGNAL_ILL: Signal = __WASI_SIGNAL_ILL;
pub const SIGNAL_TRAP: Signal = __WASI_SIGNAL_TRAP;
pub const SIGNAL_ABRT: Signal = __WASI_SIGNAL_ABRT;
pub const SIGNAL_BUS: Signal = __WASI_SIGNAL_BUS;
pub const SIGNAL_FPE: Signal = __WASI_SIGNAL_FPE;
pub const SIGNAL_KILL: Signal = __WASI_SIGNAL_KILL;
pub const SIGNAL_USR1: Signal = __WASI_SIGNAL_USR1;
pub const SIGNAL_SEGV: Signal = __WASI_SIGNAL_SEGV;
pub const SIGNAL_USR2: Signal = __WASI_SIGNAL_USR2;
pub const SIGNAL_PIPE: Signal = __WASI_SIGNAL_PIPE;
pub const SIGNAL_ALRM: Signal = __WASI_SIGNAL_ALRM;
pub const SIGNAL_TERM: Signal = __WASI_SIGNAL_TERM;
pub const SIGNAL_CHLD: Signal = __WASI_SIGNAL_CHLD;
pub const SIGNAL_CONT: Signal = __WASI_SIGNAL_CONT;
pub const SIGNAL_STOP: Signal = __WASI_SIGNAL_STOP;
pub const SIGNAL_TSTP: Signal = __WASI_SIGNAL_TSTP;
pub const SIGNAL_TTIN: Signal = __WASI_SIGNAL_TTIN;
pub const SIGNAL_TTOU: Signal = __WASI_SIGNAL_TTOU;
pub const SIGNAL_URG: Signal = __WASI_SIGNAL_URG;
pub const SIGNAL_XCPU: Signal = __WASI_SIGNAL_XCPU;
pub const SIGNAL_XFSZ: Signal = __WASI_SIGNAL_XFSZ;
pub const SIGNAL_VTALRM: Signal = __WASI_SIGNAL_VTALRM;
pub const SIGNAL_PROF: Signal = __WASI_SIGNAL_PROF;
pub const SIGNAL_WINCH: Signal = __WASI_SIGNAL_WINCH;
pub const SIGNAL_POLL: Signal = __WASI_SIGNAL_POLL;
pub const SIGNAL_PWR: Signal = __WASI_SIGNAL_PWR;
pub const SIGNAL_SYS: Signal = __WASI_SIGNAL_SYS;
pub const SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME: SubclockFlags =
    __WASI_SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME;
pub const WHENCE_CUR: Whence = __WASI_WHENCE_CUR;
pub const WHENCE_END: Whence = __WASI_WHENCE_END;
pub const WHENCE_SET: Whence = __WASI_WHENCE_SET;

macro_rules! wrap0 {
    {$f:expr} => {
        if let Some(code) = NonZeroU16::new($f) {
            Err(code)
        } else {
            Ok(())
        }
    };
}

macro_rules! wrap {
    {$f:ident($($args:expr),* $(,)?)} => {
        let mut t = MaybeUninit::uninit();
        let r = $f($($args,)* t.as_mut_ptr());
        if let Some(code) = NonZeroU16::new(r) {
            Err(code)
        } else {
            Ok(t.assume_init())
        }
    };
}

#[inline]
pub fn clock_res_get(clock_id: ClockId) -> Result<Timestamp, Error> {
    unsafe {
        wrap! { __wasi_clock_res_get(clock_id) }
    }
}

#[inline]
pub fn clock_time_get(clock_id: ClockId, precision: Timestamp) -> Result<Timestamp, Error> {
    unsafe {
        wrap! { __wasi_clock_time_get(clock_id, precision) }
    }
}

#[inline]
pub unsafe fn fd_pread(fd: Fd, iovs: &[IoVec], offset: FileSize) -> Result<usize, Error> {
    wrap! { __wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset) }
}

#[inline]
pub unsafe fn fd_pwrite(fd: Fd, iovs: &[CIoVec], offset: FileSize) -> Result<usize, Error> {
    wrap! { __wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset) }
}

#[inline]
pub fn random_get(buf: &mut [u8]) -> Result<(), Error> {
    unsafe {
        wrap0! { __wasi_random_get(buf.as_mut_ptr(), buf.len()) }
    }
}

#[inline]
pub unsafe fn fd_close(fd: Fd) -> Result<(), Error> {
    wrap0! { __wasi_fd_close(fd) }
}

#[inline]
pub unsafe fn fd_datasync(fd: Fd) -> Result<(), Error> {
    wrap0! { __wasi_fd_datasync(fd) }
}

#[inline]
pub unsafe fn fd_read(fd: Fd, iovs: &[IoVec]) -> Result<usize, Error> {
    wrap! { __wasi_fd_read(fd, iovs.as_ptr(), iovs.len()) }
}

#[inline]
pub unsafe fn fd_renumber(from: Fd, to: Fd) -> Result<(), Error> {
    wrap0! { __wasi_fd_renumber(from, to) }
}

#[inline]
pub unsafe fn fd_seek(fd: Fd, offset: FileDelta, whence: Whence) -> Result<FileSize, Error> {
    wrap! { __wasi_fd_seek(fd, offset, whence) }
}

#[inline]
pub unsafe fn fd_tell(fd: Fd) -> Result<FileSize, Error> {
    wrap! { __wasi_fd_tell(fd) }
}

#[inline]
pub unsafe fn fd_fdstat_get(fd: Fd) -> Result<FdStat, Error> {
    wrap! { __wasi_fd_fdstat_get(fd) }
}

#[inline]
pub unsafe fn fd_fdstat_set_flags(fd: Fd, flags: FdFlags) -> Result<(), Error> {
    wrap0! { __wasi_fd_fdstat_set_flags(fd, flags) }
}

#[inline]
pub unsafe fn fd_fdstat_set_rights(
    fd: Fd,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
) -> Result<(), Error> {
    wrap0! { __wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting) }
}

#[inline]
pub unsafe fn fd_sync(fd: Fd) -> Result<(), Error> {
    wrap0! { __wasi_fd_sync(fd) }
}

#[inline]
pub unsafe fn fd_write(fd: Fd, iovs: &[CIoVec]) -> Result<usize, Error> {
    wrap! { __wasi_fd_write(fd, iovs.as_ptr(), iovs.len()) }
}

#[inline]
pub unsafe fn fd_advise(
    fd: Fd,
    offset: FileSize,
    len: FileSize,
    advice: Advice,
) -> Result<(), Error> {
    wrap0! { __wasi_fd_advise(fd, offset, len, advice) }
}

#[inline]
pub unsafe fn fd_allocate(fd: Fd, offset: FileSize, len: FileSize) -> Result<(), Error> {
    wrap0! { __wasi_fd_allocate(fd, offset, len) }
}

#[inline]
pub unsafe fn path_create_directory(fd: Fd, path: &[u8]) -> Result<(), Error> {
    wrap0! { __wasi_path_create_directory(fd, path.as_ptr(), path.len()) }
}

#[inline]
pub unsafe fn path_link(
    old_fd: Fd,
    old_flags: LookupFlags,
    old_path: &[u8],
    new_fd: Fd,
    new_path: &[u8],
) -> Result<(), Error> {
    wrap0! {
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

#[inline]
pub unsafe fn path_open(
    dirfd: Fd,
    dirflags: LookupFlags,
    path: &[u8],
    oflags: OFlags,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fs_flags: FdFlags,
) -> Result<Fd, Error> {
    wrap! {
        __wasi_path_open(
            dirfd,
            dirflags,
            path.as_ptr(),
            path.len(),
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
        )
    }
}

#[inline]
pub unsafe fn fd_readdir(fd: Fd, buf: &mut [u8], cookie: DirCookie) -> Result<usize, Error> {
    wrap! { __wasi_fd_readdir(fd, buf.as_mut_ptr(), buf.len(), cookie) }
}

#[inline]
pub unsafe fn path_readlink(fd: Fd, path: &[u8], buf: &mut [u8]) -> Result<usize, Error> {
    let ptr = buf.as_mut_ptr();
    wrap! {
        __wasi_path_readlink(fd, path.as_ptr(), path.len(), ptr, buf.len())
    }
}

#[inline]
pub unsafe fn path_rename(
    old_fd: Fd,
    old_path: &[u8],
    new_fd: Fd,
    new_path: &[u8],
) -> Result<(), Error> {
    wrap0! {
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

#[inline]
pub unsafe fn fd_filestat_get(fd: Fd) -> Result<FileStat, Error> {
    wrap! { __wasi_fd_filestat_get(fd) }
}

#[inline]
pub unsafe fn fd_filestat_set_times(
    fd: Fd,
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: FstFlags,
) -> Result<(), Error> {
    wrap0! { __wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fstflags) }
}

#[inline]
pub unsafe fn fd_filestat_set_size(fd: Fd, st_size: FileSize) -> Result<(), Error> {
    wrap0! { __wasi_fd_filestat_set_size(fd, st_size) }
}

#[inline]
pub unsafe fn path_filestat_get(
    fd: Fd,
    flags: LookupFlags,
    path: &[u8],
) -> Result<FileStat, Error> {
    wrap! {
        __wasi_path_filestat_get(fd, flags, path.as_ptr(), path.len())
    }
}

#[inline]
pub unsafe fn path_filestat_set_times(
    fd: Fd,
    flags: LookupFlags,
    path: &[u8],
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: FstFlags,
) -> Result<(), Error> {
    wrap0! {
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

#[inline]
pub unsafe fn path_symlink(old_path: &[u8], fd: Fd, new_path: &[u8]) -> Result<(), Error> {
    wrap0! {
        __wasi_path_symlink(
            old_path.as_ptr(),
            old_path.len(),
            fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

#[inline]
pub unsafe fn path_unlink_file(fd: Fd, path: &[u8]) -> Result<(), Error> {
    wrap0! { __wasi_path_unlink_file(fd, path.as_ptr(), path.len()) }
}

#[inline]
pub unsafe fn path_remove_directory(fd: Fd, path: &[u8]) -> Result<(), Error> {
    wrap0! { __wasi_path_remove_directory(fd, path.as_ptr(), path.len()) }
}

#[inline]
pub unsafe fn poll_oneoff(in_: &[Subscription], out: &mut [Event]) -> Result<usize, Error> {
    assert!(out.len() >= in_.len());
    let ptr = out.as_mut_ptr() as *mut __wasi_event_t;
    wrap! {
        __wasi_poll_oneoff(
            in_.as_ptr(),
            ptr,
            in_.len(),
        )
    }
}

#[inline]
pub fn proc_exit(rval: ExitCode) -> ! {
    unsafe { __wasi_proc_exit(rval) }
}

#[inline]
pub unsafe fn sock_recv(
    sock: Fd,
    ri_data: &[IoVec],
    ri_flags: RiFlags,
) -> Result<(usize, RoFlags), Error> {
    let mut ro_datalen = MaybeUninit::<usize>::uninit();
    let mut ro_flags = MaybeUninit::<RoFlags>::uninit();
    let r = __wasi_sock_recv(
        sock,
        ri_data.as_ptr(),
        ri_data.len(),
        ri_flags,
        ro_datalen.as_mut_ptr(),
        ro_flags.as_mut_ptr(),
    );
    if let Some(code) = NonZeroU16::new(r) {
        Err(code)
    } else {
        Ok((ro_datalen.assume_init(), ro_flags.assume_init()))
    }
}

#[inline]
pub unsafe fn sock_send(sock: Fd, si_data: &[CIoVec], si_flags: SiFlags) -> Result<usize, Error> {
    wrap! { __wasi_sock_send(sock, si_data.as_ptr(), si_data.len(), si_flags) }
}

#[inline]
pub unsafe fn sock_shutdown(sock: Fd, how: SdFlags) -> Result<(), Error> {
    wrap0! { __wasi_sock_shutdown(sock, how) }
}

#[inline]
pub fn sched_yield() -> Result<(), Error> {
    unsafe {
        wrap0! { __wasi_sched_yield() }
    }
}

#[inline]
pub unsafe fn fd_prestat_get(fd: Fd) -> Result<Prestat, Error> {
    wrap! { __wasi_fd_prestat_get(fd) }
}

#[inline]
pub unsafe fn fd_prestat_dir_name(fd: Fd, path: &mut [u8]) -> Result<(), Error> {
    wrap0! { __wasi_fd_prestat_dir_name(fd, path.as_mut_ptr(), path.len()) }
}

#[derive(Copy, Clone)]
pub struct ArgsSizes {
    count: usize,
    buf_len: usize,
}

impl ArgsSizes {
    #[inline]
    pub fn get_count(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn get_buf_len(&self) -> usize {
        self.buf_len
    }
}

#[inline]
pub fn args_sizes_get() -> Result<ArgsSizes, Error> {
    let mut res = ArgsSizes {
        count: 0,
        buf_len: 0,
    };
    let code = unsafe { __wasi_args_sizes_get(&mut res.count, &mut res.buf_len) };
    if let Some(err) = NonZeroU16::new(code) {
        return Err(err);
    }
    Ok(res)
}

#[cfg(feature = "alloc")]
#[inline]
pub fn args_get(ars: ArgsSizes, mut process_arg: impl FnMut(&[u8])) -> Result<(), Error> {
    use alloc::vec;

    // TODO: remove allocations after stabilization of unsized rvalues, see:
    // https://github.com/rust-lang/rust/issues/48055
    let mut arg_ptrs = vec![core::ptr::null_mut::<u8>(); ars.count];
    let mut arg_buf = vec![0u8; ars.buf_len];
    let ret = unsafe { __wasi_args_get(arg_ptrs.as_mut_ptr(), arg_buf.as_mut_ptr()) };
    if let Some(err) = NonZeroU16::new(ret) {
        return Err(err);
    }

    for ptr in arg_ptrs {
        for n in 0.. {
            unsafe {
                if *ptr.add(n) == 0 {
                    let slice = core::slice::from_raw_parts(ptr, n);
                    process_arg(slice);
                    break;
                }
            }
        }
    }

    Ok(())
}

#[derive(Copy, Clone)]
pub struct EnvironSizes {
    count: usize,
    buf_len: usize,
}

impl EnvironSizes {
    #[inline]
    pub fn get_count(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn get_buf_len(&self) -> usize {
        self.buf_len
    }
}

#[inline]
pub fn environ_sizes_get() -> Result<EnvironSizes, Error> {
    let mut res = EnvironSizes {
        count: 0,
        buf_len: 0,
    };
    let code = unsafe { __wasi_environ_sizes_get(&mut res.count, &mut res.buf_len) };
    if let Some(err) = NonZeroU16::new(code) {
        return Err(err);
    }
    Ok(res)
}

#[cfg(feature = "alloc")]
#[inline]
pub fn environ_get(
    es: EnvironSizes,
    mut process_env: impl FnMut(&[u8], &[u8]),
) -> Result<(), Error> {
    use alloc::vec;

    // TODO: remove allocations after stabilization of unsized rvalues, see:
    // https://github.com/rust-lang/rust/issues/48055
    let mut env_ptrs = vec![core::ptr::null_mut::<u8>(); es.count];
    let mut env_buf = vec![0u8; es.buf_len];
    let ret = unsafe { __wasi_environ_get(env_ptrs.as_mut_ptr(), env_buf.as_mut_ptr()) };
    if let Some(err) = NonZeroU16::new(ret) {
        return Err(err);
    }

    for ptr in env_ptrs {
        let mut key: &[u8] = &[];
        for n in 0.. {
            unsafe {
                match *ptr.add(n) {
                    0 => {
                        let val = core::slice::from_raw_parts(ptr, n);
                        process_env(key, val);
                        break;
                    }
                    b'=' if key.is_empty() => {
                        key = core::slice::from_raw_parts(ptr, n);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

pub fn error_str(err: Error) -> Option<&'static str> {
    let desc = match err {
        ERRNO_2BIG => "Argument list too long",
        ERRNO_ACCES => "Permission denied",
        ERRNO_ADDRINUSE => "Address in use",
        ERRNO_ADDRNOTAVAIL => "Address not available",
        ERRNO_AFNOSUPPORT => "Address family not supported by protocol",
        ERRNO_AGAIN => "Resource temporarily unavailable",
        ERRNO_ALREADY => "Operation already in progress",
        ERRNO_BADF => "Bad file descriptor",
        ERRNO_BADMSG => "Bad message",
        ERRNO_BUSY => "Resource busy",
        ERRNO_CANCELED => "Operation canceled",
        ERRNO_CHILD => "No child process",
        ERRNO_CONNABORTED => "Connection aborted",
        ERRNO_CONNREFUSED => "Connection refused",
        ERRNO_CONNRESET => "Connection reset by peer",
        ERRNO_DEADLK => "Resource deadlock would occur",
        ERRNO_DESTADDRREQ => "Destination address required",
        ERRNO_DOM => "Domain error",
        ERRNO_DQUOT => "Quota exceeded",
        ERRNO_EXIST => "File exists",
        ERRNO_FAULT => "Bad address",
        ERRNO_FBIG => "File too large",
        ERRNO_HOSTUNREACH => "Host is unreachable",
        ERRNO_IDRM => "Identifier removed",
        ERRNO_ILSEQ => "Illegal byte sequence",
        ERRNO_INPROGRESS => "Operation in progress",
        ERRNO_INTR => "Interrupted system call",
        ERRNO_INVAL => "Invalid argument",
        ERRNO_IO => "Remote I/O error",
        ERRNO_ISCONN => "Socket is connected",
        ERRNO_ISDIR => "Is a directory",
        ERRNO_LOOP => "Symbolic link loop",
        ERRNO_MFILE => "No file descriptors available",
        ERRNO_MLINK => "Too many links",
        ERRNO_MSGSIZE => "Message too large",
        ERRNO_MULTIHOP => "Multihop attempted",
        ERRNO_NAMETOOLONG => "Filename too long",
        ERRNO_NETDOWN => "Network is down",
        ERRNO_NETRESET => "Connection reset by network",
        ERRNO_NETUNREACH => "Network unreachable",
        ERRNO_NFILE => "Too many open files in system",
        ERRNO_NOBUFS => "No buffer space available",
        ERRNO_NODEV => "No such device",
        ERRNO_NOENT => "No such file or directory",
        ERRNO_NOEXEC => "Exec format error",
        ERRNO_NOLCK => "No locks available",
        ERRNO_NOLINK => "Link has been severed",
        ERRNO_NOMEM => "Out of memory",
        ERRNO_NOMSG => "No message of desired type",
        ERRNO_NOPROTOOPT => "Protocol not available",
        ERRNO_NOSPC => "No space left on device",
        ERRNO_NOSYS => "Function not implemented",
        ERRNO_NOTCONN => "Socket not connected",
        ERRNO_NOTDIR => "Not a directory",
        ERRNO_NOTEMPTY => "Directory not empty",
        ERRNO_NOTRECOVERABLE => "State not recoverable",
        ERRNO_NOTSOCK => "Not a socket",
        ERRNO_NOTSUP => "Not supported",
        ERRNO_NOTTY => "Not a tty",
        ERRNO_NXIO => "No such device or address",
        ERRNO_OVERFLOW => "Value too large for data type",
        ERRNO_OWNERDEAD => "Previous owner died",
        ERRNO_PERM => "Operation not permitted",
        ERRNO_PIPE => "Broken pipe",
        ERRNO_PROTO => "Protocol error",
        ERRNO_PROTONOSUPPORT => "Protocol not supported",
        ERRNO_PROTOTYPE => "Protocol wrong type for socket",
        ERRNO_RANGE => "Result not representable",
        ERRNO_ROFS => "Read-only file system",
        ERRNO_SPIPE => "Invalid seek",
        ERRNO_SRCH => "No such process",
        ERRNO_STALE => "Stale file handle",
        ERRNO_TIMEDOUT => "Operation timed out",
        ERRNO_TXTBSY => "Text file busy",
        ERRNO_XDEV => "Cross-device link",
        ERRNO_NOTCAPABLE => "Capabilities insufficient",
        _ => return None,
    };
    Some(desc)
}
