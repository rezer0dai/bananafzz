extern crate core;
use self::core::state::id::StateTableId;
use self::core::generator::leaf::IArgLeaf;

extern crate api;
use self::api::leafs::fd_leaf::{
    FdHolder,
    RndFd
};

use super::super::common::table::*;

extern crate generic;

pub fn rnd_fd(sid: StateTableId) -> Box<dyn IArgLeaf> {
    match StateIds::from(sid.clone()){
        _ => Box::new(FdHolder::new(FD_SIZE, vec![ Box::new(RndFd::new(sid, FD_SIZE)), ])),
    }
}

//better to use enums, only one dead_code
//problem with repeated values, though we can group them which is anyway better solution
#[allow(dead_code)]
pub const ADDR_COMPAT_LAYOUT: u64 = 2097152;
#[allow(dead_code)]
pub const ADDR_LIMIT_32BIT: u64 = 8388608;
#[allow(dead_code)]
pub const ADDR_LIMIT_3GB: u64 = 134217728;
#[allow(dead_code)]
pub const ADDR_NO_RANDOMIZE: u64 = 262144;
#[allow(dead_code)]
pub const ARCH_GET_FS: u64 = 4099;
#[allow(dead_code)]
pub const ARCH_GET_GS: u64 = 4100;
#[allow(dead_code)]
pub const ARCH_SET_FS: u64 = 4098;
#[allow(dead_code)]
pub const ARCH_SET_GS: u64 = 4097;
#[allow(dead_code)]
pub const AT_EMPTY_PATH: u64 = 4096;
#[allow(dead_code)]
pub const AT_FDCWD: u64 = 18446744073709551516;
#[allow(dead_code)]
pub const AT_NO_AUTOMOUNT: u64 = 2048;
#[allow(dead_code)]
pub const AT_REMOVEDIR: u64 = 512;
#[allow(dead_code)]
pub const AT_STATX_DONT_SYNC: u64 = 16384;
#[allow(dead_code)]
pub const AT_STATX_FORCE_SYNC: u64 = 8192;
#[allow(dead_code)]
pub const AT_STATX_SYNC_AS_STAT: u64 = 0;
#[allow(dead_code)]
pub const AT_STATX_SYNC_TYPE: u64 = 24576;
#[allow(dead_code)]
pub const AT_SYMLINK_FOLLOW: u64 = 1024;
#[allow(dead_code)]
pub const AT_SYMLINK_NOFOLLOW: u64 = 256;
#[allow(dead_code)]
pub const CLOCK_BOOTTIME: u64 = 7;
#[allow(dead_code)]
pub const CLOCK_MONOTONIC: u64 = 1;
#[allow(dead_code)]
pub const CLOCK_MONOTONIC_COARSE: u64 = 6;
#[allow(dead_code)]
pub const CLOCK_MONOTONIC_RAW: u64 = 4;
#[allow(dead_code)]
pub const CLOCK_PROCESS_CPUTIME_ID: u64 = 2;
#[allow(dead_code)]
pub const CLOCK_REALTIME: u64 = 0;
#[allow(dead_code)]
pub const CLOCK_REALTIME_COARSE: u64 = 5;
#[allow(dead_code)]
pub const CLOCK_THREAD_CPUTIME_ID: u64 = 3;
#[allow(dead_code)]
pub const CLONE_CHILD_CLEARTID: u64 = 2097152;
#[allow(dead_code)]
pub const CLONE_CHILD_SETTID: u64 = 16777216;
#[allow(dead_code)]
pub const CLONE_FILES: u64 = 1024;
#[allow(dead_code)]
pub const CLONE_FS: u64 = 512;
#[allow(dead_code)]
pub const CLONE_IO: u64 = 2147483648;
#[allow(dead_code)]
pub const CLONE_NEWCGROUP: u64 = 33554432;
#[allow(dead_code)]
pub const CLONE_NEWIPC: u64 = 134217728;
#[allow(dead_code)]
pub const CLONE_NEWNET: u64 = 1073741824;
#[allow(dead_code)]
pub const CLONE_NEWNS: u64 = 131072;
#[allow(dead_code)]
pub const CLONE_NEWPID: u64 = 536870912;
#[allow(dead_code)]
pub const CLONE_NEWUSER: u64 = 268435456;
#[allow(dead_code)]
pub const CLONE_NEWUTS: u64 = 67108864;
#[allow(dead_code)]
pub const CLONE_PARENT: u64 = 32768;
#[allow(dead_code)]
pub const CLONE_PARENT_SETTID: u64 = 1048576;
#[allow(dead_code)]
pub const CLONE_PTRACE: u64 = 8192;
#[allow(dead_code)]
pub const CLONE_SETTLS: u64 = 524288;
#[allow(dead_code)]
pub const CLONE_SIGHAND: u64 = 2048;
#[allow(dead_code)]
pub const CLONE_SYSVSEM: u64 = 262144;
#[allow(dead_code)]
pub const CLONE_THREAD: u64 = 65536;
#[allow(dead_code)]
pub const CLONE_UNTRACED: u64 = 8388608;
#[allow(dead_code)]
pub const CLONE_VFORK: u64 = 16384;
#[allow(dead_code)]
pub const CLONE_VM: u64 = 256;
#[allow(dead_code)]
pub const DN_ACCESS: u64 = 1;
#[allow(dead_code)]
pub const DN_ATTRIB: u64 = 32;
#[allow(dead_code)]
pub const DN_CREATE: u64 = 4;
#[allow(dead_code)]
pub const DN_DELETE: u64 = 8;
#[allow(dead_code)]
pub const DN_MODIFY: u64 = 2;
#[allow(dead_code)]
pub const DN_MULTISHOT: u64 = 2147483648;
#[allow(dead_code)]
pub const DN_RENAME: u64 = 16;
#[allow(dead_code)]
pub const EFD_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const EFD_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const EFD_SEMAPHORE: u64 = 1;
#[allow(dead_code)]
pub const EPOLLET: u64 = 2147483648;
#[allow(dead_code)]
pub const EPOLLEXCLUSIVE: u64 = 268435456;
#[allow(dead_code)]
pub const EPOLLONESHOT: u64 = 1073741824;
#[allow(dead_code)]
pub const EPOLLWAKEUP: u64 = 536870912;
#[allow(dead_code)]
pub const EPOLL_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const EPOLL_CTL_ADD: u64 = 1;
#[allow(dead_code)]
pub const EPOLL_CTL_DEL: u64 = 2;
#[allow(dead_code)]
pub const EPOLL_CTL_MOD: u64 = 3;
#[allow(dead_code)]
pub const FALLOC_FL_KEEP_SIZE: u64 = 1;
#[allow(dead_code)]
pub const FALLOC_FL_PUNCH_HOLE: u64 = 2;
#[allow(dead_code)]
pub const FAN_ACCESS: u64 = 1;
#[allow(dead_code)]
pub const FAN_ACCESS_PERM: u64 = 131072;
#[allow(dead_code)]
pub const FAN_CLASS_CONTENT: u64 = 4;
#[allow(dead_code)]
pub const FAN_CLASS_NOTIF: u64 = 0;
#[allow(dead_code)]
pub const FAN_CLASS_PRE_CONTENT: u64 = 8;
#[allow(dead_code)]
pub const FAN_CLOEXEC: u64 = 1;
#[allow(dead_code)]
pub const FAN_CLOSE_NOWRITE: u64 = 16;
#[allow(dead_code)]
pub const FAN_CLOSE_WRITE: u64 = 8;
#[allow(dead_code)]
pub const FAN_EVENT_ON_CHILD: u64 = 134217728;
#[allow(dead_code)]
pub const FAN_MARK_ADD: u64 = 1;
#[allow(dead_code)]
pub const FAN_MARK_DONT_FOLLOW: u64 = 4;
#[allow(dead_code)]
pub const FAN_MARK_FLUSH: u64 = 128;
#[allow(dead_code)]
pub const FAN_MARK_IGNORED_MASK: u64 = 32;
#[allow(dead_code)]
pub const FAN_MARK_IGNORED_SURV_MODIFY: u64 = 64;
#[allow(dead_code)]
pub const FAN_MARK_MOUNT: u64 = 16;
#[allow(dead_code)]
pub const FAN_MARK_ONLYDIR: u64 = 8;
#[allow(dead_code)]
pub const FAN_MARK_REMOVE: u64 = 2;
#[allow(dead_code)]
pub const FAN_MODIFY: u64 = 2;
#[allow(dead_code)]
pub const FAN_NONBLOCK: u64 = 2;
#[allow(dead_code)]
pub const FAN_ONDIR: u64 = 1073741824;
#[allow(dead_code)]
pub const FAN_OPEN: u64 = 32;
#[allow(dead_code)]
pub const FAN_OPEN_PERM: u64 = 65536;
#[allow(dead_code)]
pub const FAN_UNLIMITED_MARKS: u64 = 32;
#[allow(dead_code)]
pub const FAN_UNLIMITED_QUEUE: u64 = 16;
#[allow(dead_code)]
pub const FASYNC: u64 = 8192;
#[allow(dead_code)]
pub const FD_CLOEXEC: u64 = 1;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_DATA_ENCRYPTED: u64 = 128;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_DATA_INLINE: u64 = 512;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_DATA_TAIL: u64 = 1024;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_DELALLOC: u64 = 4;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_ENCODED: u64 = 8;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_LAST: u64 = 1;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_MERGED: u64 = 4096;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_NOT_ALIGNED: u64 = 256;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_SHARED: u64 = 8192;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_UNKNOWN: u64 = 2;
#[allow(dead_code)]
pub const FIEMAP_EXTENT_UNWRITTEN: u64 = 2048;
#[allow(dead_code)]
pub const FIEMAP_FLAG_CACHE: u64 = 4;
#[allow(dead_code)]
pub const FIEMAP_FLAG_SYNC: u64 = 1;
#[allow(dead_code)]
pub const FIEMAP_FLAG_XATTR: u64 = 2;
#[allow(dead_code)]
pub const FIFREEZE: u64 = 3221510263;
#[allow(dead_code)]
pub const FIGETBSZ: u64 = 2;
#[allow(dead_code)]
pub const FIOASYNC: u64 = 21586;
#[allow(dead_code)]
pub const FIOCLEX: u64 = 21585;
#[allow(dead_code)]
pub const FIONBIO: u64 = 21537;
#[allow(dead_code)]
pub const FIONCLEX: u64 = 21584;
#[allow(dead_code)]
pub const FIOQSIZE: u64 = 21600;
#[allow(dead_code)]
pub const FITHAW: u64 = 3221510264;
#[allow(dead_code)]
pub const FS_IOC_FIEMAP: u64 = 3223348747;
#[allow(dead_code)]
pub const FUTEX_CMP_REQUEUE: u64 = 4;
#[allow(dead_code)]
pub const FUTEX_REQUEUE: u64 = 3;
#[allow(dead_code)]
pub const FUTEX_WAIT: u64 = 0;
#[allow(dead_code)]
pub const FUTEX_WAIT_BITSET: u64 = 9;
#[allow(dead_code)]
pub const FUTEX_WAKE: u64 = 1;
#[allow(dead_code)]
pub const F_ADD_SEALS: u64 = 1033;
#[allow(dead_code)]
pub const F_DUPFD: u64 = 0;
#[allow(dead_code)]
pub const F_DUPFD_CLOEXEC: u64 = 1030;
#[allow(dead_code)]
pub const F_GETFD: u64 = 1;
#[allow(dead_code)]
pub const F_GETFL: u64 = 3;
#[allow(dead_code)]
pub const F_GETLEASE: u64 = 1025;
#[allow(dead_code)]
pub const F_GETLK: u64 = 5;
#[allow(dead_code)]
pub const F_GETOWN: u64 = 9;
#[allow(dead_code)]
pub const F_GETOWN_EX: u64 = 16;
#[allow(dead_code)]
pub const F_GETPIPE_SZ: u64 = 1032;
#[allow(dead_code)]
pub const F_GETSIG: u64 = 11;
#[allow(dead_code)]
pub const F_GET_SEALS: u64 = 1034;
#[allow(dead_code)]
pub const F_OWNER_PGRP: u64 = 2;
#[allow(dead_code)]
pub const F_OWNER_PID: u64 = 1;
#[allow(dead_code)]
pub const F_OWNER_TID: u64 = 0;
#[allow(dead_code)]
pub const F_RDLCK: u64 = 0;
#[allow(dead_code)]
pub const F_SEAL_GROW: u64 = 4;
#[allow(dead_code)]
pub const F_SEAL_SEAL: u64 = 1;
#[allow(dead_code)]
pub const F_SEAL_SHRINK: u64 = 2;
#[allow(dead_code)]
pub const F_SEAL_WRITE: u64 = 8;
#[allow(dead_code)]
pub const F_SETFD: u64 = 2;
#[allow(dead_code)]
pub const F_SETFL: u64 = 4;
#[allow(dead_code)]
pub const F_SETLEASE: u64 = 1024;
#[allow(dead_code)]
pub const F_SETLK: u64 = 6;
#[allow(dead_code)]
pub const F_SETLKW: u64 = 7;
#[allow(dead_code)]
pub const F_SETOWN: u64 = 8;
#[allow(dead_code)]
pub const F_SETOWN_EX: u64 = 15;
#[allow(dead_code)]
pub const F_SETPIPE_SZ: u64 = 1031;
#[allow(dead_code)]
pub const F_SETSIG: u64 = 10;
#[allow(dead_code)]
pub const F_UNLCK: u64 = 2;
#[allow(dead_code)]
pub const F_WRLCK: u64 = 1;
#[allow(dead_code)]
pub const GRND_NONBLOCK: u64 = 1;
#[allow(dead_code)]
pub const GRND_RANDOM: u64 = 2;
#[allow(dead_code)]
pub const IN_ACCESS: u64 = 1;
#[allow(dead_code)]
pub const IN_ATTRIB: u64 = 4;
#[allow(dead_code)]
pub const IN_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const IN_CLOSE_NOWRITE: u64 = 16;
#[allow(dead_code)]
pub const IN_CLOSE_WRITE: u64 = 8;
#[allow(dead_code)]
pub const IN_CREATE: u64 = 256;
#[allow(dead_code)]
pub const IN_DELETE: u64 = 512;
#[allow(dead_code)]
pub const IN_DELETE_SELF: u64 = 1024;
#[allow(dead_code)]
pub const IN_DONT_FOLLOW: u64 = 33554432;
#[allow(dead_code)]
pub const IN_EXCL_UNLINK: u64 = 67108864;
#[allow(dead_code)]
pub const IN_MASK_ADD: u64 = 536870912;
#[allow(dead_code)]
pub const IN_MODIFY: u64 = 2;
#[allow(dead_code)]
pub const IN_MOVED_FROM: u64 = 64;
#[allow(dead_code)]
pub const IN_MOVED_TO: u64 = 128;
#[allow(dead_code)]
pub const IN_MOVE_SELF: u64 = 2048;
#[allow(dead_code)]
pub const IN_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const IN_ONESHOT: u64 = 2147483648;
#[allow(dead_code)]
pub const IN_ONLYDIR: u64 = 16777216;
#[allow(dead_code)]
pub const IN_OPEN: u64 = 32;
#[allow(dead_code)]
pub const IOCB_CMD_FDSYNC: u64 = 3;
#[allow(dead_code)]
pub const IOCB_CMD_FSYNC: u64 = 2;
#[allow(dead_code)]
pub const IOCB_CMD_NOOP: u64 = 6;
#[allow(dead_code)]
pub const IOCB_CMD_PREAD: u64 = 0;
#[allow(dead_code)]
pub const IOCB_CMD_PREADV: u64 = 7;
#[allow(dead_code)]
pub const IOCB_CMD_PWRITE: u64 = 1;
#[allow(dead_code)]
pub const IOCB_CMD_PWRITEV: u64 = 8;
#[allow(dead_code)]
pub const IOCB_FLAG_RESFD: u64 = 1;
#[allow(dead_code)]
pub const IOPRIO_WHO_PGRP: u64 = 2;
#[allow(dead_code)]
pub const IOPRIO_WHO_PROCESS: u64 = 1;
#[allow(dead_code)]
pub const IOPRIO_WHO_USER: u64 = 3;
#[allow(dead_code)]
pub const ITIMER_PROF: u64 = 2;
#[allow(dead_code)]
pub const ITIMER_REAL: u64 = 0;
#[allow(dead_code)]
pub const ITIMER_VIRTUAL: u64 = 1;
#[allow(dead_code)]
pub const KCMP_FILE: u64 = 0;
#[allow(dead_code)]
pub const KCMP_FILES: u64 = 2;
#[allow(dead_code)]
pub const KCMP_FS: u64 = 3;
#[allow(dead_code)]
pub const KCMP_IO: u64 = 5;
#[allow(dead_code)]
pub const KCMP_SIGHAND: u64 = 4;
#[allow(dead_code)]
pub const KCMP_SYSVSEM: u64 = 6;
#[allow(dead_code)]
pub const KCMP_VM: u64 = 1;
#[allow(dead_code)]
pub const KCOV_ENABLE: u64 = 25444;
#[allow(dead_code)]
pub const KCOV_INIT_TRACE: u64 = 2148033281;
#[allow(dead_code)]
pub const KCOV_TRACE_CMP: u64 = 1;
#[allow(dead_code)]
pub const KCOV_TRACE_PC: u64 = 0;
#[allow(dead_code)]
pub const KEXEC_ARCH_386: u64 = 196608;
#[allow(dead_code)]
pub const KEXEC_ARCH_ARM: u64 = 2621440;
#[allow(dead_code)]
pub const KEXEC_ARCH_IA_64: u64 = 3276800;
#[allow(dead_code)]
pub const KEXEC_ARCH_MIPS: u64 = 524288;
#[allow(dead_code)]
pub const KEXEC_ARCH_MIPS_LE: u64 = 655360;
#[allow(dead_code)]
pub const KEXEC_ARCH_PPC: u64 = 1310720;
#[allow(dead_code)]
pub const KEXEC_ARCH_PPC64: u64 = 1376256;
#[allow(dead_code)]
pub const KEXEC_ARCH_S390: u64 = 1441792;
#[allow(dead_code)]
pub const KEXEC_ARCH_SH: u64 = 2752512;
#[allow(dead_code)]
pub const KEXEC_ARCH_X86_64: u64 = 4063232;
#[allow(dead_code)]
pub const KEXEC_ON_CRASH: u64 = 1;
#[allow(dead_code)]
pub const KEXEC_PRESERVE_CONTEXT: u64 = 2;
#[allow(dead_code)]
pub const LOCK_EX: u64 = 2;
#[allow(dead_code)]
pub const LOCK_NB: u64 = 4;
#[allow(dead_code)]
pub const LOCK_SH: u64 = 1;
#[allow(dead_code)]
pub const LOCK_UN: u64 = 8;
#[allow(dead_code)]
pub const MADV_DODUMP: u64 = 17;
#[allow(dead_code)]
pub const MADV_DOFORK: u64 = 11;
#[allow(dead_code)]
pub const MADV_DONTDUMP: u64 = 16;
#[allow(dead_code)]
pub const MADV_DONTFORK: u64 = 10;
#[allow(dead_code)]
pub const MADV_DONTNEED: u64 = 4;
#[allow(dead_code)]
pub const MADV_HUGEPAGE: u64 = 14;
#[allow(dead_code)]
pub const MADV_HWPOISON: u64 = 100;
#[allow(dead_code)]
pub const MADV_MERGEABLE: u64 = 12;
#[allow(dead_code)]
pub const MADV_NOHUGEPAGE: u64 = 15;
#[allow(dead_code)]
pub const MADV_NORMAL: u64 = 0;
#[allow(dead_code)]
pub const MADV_RANDOM: u64 = 1;
#[allow(dead_code)]
pub const MADV_REMOVE: u64 = 9;
#[allow(dead_code)]
pub const MADV_SEQUENTIAL: u64 = 2;
#[allow(dead_code)]
pub const MADV_SOFT_OFFLINE: u64 = 101;
#[allow(dead_code)]
pub const MADV_UNMERGEABLE: u64 = 13;
#[allow(dead_code)]
pub const MADV_WILLNEED: u64 = 3;
#[allow(dead_code)]
pub const MAP_32BIT: u64 = 64;
#[allow(dead_code)]
pub const MAP_ANONYMOUS: u64 = 32;
#[allow(dead_code)]
pub const MAP_DENYWRITE: u64 = 2048;
#[allow(dead_code)]
pub const MAP_EXECUTABLE: u64 = 4096;
#[allow(dead_code)]
pub const MAP_FILE: u64 = 0;
#[allow(dead_code)]
pub const MAP_FIXED: u64 = 16;
#[allow(dead_code)]
pub const MAP_GROWSDOWN: u64 = 256;
#[allow(dead_code)]
pub const MAP_HUGETLB: u64 = 262144;
#[allow(dead_code)]
pub const MAP_LOCKED: u64 = 8192;
#[allow(dead_code)]
pub const MAP_NONBLOCK: u64 = 65536;
#[allow(dead_code)]
pub const MAP_NORESERVE: u64 = 16384;
#[allow(dead_code)]
pub const MAP_POPULATE: u64 = 32768;
#[allow(dead_code)]
pub const MAP_PRIVATE: u64 = 2;
#[allow(dead_code)]
pub const MAP_SHARED: u64 = 1;
#[allow(dead_code)]
pub const MAP_STACK: u64 = 131072;
#[allow(dead_code)]
pub const MAP_UNINITIALIZED: u64 = 0;
#[allow(dead_code)]
pub const MCL_CURRENT: u64 = 1;
#[allow(dead_code)]
pub const MCL_FUTURE: u64 = 2;
#[allow(dead_code)]
pub const MFD_ALLOW_SEALING: u64 = 2;
#[allow(dead_code)]
pub const MFD_CLOEXEC: u64 = 1;
#[allow(dead_code)]
pub const MLOCK_ONFAULT: u64 = 1;
#[allow(dead_code)]
pub const MMAP_PAGE_ZERO: u64 = 1048576;
#[allow(dead_code)]
pub const MNT_DETACH: u64 = 2;
#[allow(dead_code)]
pub const MNT_EXPIRE: u64 = 4;
#[allow(dead_code)]
pub const MNT_FORCE: u64 = 1;
#[allow(dead_code)]
pub const MODULE_INIT_IGNORE_MODVERSIONS: u64 = 1;
#[allow(dead_code)]
pub const MODULE_INIT_IGNORE_VERMAGIC: u64 = 2;
#[allow(dead_code)]
pub const MPOL_BIND: u64 = 2;
#[allow(dead_code)]
pub const MPOL_DEFAULT: u64 = 0;
#[allow(dead_code)]
pub const MPOL_F_ADDR: u64 = 2;
#[allow(dead_code)]
pub const MPOL_F_MEMS_ALLOWED: u64 = 4;
#[allow(dead_code)]
pub const MPOL_F_NODE: u64 = 1;
#[allow(dead_code)]
pub const MPOL_F_RELATIVE_NODES: u64 = 16384;
#[allow(dead_code)]
pub const MPOL_F_STATIC_NODES: u64 = 32768;
#[allow(dead_code)]
pub const MPOL_INTERLEAVE: u64 = 3;
#[allow(dead_code)]
pub const MPOL_MF_MOVE: u64 = 2;
#[allow(dead_code)]
pub const MPOL_MF_MOVE_ALL: u64 = 4;
#[allow(dead_code)]
pub const MPOL_MF_STRICT: u64 = 1;
#[allow(dead_code)]
pub const MPOL_PREFERRED: u64 = 1;
#[allow(dead_code)]
pub const MREMAP_FIXED: u64 = 2;
#[allow(dead_code)]
pub const MREMAP_MAYMOVE: u64 = 1;
#[allow(dead_code)]
pub const MS_ASYNC: u64 = 1;
#[allow(dead_code)]
pub const MS_BIND: u64 = 4096;
#[allow(dead_code)]
pub const MS_DIRSYNC: u64 = 128;
#[allow(dead_code)]
pub const MS_INVALIDATE: u64 = 2;
#[allow(dead_code)]
pub const MS_I_VERSION: u64 = 8388608;
#[allow(dead_code)]
pub const MS_LAZYTIME: u64 = 33554432;
#[allow(dead_code)]
pub const MS_MANDLOCK: u64 = 64;
#[allow(dead_code)]
pub const MS_MOVE: u64 = 8192;
#[allow(dead_code)]
pub const MS_NOATIME: u64 = 1024;
#[allow(dead_code)]
pub const MS_NODEV: u64 = 4;
#[allow(dead_code)]
pub const MS_NODIRATIME: u64 = 2048;
#[allow(dead_code)]
pub const MS_NOEXEC: u64 = 8;
#[allow(dead_code)]
pub const MS_NOSUID: u64 = 2;
#[allow(dead_code)]
pub const MS_POSIXACL: u64 = 65536;
#[allow(dead_code)]
pub const MS_PRIVATE: u64 = 262144;
#[allow(dead_code)]
pub const MS_RDONLY: u64 = 1;
#[allow(dead_code)]
pub const MS_REC: u64 = 16384;
#[allow(dead_code)]
pub const MS_RELATIME: u64 = 2097152;
#[allow(dead_code)]
pub const MS_REMOUNT: u64 = 32;
#[allow(dead_code)]
pub const MS_SHARED: u64 = 1048576;
#[allow(dead_code)]
pub const MS_SILENT: u64 = 32768;
#[allow(dead_code)]
pub const MS_SLAVE: u64 = 524288;
#[allow(dead_code)]
pub const MS_STRICTATIME: u64 = 16777216;
#[allow(dead_code)]
pub const MS_SYNC: u64 = 4;
#[allow(dead_code)]
pub const MS_SYNCHRONOUS: u64 = 16;
#[allow(dead_code)]
pub const MS_UNBINDABLE: u64 = 131072;
#[allow(dead_code)]
pub const NT_386_IOPERM: u64 = 513;
#[allow(dead_code)]
pub const NT_386_TLS: u64 = 512;
#[allow(dead_code)]
pub const NT_AUXV: u64 = 6;
#[allow(dead_code)]
pub const NT_PRFPREG: u64 = 2;
#[allow(dead_code)]
pub const NT_PRPSINFO: u64 = 3;
#[allow(dead_code)]
pub const NT_PRSTATUS: u64 = 1;
#[allow(dead_code)]
pub const NT_TASKSTRUCT: u64 = 4;
#[allow(dead_code)]
pub const NT_X86_XSTATE: u64 = 514;
#[allow(dead_code)]
pub const O_APPEND: u64 = 1024;
#[allow(dead_code)]
pub const O_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const O_CREAT: u64 = 64;
#[allow(dead_code)]
pub const O_DIRECT: u64 = 16384;
#[allow(dead_code)]
pub const O_DIRECTORY: u64 = 65536;
#[allow(dead_code)]
pub const O_DSYNC: u64 = 4096;
#[allow(dead_code)]
pub const O_EXCL: u64 = 128;
#[allow(dead_code)]
pub const O_LARGEFILE: u64 = 32768;
#[allow(dead_code)]
pub const O_NOATIME: u64 = 262144;
#[allow(dead_code)]
pub const O_NOCTTY: u64 = 256;
#[allow(dead_code)]
pub const O_NOFOLLOW: u64 = 131072;
#[allow(dead_code)]
pub const O_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const O_PATH: u64 = 2097152;
#[allow(dead_code)]
pub const O_RDONLY: u64 = 0;
#[allow(dead_code)]
pub const O_RDWR: u64 = 2;
#[allow(dead_code)]
pub const O_SYNC: u64 = 1052672;
#[allow(dead_code)]
pub const O_TRUNC: u64 = 512;
#[allow(dead_code)]
pub const O_WRONLY: u64 = 1;
#[allow(dead_code)]
pub const PER_BSD: u64 = 6;
#[allow(dead_code)]
pub const PER_HPUX: u64 = 16;
#[allow(dead_code)]
pub const PER_IRIX32: u64 = 67108873;
#[allow(dead_code)]
pub const PER_IRIX64: u64 = 67108875;
#[allow(dead_code)]
pub const PER_IRIXN32: u64 = 67108874;
#[allow(dead_code)]
pub const PER_ISCR4: u64 = 67108869;
#[allow(dead_code)]
pub const PER_LINUX: u64 = 0;
#[allow(dead_code)]
pub const PER_LINUX32: u64 = 8;
#[allow(dead_code)]
pub const PER_OSF4: u64 = 15;
#[allow(dead_code)]
pub const PER_OSR5: u64 = 100663299;
#[allow(dead_code)]
pub const PER_RISCOS: u64 = 12;
#[allow(dead_code)]
pub const PER_SOLARIS: u64 = 67108877;
#[allow(dead_code)]
pub const PER_SVR3: u64 = 83886082;
#[allow(dead_code)]
pub const PER_SVR4: u64 = 68157441;
#[allow(dead_code)]
pub const PER_UW7: u64 = 68157454;
#[allow(dead_code)]
pub const PER_WYSEV386: u64 = 83886084;
#[allow(dead_code)]
pub const PER_XENIX: u64 = 83886087;
#[allow(dead_code)]
pub const PKEY_DISABLE_ACCESS: u64 = 1;
#[allow(dead_code)]
pub const PKEY_DISABLE_WRITE: u64 = 2;
#[allow(dead_code)]
pub const POLLERR: u64 = 8;
#[allow(dead_code)]
pub const POLLFREE: u64 = 16384;
#[allow(dead_code)]
pub const POLLHUP: u64 = 16;
#[allow(dead_code)]
pub const POLLIN: u64 = 1;
#[allow(dead_code)]
pub const POLLMSG: u64 = 1024;
#[allow(dead_code)]
pub const POLLNVAL: u64 = 32;
#[allow(dead_code)]
pub const POLLOUT: u64 = 4;
#[allow(dead_code)]
pub const POLLPRI: u64 = 2;
#[allow(dead_code)]
pub const POLLRDBAND: u64 = 128;
#[allow(dead_code)]
pub const POLLRDHUP: u64 = 8192;
#[allow(dead_code)]
pub const POLLRDNORM: u64 = 64;
#[allow(dead_code)]
pub const POLLREMOVE: u64 = 4096;
#[allow(dead_code)]
pub const POLLWRBAND: u64 = 512;
#[allow(dead_code)]
pub const POLLWRNORM: u64 = 256;
#[allow(dead_code)]
pub const POLL_BUSY_LOOP: u64 = 32768;
#[allow(dead_code)]
pub const POSIX_FADV_DONTNEED: u64 = 4;
#[allow(dead_code)]
pub const POSIX_FADV_NOREUSE: u64 = 5;
#[allow(dead_code)]
pub const POSIX_FADV_NORMAL: u64 = 0;
#[allow(dead_code)]
pub const POSIX_FADV_RANDOM: u64 = 1;
#[allow(dead_code)]
pub const POSIX_FADV_SEQUENTIAL: u64 = 2;
#[allow(dead_code)]
pub const POSIX_FADV_WILLNEED: u64 = 3;
#[allow(dead_code)]
pub const PRIO_PGRP: u64 = 1;
#[allow(dead_code)]
pub const PRIO_PROCESS: u64 = 0;
#[allow(dead_code)]
pub const PRIO_USER: u64 = 2;
#[allow(dead_code)]
pub const PROT_EXEC: u64 = 4;
#[allow(dead_code)]
pub const PROT_GROWSDOWN: u64 = 16777216;
#[allow(dead_code)]
pub const PROT_GROWSUP: u64 = 33554432;
#[allow(dead_code)]
pub const PROT_READ: u64 = 1;
#[allow(dead_code)]
pub const PROT_SEM: u64 = 8;
#[allow(dead_code)]
pub const PROT_WRITE: u64 = 2;
#[allow(dead_code)]
pub const PR_CAPBSET_DROP: u64 = 24;
#[allow(dead_code)]
pub const PR_CAPBSET_READ: u64 = 23;
#[allow(dead_code)]
pub const PR_ENDIAN_BIG: u64 = 0;
#[allow(dead_code)]
pub const PR_ENDIAN_LITTLE: u64 = 1;
#[allow(dead_code)]
pub const PR_ENDIAN_PPC_LITTLE: u64 = 2;
#[allow(dead_code)]
pub const PR_FP_EXC_ASYNC: u64 = 2;
#[allow(dead_code)]
pub const PR_FP_EXC_DISABLED: u64 = 0;
#[allow(dead_code)]
pub const PR_FP_EXC_DIV: u64 = 65536;
#[allow(dead_code)]
pub const PR_FP_EXC_INV: u64 = 1048576;
#[allow(dead_code)]
pub const PR_FP_EXC_NONRECOV: u64 = 1;
#[allow(dead_code)]
pub const PR_FP_EXC_OVF: u64 = 131072;
#[allow(dead_code)]
pub const PR_FP_EXC_PRECISE: u64 = 3;
#[allow(dead_code)]
pub const PR_FP_EXC_RES: u64 = 524288;
#[allow(dead_code)]
pub const PR_FP_EXC_SW_ENABLE: u64 = 128;
#[allow(dead_code)]
pub const PR_FP_EXC_UND: u64 = 262144;
#[allow(dead_code)]
pub const PR_GET_CHILD_SUBREAPER: u64 = 37;
#[allow(dead_code)]
pub const PR_GET_DUMPABLE: u64 = 3;
#[allow(dead_code)]
pub const PR_GET_ENDIAN: u64 = 19;
#[allow(dead_code)]
pub const PR_GET_FPEMU: u64 = 9;
#[allow(dead_code)]
pub const PR_GET_FPEXC: u64 = 11;
#[allow(dead_code)]
pub const PR_GET_KEEPCAPS: u64 = 7;
#[allow(dead_code)]
pub const PR_GET_NAME: u64 = 16;
#[allow(dead_code)]
pub const PR_GET_NO_NEW_PRIVS: u64 = 39;
#[allow(dead_code)]
pub const PR_GET_PDEATHSIG: u64 = 2;
#[allow(dead_code)]
pub const PR_GET_SECCOMP: u64 = 21;
#[allow(dead_code)]
pub const PR_GET_SECUREBITS: u64 = 27;
#[allow(dead_code)]
pub const PR_GET_TID_ADDRESS: u64 = 40;
#[allow(dead_code)]
pub const PR_GET_TIMERSLACK: u64 = 30;
#[allow(dead_code)]
pub const PR_GET_TIMING: u64 = 13;
#[allow(dead_code)]
pub const PR_GET_TSC: u64 = 25;
#[allow(dead_code)]
pub const PR_GET_UNALIGN: u64 = 5;
#[allow(dead_code)]
pub const PR_MCE_KILL: u64 = 33;
#[allow(dead_code)]
pub const PR_MCE_KILL_GET: u64 = 34;
#[allow(dead_code)]
pub const PR_SET_CHILD_SUBREAPER: u64 = 36;
#[allow(dead_code)]
pub const PR_SET_DUMPABLE: u64 = 4;
#[allow(dead_code)]
pub const PR_SET_ENDIAN: u64 = 20;
#[allow(dead_code)]
pub const PR_SET_FPEMU: u64 = 10;
#[allow(dead_code)]
pub const PR_SET_FPEXC: u64 = 12;
#[allow(dead_code)]
pub const PR_SET_KEEPCAPS: u64 = 8;
#[allow(dead_code)]
pub const PR_SET_MM: u64 = 35;
#[allow(dead_code)]
pub const PR_SET_MM_BRK: u64 = 7;
#[allow(dead_code)]
pub const PR_SET_MM_END_CODE: u64 = 2;
#[allow(dead_code)]
pub const PR_SET_MM_END_DATA: u64 = 4;
#[allow(dead_code)]
pub const PR_SET_MM_START_BRK: u64 = 6;
#[allow(dead_code)]
pub const PR_SET_MM_START_CODE: u64 = 1;
#[allow(dead_code)]
pub const PR_SET_MM_START_DATA: u64 = 3;
#[allow(dead_code)]
pub const PR_SET_MM_START_STACK: u64 = 5;
#[allow(dead_code)]
pub const PR_SET_NAME: u64 = 15;
#[allow(dead_code)]
pub const PR_SET_NO_NEW_PRIVS: u64 = 38;
#[allow(dead_code)]
pub const PR_SET_PDEATHSIG: u64 = 1;
#[allow(dead_code)]
pub const PR_SET_PTRACER: u64 = 1499557217;
#[allow(dead_code)]
pub const PR_SET_SECCOMP: u64 = 22;
#[allow(dead_code)]
pub const PR_SET_SECUREBITS: u64 = 28;
#[allow(dead_code)]
pub const PR_SET_TIMERSLACK: u64 = 29;
#[allow(dead_code)]
pub const PR_SET_TIMING: u64 = 14;
#[allow(dead_code)]
pub const PR_SET_TSC: u64 = 26;
#[allow(dead_code)]
pub const PR_SET_UNALIGN: u64 = 6;
#[allow(dead_code)]
pub const PR_TASK_PERF_EVENTS_DISABLE: u64 = 31;
#[allow(dead_code)]
pub const PR_TASK_PERF_EVENTS_ENABLE: u64 = 32;
#[allow(dead_code)]
pub const PTRACE_ATTACH: u64 = 16;
#[allow(dead_code)]
pub const PTRACE_CONT: u64 = 7;
#[allow(dead_code)]
pub const PTRACE_DETACH: u64 = 17;
#[allow(dead_code)]
pub const PTRACE_GETEVENTMSG: u64 = 16897;
#[allow(dead_code)]
pub const PTRACE_GETFPREGS: u64 = 14;
#[allow(dead_code)]
pub const PTRACE_GETREGS: u64 = 12;
#[allow(dead_code)]
pub const PTRACE_GETREGSET: u64 = 16900;
#[allow(dead_code)]
pub const PTRACE_GETSIGINFO: u64 = 16898;
#[allow(dead_code)]
pub const PTRACE_INTERRUPT: u64 = 16903;
#[allow(dead_code)]
pub const PTRACE_KILL: u64 = 8;
#[allow(dead_code)]
pub const PTRACE_LISTEN: u64 = 16904;
#[allow(dead_code)]
pub const PTRACE_O_EXITKILL: u64 = 1048576;
#[allow(dead_code)]
pub const PTRACE_O_TRACECLONE: u64 = 8;
#[allow(dead_code)]
pub const PTRACE_O_TRACEEXEC: u64 = 16;
#[allow(dead_code)]
pub const PTRACE_O_TRACEEXIT: u64 = 64;
#[allow(dead_code)]
pub const PTRACE_O_TRACEFORK: u64 = 2;
#[allow(dead_code)]
pub const PTRACE_O_TRACESYSGOOD: u64 = 1;
#[allow(dead_code)]
pub const PTRACE_O_TRACEVFORK: u64 = 4;
#[allow(dead_code)]
pub const PTRACE_O_TRACEVFORKDONE: u64 = 32;
#[allow(dead_code)]
pub const PTRACE_PEEKDATA: u64 = 2;
#[allow(dead_code)]
pub const PTRACE_PEEKTEXT: u64 = 1;
#[allow(dead_code)]
pub const PTRACE_PEEKUSR: u64 = 3;
#[allow(dead_code)]
pub const PTRACE_POKEDATA: u64 = 5;
#[allow(dead_code)]
pub const PTRACE_POKETEXT: u64 = 4;
#[allow(dead_code)]
pub const PTRACE_POKEUSR: u64 = 6;
#[allow(dead_code)]
pub const PTRACE_SEIZE: u64 = 16902;
#[allow(dead_code)]
pub const PTRACE_SETFPREGS: u64 = 15;
#[allow(dead_code)]
pub const PTRACE_SETOPTIONS: u64 = 16896;
#[allow(dead_code)]
pub const PTRACE_SETREGS: u64 = 13;
#[allow(dead_code)]
pub const PTRACE_SETREGSET: u64 = 16901;
#[allow(dead_code)]
pub const PTRACE_SETSIGINFO: u64 = 16899;
#[allow(dead_code)]
pub const PTRACE_SINGLESTEP: u64 = 9;
#[allow(dead_code)]
pub const PTRACE_SYSCALL: u64 = 24;
#[allow(dead_code)]
pub const PTRACE_SYSEMU: u64 = 31;
#[allow(dead_code)]
pub const PTRACE_SYSEMU_SINGLESTEP: u64 = 32;
#[allow(dead_code)]
pub const PTRACE_TRACEME: u64 = 0;
#[allow(dead_code)]
pub const P_ALL: u64 = 0;
#[allow(dead_code)]
pub const P_PGID: u64 = 2;
#[allow(dead_code)]
pub const P_PID: u64 = 1;
#[allow(dead_code)]
pub const READ_IMPLIES_EXEC: u64 = 4194304;
#[allow(dead_code)]
pub const RENAME_EXCHANGE: u64 = 2;
#[allow(dead_code)]
pub const RENAME_NOREPLACE: u64 = 1;
#[allow(dead_code)]
pub const RENAME_WHITEOUT: u64 = 4;
#[allow(dead_code)]
pub const RLIMIT_AS: u64 = 9;
#[allow(dead_code)]
pub const RLIMIT_CORE: u64 = 4;
#[allow(dead_code)]
pub const RLIMIT_CPU: u64 = 0;
#[allow(dead_code)]
pub const RLIMIT_DATA: u64 = 2;
#[allow(dead_code)]
pub const RLIMIT_FSIZE: u64 = 1;
#[allow(dead_code)]
pub const RLIMIT_LOCKS: u64 = 10;
#[allow(dead_code)]
pub const RLIMIT_MEMLOCK: u64 = 8;
#[allow(dead_code)]
pub const RLIMIT_MSGQUEUE: u64 = 12;
#[allow(dead_code)]
pub const RLIMIT_NICE: u64 = 13;
#[allow(dead_code)]
pub const RLIMIT_NOFILE: u64 = 7;
#[allow(dead_code)]
pub const RLIMIT_NPROC: u64 = 6;
#[allow(dead_code)]
pub const RLIMIT_RSS: u64 = 5;
#[allow(dead_code)]
pub const RLIMIT_RTPRIO: u64 = 14;
#[allow(dead_code)]
pub const RLIMIT_RTTIME: u64 = 15;
#[allow(dead_code)]
pub const RLIMIT_SIGPENDING: u64 = 11;
#[allow(dead_code)]
pub const RLIMIT_STACK: u64 = 3;
#[allow(dead_code)]
pub const RUSAGE_CHILDREN: u64 = 18446744073709551615;
#[allow(dead_code)]
pub const RUSAGE_SELF: u64 = 0;
#[allow(dead_code)]
pub const RUSAGE_THREAD: u64 = 1;
#[allow(dead_code)]
pub const SA_NOCLDSTOP: u64 = 1;
#[allow(dead_code)]
pub const SA_NOCLDWAIT: u64 = 2;
#[allow(dead_code)]
pub const SA_NODEFER: u64 = 1073741824;
#[allow(dead_code)]
pub const SA_ONSTACK: u64 = 134217728;
#[allow(dead_code)]
pub const SA_RESETHAND: u64 = 2147483648;
#[allow(dead_code)]
pub const SA_RESTART: u64 = 268435456;
#[allow(dead_code)]
pub const SA_SIGINFO: u64 = 4;
#[allow(dead_code)]
pub const SCHED_BATCH: u64 = 3;
#[allow(dead_code)]
pub const SCHED_DEADLINE: u64 = 6;
#[allow(dead_code)]
pub const SCHED_FIFO: u64 = 1;
#[allow(dead_code)]
pub const SCHED_FLAG_RESET_ON_FORK: u64 = 1;
#[allow(dead_code)]
pub const SCHED_IDLE: u64 = 5;
#[allow(dead_code)]
pub const SCHED_NORMAL: u64 = 0;
#[allow(dead_code)]
pub const SCHED_RR: u64 = 2;
#[allow(dead_code)]
pub const SECCOMP_FILTER_FLAG_TSYNC: u64 = 1;
#[allow(dead_code)]
pub const SECCOMP_MODE_DISABLED: u64 = 0;
#[allow(dead_code)]
pub const SECCOMP_MODE_FILTER: u64 = 2;
#[allow(dead_code)]
pub const SECCOMP_MODE_STRICT: u64 = 1;
#[allow(dead_code)]
pub const SECCOMP_SET_MODE_FILTER: u64 = 1;
#[allow(dead_code)]
pub const SECCOMP_SET_MODE_STRICT: u64 = 0;
#[allow(dead_code)]
pub const SEEK_CUR: u64 = 1;
#[allow(dead_code)]
pub const SEEK_DATA: u64 = 3;
#[allow(dead_code)]
pub const SEEK_END: u64 = 2;
#[allow(dead_code)]
pub const SEEK_HOLE: u64 = 4;
#[allow(dead_code)]
pub const SEEK_SET: u64 = 0;
#[allow(dead_code)]
pub const SFD_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const SFD_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const SHORT_INODE: u64 = 16777216;
#[allow(dead_code)]
pub const SIGEV_NONE: u64 = 1;
#[allow(dead_code)]
pub const SIGEV_SIGNAL: u64 = 0;
#[allow(dead_code)]
pub const SIGEV_THREAD: u64 = 2;
#[allow(dead_code)]
pub const SIGEV_THREAD_ID: u64 = 4;
#[allow(dead_code)]
pub const SIG_BLOCK: u64 = 0;
#[allow(dead_code)]
pub const SIG_SETMASK: u64 = 2;
#[allow(dead_code)]
pub const SIG_UNBLOCK: u64 = 1;
#[allow(dead_code)]
pub const SPLICE_F_GIFT: u64 = 8;
#[allow(dead_code)]
pub const SPLICE_F_MORE: u64 = 4;
#[allow(dead_code)]
pub const SPLICE_F_MOVE: u64 = 1;
#[allow(dead_code)]
pub const SPLICE_F_NONBLOCK: u64 = 2;
#[allow(dead_code)]
pub const STATX_ALL: u64 = 4095;
#[allow(dead_code)]
pub const STATX_ATIME: u64 = 32;
#[allow(dead_code)]
pub const STATX_BASIC_STATS: u64 = 2047;
#[allow(dead_code)]
pub const STATX_BLOCKS: u64 = 1024;
#[allow(dead_code)]
pub const STATX_BTIME: u64 = 2048;
#[allow(dead_code)]
pub const STATX_CTIME: u64 = 128;
#[allow(dead_code)]
pub const STATX_GID: u64 = 16;
#[allow(dead_code)]
pub const STATX_INO: u64 = 256;
#[allow(dead_code)]
pub const STATX_MODE: u64 = 2;
#[allow(dead_code)]
pub const STATX_MTIME: u64 = 64;
#[allow(dead_code)]
pub const STATX_NLINK: u64 = 4;
#[allow(dead_code)]
pub const STATX_SIZE: u64 = 512;
#[allow(dead_code)]
pub const STATX_TYPE: u64 = 1;
#[allow(dead_code)]
pub const STATX_UID: u64 = 8;
#[allow(dead_code)]
pub const STICKY_TIMEOUTS: u64 = 67108864;
#[allow(dead_code)]
pub const SYNC_FILE_RANGE_WAIT_AFTER: u64 = 4;
#[allow(dead_code)]
pub const SYNC_FILE_RANGE_WAIT_BEFORE: u64 = 1;
#[allow(dead_code)]
pub const SYNC_FILE_RANGE_WRITE: u64 = 2;
#[allow(dead_code)]
pub const SYSLOG_ACTION_CLEAR: u64 = 5;
#[allow(dead_code)]
pub const SYSLOG_ACTION_CLOSE: u64 = 0;
#[allow(dead_code)]
pub const SYSLOG_ACTION_CONSOLE_OFF: u64 = 6;
#[allow(dead_code)]
pub const SYSLOG_ACTION_CONSOLE_ON: u64 = 7;
#[allow(dead_code)]
pub const SYSLOG_ACTION_OPEN: u64 = 1;
#[allow(dead_code)]
pub const SYSLOG_ACTION_READ: u64 = 2;
#[allow(dead_code)]
pub const SYSLOG_ACTION_READ_ALL: u64 = 3;
#[allow(dead_code)]
pub const SYSLOG_ACTION_READ_CLEAR: u64 = 4;
#[allow(dead_code)]
pub const SYSLOG_ACTION_SIZE_BUFFER: u64 = 10;
#[allow(dead_code)]
pub const SYSLOG_ACTION_SIZE_UNREAD: u64 = 9;
#[allow(dead_code)]
pub const S_IFBLK: u64 = 24576;
#[allow(dead_code)]
pub const S_IFCHR: u64 = 8192;
#[allow(dead_code)]
pub const S_IFIFO: u64 = 4096;
#[allow(dead_code)]
pub const S_IFREG: u64 = 32768;
#[allow(dead_code)]
pub const S_IFSOCK: u64 = 49152;
#[allow(dead_code)]
pub const S_IRGRP: u64 = 32;
#[allow(dead_code)]
pub const S_IROTH: u64 = 4;
#[allow(dead_code)]
pub const S_IRUSR: u64 = 256;
#[allow(dead_code)]
pub const S_IWGRP: u64 = 16;
#[allow(dead_code)]
pub const S_IWOTH: u64 = 2;
#[allow(dead_code)]
pub const S_IWUSR: u64 = 128;
#[allow(dead_code)]
pub const S_IXGRP: u64 = 8;
#[allow(dead_code)]
pub const S_IXOTH: u64 = 1;
#[allow(dead_code)]
pub const S_IXUSR: u64 = 64;
#[allow(dead_code)]
pub const TFD_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const TFD_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const TFD_TIMER_ABSTIME: u64 = 1;
#[allow(dead_code)]
pub const TIMER_ABSTIME: u64 = 1;
#[allow(dead_code)]
pub const UFFDIO_API: u64 = 3222841919;
#[allow(dead_code)]
pub const UFFDIO_COPY_MODE_DONTWAKE: u64 = 1;
#[allow(dead_code)]
pub const UFFDIO_REGISTER: u64 = 3223366144;
#[allow(dead_code)]
pub const UFFDIO_REGISTER_MODE_MISSING: u64 = 1;
#[allow(dead_code)]
pub const UFFDIO_REGISTER_MODE_WP: u64 = 2;
#[allow(dead_code)]
pub const UFFDIO_UNREGISTER: u64 = 2148575745;
#[allow(dead_code)]
pub const UFFDIO_WAKE: u64 = 2148575746;
#[allow(dead_code)]
pub const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u64 = 1;
#[allow(dead_code)]
pub const UFFD_API: u64 = 170;
#[allow(dead_code)]
pub const UFFD_FEATURE_EVENT_FORK: u64 = 2;
#[allow(dead_code)]
pub const UFFD_FEATURE_EVENT_REMAP: u64 = 4;
#[allow(dead_code)]
pub const UFFD_FEATURE_EVENT_REMOVE: u64 = 8;
#[allow(dead_code)]
pub const UFFD_FEATURE_EVENT_UNMAP: u64 = 64;
#[allow(dead_code)]
pub const UFFD_FEATURE_MISSING_HUGETLBFS: u64 = 16;
#[allow(dead_code)]
pub const UFFD_FEATURE_MISSING_SHMEM: u64 = 32;
#[allow(dead_code)]
pub const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u64 = 1;
#[allow(dead_code)]
pub const UMOUNT_NOFOLLOW: u64 = 8;
#[allow(dead_code)]
pub const WCONTINUED: u64 = 8;
#[allow(dead_code)]
pub const WEXITED: u64 = 4;
#[allow(dead_code)]
pub const WHOLE_SECONDS: u64 = 33554432;
#[allow(dead_code)]
pub const WNOHANG: u64 = 1;
#[allow(dead_code)]
pub const WNOWAIT: u64 = 16777216;
#[allow(dead_code)]
pub const WSTOPPED: u64 = 2;
#[allow(dead_code)]
pub const WUNTRACED: u64 = 2;

#[allow(dead_code)]
pub const GETALL: u64 = 13;
#[allow(dead_code)]
pub const GETNCNT: u64 = 14;
#[allow(dead_code)]
pub const GETPID: u64 = 11;
#[allow(dead_code)]
pub const GETVAL: u64 = 12;
#[allow(dead_code)]
pub const GETZCNT: u64 = 15;

#[allow(dead_code)]
pub const IPC_CREAT: u64 = 512;
#[allow(dead_code)]
pub const IPC_EXCL: u64 = 1024;
#[allow(dead_code)]
pub const IPC_INFO: u64 = 3;
#[allow(dead_code)]
pub const IPC_NOWAIT: u64 = 2048;
#[allow(dead_code)]
pub const IPC_PRIVATE: u64 = 0;
#[allow(dead_code)]
pub const IPC_RMID: u64 = 0;
#[allow(dead_code)]
pub const IPC_SET: u64 = 1;
#[allow(dead_code)]
pub const IPC_STAT: u64 = 2;
#[allow(dead_code)]
pub const MSG_EXCEPT: u64 = 8192;
#[allow(dead_code)]
pub const MSG_INFO: u64 = 12;
#[allow(dead_code)]
pub const MSG_NOERROR: u64 = 4096;
#[allow(dead_code)]
pub const MSG_STAT: u64 = 11;
#[allow(dead_code)]
pub const SEM_INFO: u64 = 19;
#[allow(dead_code)]
pub const SEM_STAT: u64 = 18;
#[allow(dead_code)]
pub const SEM_UNDO: u64 = 4096;
#[allow(dead_code)]
pub const SETALL: u64 = 17;
#[allow(dead_code)]
pub const SETVAL: u64 = 16;
#[allow(dead_code)]
pub const SHM_HUGETLB: u64 = 2048;
#[allow(dead_code)]
pub const SHM_HUGE_1GB: u64 = 2013265920;
#[allow(dead_code)]
pub const SHM_HUGE_2MB: u64 = 1409286144;
#[allow(dead_code)]
pub const SHM_INFO: u64 = 14;
#[allow(dead_code)]
pub const SHM_LOCK: u64 = 11;
#[allow(dead_code)]
pub const SHM_NORESERVE: u64 = 4096;
#[allow(dead_code)]
pub const SHM_RDONLY: u64 = 4096;
#[allow(dead_code)]
pub const SHM_REMAP: u64 = 16384;
#[allow(dead_code)]
pub const SHM_RND: u64 = 8192;
#[allow(dead_code)]
pub const SHM_STAT: u64 = 13;
#[allow(dead_code)]
pub const SHM_UNLOCK: u64 = 12;
#[allow(dead_code)]
pub const SHM_LOCKED: u64 = 400;
#[allow(dead_code)]
pub const SHM_DEST: u64 = 200;


#[allow(dead_code)]
pub const SOL_AAL: u64 = 265;
#[allow(dead_code)]
pub const SOL_ALG: u64 = 279;
#[allow(dead_code)]
pub const SOL_ATALK: u64 = 258;
#[allow(dead_code)]
pub const SOL_ATM: u64 = 264;
#[allow(dead_code)]
pub const SOL_AX25: u64 = 257;
#[allow(dead_code)]
pub const SOL_BLUETOOTH: u64 = 274;
#[allow(dead_code)]
pub const SOL_CAIF: u64 = 278;
#[allow(dead_code)]
pub const SOL_DCCP: u64 = 269;
#[allow(dead_code)]
pub const SOL_DECNET: u64 = 261;
#[allow(dead_code)]
pub const SOL_ICMPV6: u64 = 58;
#[allow(dead_code)]
pub const SOL_IP: u64 = 0;
#[allow(dead_code)]
pub const SOL_IPV6: u64 = 41;
#[allow(dead_code)]
pub const SOL_IPX: u64 = 256;
#[allow(dead_code)]
pub const SOL_IRDA: u64 = 266;
#[allow(dead_code)]
pub const SOL_IUCV: u64 = 277;
#[allow(dead_code)]
pub const SOL_KCM: u64 = 281;
#[allow(dead_code)]
pub const SOL_LLC: u64 = 268;
#[allow(dead_code)]
pub const SOL_NETBEUI: u64 = 267;
#[allow(dead_code)]
pub const SOL_NETLINK: u64 = 270;
#[allow(dead_code)]
pub const SOL_NETROM: u64 = 259;
#[allow(dead_code)]
pub const SOL_NFC: u64 = 280;
#[allow(dead_code)]
pub const SOL_PACKET: u64 = 263;
#[allow(dead_code)]
pub const SOL_PNPIPE: u64 = 275;
#[allow(dead_code)]
pub const SOL_PPPOL2TP: u64 = 273;
#[allow(dead_code)]
pub const SOL_RAW: u64 = 255;
#[allow(dead_code)]
pub const SOL_RDS: u64 = 276;
#[allow(dead_code)]
pub const SOL_ROSE: u64 = 260;
#[allow(dead_code)]
pub const SOL_RXRPC: u64 = 272;
#[allow(dead_code)]
pub const SOL_SCTP: u64 = 132;
#[allow(dead_code)]
pub const SOL_SOCKET: u64 = 1;
#[allow(dead_code)]
pub const SOL_TCP: u64 = 6;
#[allow(dead_code)]
pub const SOL_TIPC: u64 = 271;
#[allow(dead_code)]
pub const SOL_UDP: u64 = 17;
#[allow(dead_code)]
pub const SOL_UDPLITE: u64 = 136;
#[allow(dead_code)]
pub const SOPASS_MAX: u64 = 6;
#[allow(dead_code)]
pub const SO_ACCEPTCONN: u64 = 30;
#[allow(dead_code)]
pub const SO_ATTACH_BPF: u64 = 50;
#[allow(dead_code)]
pub const SO_ATTACH_FILTER: u64 = 26;
#[allow(dead_code)]
pub const SO_BINDTODEVICE: u64 = 25;
#[allow(dead_code)]
pub const SO_BROADCAST: u64 = 6;
#[allow(dead_code)]
pub const SO_BUSY_POLL: u64 = 46;
#[allow(dead_code)]
pub const SO_DEBUG: u64 = 1;
#[allow(dead_code)]
pub const SO_DETACH_FILTER: u64 = 27;
#[allow(dead_code)]
pub const SO_DOMAIN: u64 = 39;
#[allow(dead_code)]
pub const SO_DONTROUTE: u64 = 5;
#[allow(dead_code)]
pub const SO_ERROR: u64 = 4;
#[allow(dead_code)]
pub const SO_GET_FILTER: u64 = 26;
#[allow(dead_code)]
pub const SO_KEEPALIVE: u64 = 9;
#[allow(dead_code)]
pub const SO_LINGER: u64 = 13;
#[allow(dead_code)]
pub const SO_LOCK_FILTER: u64 = 44;
#[allow(dead_code)]
pub const SO_MARK: u64 = 36;
#[allow(dead_code)]
pub const SO_MAX_PACING_RATE: u64 = 47;
#[allow(dead_code)]
pub const SO_NOFCS: u64 = 43;
#[allow(dead_code)]
pub const SO_NO_CHECK: u64 = 11;
#[allow(dead_code)]
pub const SO_OOBINLINE: u64 = 10;
#[allow(dead_code)]
pub const SO_PASSCRED: u64 = 16;
#[allow(dead_code)]
pub const SO_PASSSEC: u64 = 34;
#[allow(dead_code)]
pub const SO_PEEK_OFF: u64 = 42;
#[allow(dead_code)]
pub const SO_PEERCRED: u64 = 17;
#[allow(dead_code)]
pub const SO_PEERNAME: u64 = 28;
#[allow(dead_code)]
pub const SO_PEERSEC: u64 = 31;
#[allow(dead_code)]
pub const SO_PRIORITY: u64 = 12;
#[allow(dead_code)]
pub const SO_PROTOCOL: u64 = 38;
#[allow(dead_code)]
pub const SO_RCVBUF: u64 = 8;
#[allow(dead_code)]
pub const SO_RCVBUFFORCE: u64 = 33;
#[allow(dead_code)]
pub const SO_RCVLOWAT: u64 = 18;
#[allow(dead_code)]
pub const SO_RCVTIMEO: u64 = 20;
#[allow(dead_code)]
pub const SO_REUSEADDR: u64 = 2;
#[allow(dead_code)]
pub const SO_REUSEPORT: u64 = 15;
#[allow(dead_code)]
pub const SO_RXQ_OVFL: u64 = 40;
#[allow(dead_code)]
pub const SO_SELECT_ERR_QUEUE: u64 = 45;
#[allow(dead_code)]
pub const SO_SNDBUF: u64 = 7;
#[allow(dead_code)]
pub const SO_SNDBUFFORCE: u64 = 32;
#[allow(dead_code)]
pub const SO_SNDLOWAT: u64 = 19;
#[allow(dead_code)]
pub const SO_SNDTIMEO: u64 = 21;
#[allow(dead_code)]
pub const SO_TIMESTAMP: u64 = 29;
#[allow(dead_code)]
pub const SO_TIMESTAMPING: u64 = 37;
#[allow(dead_code)]
pub const SO_TIMESTAMPNS: u64 = 35;
#[allow(dead_code)]
pub const SO_TYPE: u64 = 3;
#[allow(dead_code)]
pub const SO_WIFI_STATUS: u64 = 41;

#[allow(dead_code)]
pub const AF_APPLETALK: u64 = 5;
#[allow(dead_code)]
pub const AF_ATMPVC: u64 = 8;
#[allow(dead_code)]
pub const AF_AX25: u64 = 3;
#[allow(dead_code)]
pub const AF_INET: u64 = 2;
#[allow(dead_code)]
pub const AF_INET6: u64 = 10;
#[allow(dead_code)]
pub const AF_IPX: u64 = 4;
#[allow(dead_code)]
pub const AF_NETLINK: u64 = 16;
#[allow(dead_code)]
pub const AF_PACKET: u64 = 17;
#[allow(dead_code)]
pub const AF_UNIX: u64 = 1;
#[allow(dead_code)]
pub const AF_X25: u64 = 9;
#[allow(dead_code)]
pub const AH_ESP_V4_FLOW: u64 = 4;
#[allow(dead_code)]
pub const AH_ESP_V6_FLOW: u64 = 8;
#[allow(dead_code)]
pub const AH_V4_FLOW: u64 = 9;
#[allow(dead_code)]
pub const AH_V6_FLOW: u64 = 11;
#[allow(dead_code)]
pub const BRCTL_ADD_BRIDGE: u64 = 2;
#[allow(dead_code)]
pub const BRCTL_DEL_BRIDGE: u64 = 3;
#[allow(dead_code)]
pub const BRCTL_GET_BRIDGES: u64 = 1;
#[allow(dead_code)]
pub const BRCTL_GET_VERSION: u64 = 0;
#[allow(dead_code)]
pub const ESP_V4_FLOW: u64 = 10;
#[allow(dead_code)]
pub const ESP_V6_FLOW: u64 = 12;
#[allow(dead_code)]
pub const ETHER_FLOW: u64 = 18;
#[allow(dead_code)]
pub const ETHTOOL_BUSINFO_LEN: u64 = 32;
#[allow(dead_code)]
pub const ETHTOOL_EROMVERS_LEN: u64 = 32;
#[allow(dead_code)]
pub const ETHTOOL_FLASHDEV: u64 = 51;
#[allow(dead_code)]
pub const ETHTOOL_FLASH_MAX_FILENAME: u64 = 128;
#[allow(dead_code)]
pub const ETHTOOL_FWVERS_LEN: u64 = 32;
#[allow(dead_code)]
pub const ETHTOOL_GCHANNELS: u64 = 60;
#[allow(dead_code)]
pub const ETHTOOL_GCOALESCE: u64 = 14;
#[allow(dead_code)]
pub const ETHTOOL_GDRVINFO: u64 = 3;
#[allow(dead_code)]
pub const ETHTOOL_GEEE: u64 = 68;
#[allow(dead_code)]
pub const ETHTOOL_GEEPROM: u64 = 11;
#[allow(dead_code)]
pub const ETHTOOL_GET_DUMP_DATA: u64 = 64;
#[allow(dead_code)]
pub const ETHTOOL_GET_DUMP_FLAG: u64 = 63;
#[allow(dead_code)]
pub const ETHTOOL_GET_TS_INFO: u64 = 65;
#[allow(dead_code)]
pub const ETHTOOL_GFEATURES: u64 = 58;
#[allow(dead_code)]
pub const ETHTOOL_GFLAGS: u64 = 37;
#[allow(dead_code)]
pub const ETHTOOL_GGRO: u64 = 43;
#[allow(dead_code)]
pub const ETHTOOL_GGSO: u64 = 35;
#[allow(dead_code)]
pub const ETHTOOL_GLINK: u64 = 10;
#[allow(dead_code)]
pub const ETHTOOL_GLINKSETTINGS: u64 = 76;
#[allow(dead_code)]
pub const ETHTOOL_GMODULEEEPROM: u64 = 67;
#[allow(dead_code)]
pub const ETHTOOL_GMODULEINFO: u64 = 66;
#[allow(dead_code)]
pub const ETHTOOL_GMSGLVL: u64 = 7;
#[allow(dead_code)]
pub const ETHTOOL_GPAUSEPARAM: u64 = 18;
#[allow(dead_code)]
pub const ETHTOOL_GPERMADDR: u64 = 32;
#[allow(dead_code)]
pub const ETHTOOL_GPFLAGS: u64 = 39;
#[allow(dead_code)]
pub const ETHTOOL_GPHYSTATS: u64 = 74;
#[allow(dead_code)]
pub const ETHTOOL_GREGS: u64 = 4;
#[allow(dead_code)]
pub const ETHTOOL_GRINGPARAM: u64 = 16;
#[allow(dead_code)]
pub const ETHTOOL_GRSSH: u64 = 70;
#[allow(dead_code)]
pub const ETHTOOL_GRXCLSRLALL: u64 = 48;
#[allow(dead_code)]
pub const ETHTOOL_GRXCLSRLCNT: u64 = 46;
#[allow(dead_code)]
pub const ETHTOOL_GRXCLSRULE: u64 = 47;
#[allow(dead_code)]
pub const ETHTOOL_GRXCSUM: u64 = 20;
#[allow(dead_code)]
pub const ETHTOOL_GRXFH: u64 = 41;
#[allow(dead_code)]
pub const ETHTOOL_GRXFHINDIR: u64 = 56;
#[allow(dead_code)]
pub const ETHTOOL_GRXNTUPLE: u64 = 54;
#[allow(dead_code)]
pub const ETHTOOL_GRXRINGS: u64 = 45;
#[allow(dead_code)]
pub const ETHTOOL_GSET: u64 = 1;
#[allow(dead_code)]
pub const ETHTOOL_GSG: u64 = 24;
#[allow(dead_code)]
pub const ETHTOOL_GSSET_INFO: u64 = 55;
#[allow(dead_code)]
pub const ETHTOOL_GSTATS: u64 = 29;
#[allow(dead_code)]
pub const ETHTOOL_GSTRINGS: u64 = 27;
#[allow(dead_code)]
pub const ETHTOOL_GTSO: u64 = 30;
#[allow(dead_code)]
pub const ETHTOOL_GTUNABLE: u64 = 72;
#[allow(dead_code)]
pub const ETHTOOL_GTXCSUM: u64 = 22;
#[allow(dead_code)]
pub const ETHTOOL_GUFO: u64 = 33;
#[allow(dead_code)]
pub const ETHTOOL_GWOL: u64 = 5;
#[allow(dead_code)]
pub const ETHTOOL_NWAY_RST: u64 = 9;
#[allow(dead_code)]
pub const ETHTOOL_PERQUEUE: u64 = 75;
#[allow(dead_code)]
pub const ETHTOOL_PHYS_ID: u64 = 28;
#[allow(dead_code)]
pub const ETHTOOL_PHY_GTUNABLE: u64 = 78;
#[allow(dead_code)]
pub const ETHTOOL_PHY_STUNABLE: u64 = 79;
#[allow(dead_code)]
pub const ETHTOOL_RESET: u64 = 52;
#[allow(dead_code)]
pub const ETHTOOL_RXNTUPLE_ACTION_CLEAR: u64 = 18446744073709551614;
#[allow(dead_code)]
pub const ETHTOOL_RXNTUPLE_ACTION_DROP: u64 = 18446744073709551615;
#[allow(dead_code)]
pub const ETHTOOL_SCHANNELS: u64 = 61;
#[allow(dead_code)]
pub const ETHTOOL_SCOALESCE: u64 = 15;
#[allow(dead_code)]
pub const ETHTOOL_SEEE: u64 = 69;
#[allow(dead_code)]
pub const ETHTOOL_SEEPROM: u64 = 12;
#[allow(dead_code)]
pub const ETHTOOL_SET_DUMP: u64 = 62;
#[allow(dead_code)]
pub const ETHTOOL_SFEATURES: u64 = 59;
#[allow(dead_code)]
pub const ETHTOOL_SFLAGS: u64 = 38;
#[allow(dead_code)]
pub const ETHTOOL_SGRO: u64 = 44;
#[allow(dead_code)]
pub const ETHTOOL_SGSO: u64 = 36;
#[allow(dead_code)]
pub const ETHTOOL_SLINKSETTINGS: u64 = 77;
#[allow(dead_code)]
pub const ETHTOOL_SMSGLVL: u64 = 8;
#[allow(dead_code)]
pub const ETHTOOL_SPAUSEPARAM: u64 = 19;
#[allow(dead_code)]
pub const ETHTOOL_SPFLAGS: u64 = 40;
#[allow(dead_code)]
pub const ETHTOOL_SRINGPARAM: u64 = 17;
#[allow(dead_code)]
pub const ETHTOOL_SRSSH: u64 = 71;
#[allow(dead_code)]
pub const ETHTOOL_SRXCLSRLDEL: u64 = 49;
#[allow(dead_code)]
pub const ETHTOOL_SRXCLSRLINS: u64 = 50;
#[allow(dead_code)]
pub const ETHTOOL_SRXCSUM: u64 = 21;
#[allow(dead_code)]
pub const ETHTOOL_SRXFH: u64 = 42;
#[allow(dead_code)]
pub const ETHTOOL_SRXFHINDIR: u64 = 57;
#[allow(dead_code)]
pub const ETHTOOL_SRXNTUPLE: u64 = 53;
#[allow(dead_code)]
pub const ETHTOOL_SSET: u64 = 2;
#[allow(dead_code)]
pub const ETHTOOL_SSG: u64 = 25;
#[allow(dead_code)]
pub const ETHTOOL_STSO: u64 = 31;
#[allow(dead_code)]
pub const ETHTOOL_STUNABLE: u64 = 73;
#[allow(dead_code)]
pub const ETHTOOL_STXCSUM: u64 = 23;
#[allow(dead_code)]
pub const ETHTOOL_SUFO: u64 = 34;
#[allow(dead_code)]
pub const ETHTOOL_SWOL: u64 = 6;
#[allow(dead_code)]
pub const ETHTOOL_TEST: u64 = 26;
#[allow(dead_code)]
pub const ETH_RX_NFC_IP4: u64 = 1;
#[allow(dead_code)]
pub const FIOGETOWN: u64 = 35075;
#[allow(dead_code)]
pub const FIOSETOWN: u64 = 35073;
#[allow(dead_code)]
pub const IFF_ATTACH_QUEUE: u64 = 512;
#[allow(dead_code)]
pub const IFF_DETACH_QUEUE: u64 = 1024;
#[allow(dead_code)]
pub const IFF_MULTI_QUEUE: u64 = 256;
#[allow(dead_code)]
pub const IFF_NOFILTER: u64 = 4096;
#[allow(dead_code)]
pub const IFF_NO_PI: u64 = 4096;
#[allow(dead_code)]
pub const IFF_ONE_QUEUE: u64 = 8192;
#[allow(dead_code)]
pub const IFF_PERSIST: u64 = 2048;
#[allow(dead_code)]
pub const IFF_TAP: u64 = 2;
#[allow(dead_code)]
pub const IFF_TUN: u64 = 1;
#[allow(dead_code)]
pub const IFF_TUN_EXCL: u64 = 32768;
#[allow(dead_code)]
pub const IFF_VNET_HDR: u64 = 16384;
#[allow(dead_code)]
pub const IFNAMSIZ: u64 = 16;
#[allow(dead_code)]
pub const IPPROTO_ICMP: u64 = 1;
#[allow(dead_code)]
pub const IPV4_FLOW: u64 = 16;
#[allow(dead_code)]
pub const IPV4_USER_FLOW: u64 = 13;
#[allow(dead_code)]
pub const IPV6_FLOW: u64 = 17;
#[allow(dead_code)]
pub const IPV6_USER_FLOW: u64 = 14;
#[allow(dead_code)]
pub const IP_USER_FLOW: u64 = 13;
#[allow(dead_code)]
pub const MAX_NUM_QUEUE: u64 = 4096;
#[allow(dead_code)]
pub const MSG_BATCH: u64 = 262144;
#[allow(dead_code)]
pub const MSG_CMSG_CLOEXEC: u64 = 1073741824;
#[allow(dead_code)]
pub const MSG_CONFIRM: u64 = 2048;
#[allow(dead_code)]
pub const MSG_DONTROUTE: u64 = 4;
#[allow(dead_code)]
pub const MSG_DONTWAIT: u64 = 64;
#[allow(dead_code)]
pub const MSG_EOR: u64 = 128;
#[allow(dead_code)]
pub const MSG_ERRQUEUE: u64 = 8192;
#[allow(dead_code)]
pub const MSG_FASTOPEN: u64 = 536870912;
#[allow(dead_code)]
pub const MSG_MORE: u64 = 32768;
#[allow(dead_code)]
pub const MSG_NOSIGNAL: u64 = 16384;
#[allow(dead_code)]
pub const MSG_OOB: u64 = 1;
#[allow(dead_code)]
pub const MSG_PEEK: u64 = 2;
#[allow(dead_code)]
pub const MSG_PROBE: u64 = 16;
#[allow(dead_code)]
pub const MSG_TRUNC: u64 = 32;
#[allow(dead_code)]
pub const MSG_WAITALL: u64 = 256;
#[allow(dead_code)]
pub const MSG_WAITFORONE: u64 = 65536;
#[allow(dead_code)]
pub const SCTP_V4_FLOW: u64 = 3;
#[allow(dead_code)]
pub const SCTP_V6_FLOW: u64 = 7;
#[allow(dead_code)]
pub const SHUT_RD: u64 = 0;
#[allow(dead_code)]
pub const SHUT_WR: u64 = 1;
#[allow(dead_code)]
pub const SIOCADDDLCI: u64 = 35200;
#[allow(dead_code)]
pub const SIOCADDMULTI: u64 = 35121;
#[allow(dead_code)]
pub const SIOCBONDCHANGEACTIVE: u64 = 35221;
#[allow(dead_code)]
pub const SIOCBONDENSLAVE: u64 = 35216;
#[allow(dead_code)]
pub const SIOCBONDINFOQUERY: u64 = 35220;
#[allow(dead_code)]
pub const SIOCBONDRELEASE: u64 = 35217;
#[allow(dead_code)]
pub const SIOCBONDSETHWADDR: u64 = 35218;
#[allow(dead_code)]
pub const SIOCBONDSLAVEINFOQUERY: u64 = 35219;
#[allow(dead_code)]
pub const SIOCBRADDBR: u64 = 35232;
#[allow(dead_code)]
pub const SIOCBRADDIF: u64 = 35234;
#[allow(dead_code)]
pub const SIOCBRDELBR: u64 = 35233;
#[allow(dead_code)]
pub const SIOCBRDELIF: u64 = 35235;
#[allow(dead_code)]
pub const SIOCDELDLCI: u64 = 35201;
#[allow(dead_code)]
pub const SIOCDELMULTI: u64 = 35122;
#[allow(dead_code)]
pub const SIOCDEVPRIVATE_BEG: u64 = 35312;
#[allow(dead_code)]
pub const SIOCDEVPRIVATE_END: u64 = 35327;
#[allow(dead_code)]
pub const SIOCDIFADDR: u64 = 35126;
#[allow(dead_code)]
pub const SIOCETHTOOL: u64 = 35142;
#[allow(dead_code)]
pub const SIOCGHWTSTAMP: u64 = 35249;
#[allow(dead_code)]
pub const SIOCGIFADDR: u64 = 35093;
#[allow(dead_code)]
pub const SIOCGIFBR: u64 = 35136;
#[allow(dead_code)]
pub const SIOCGIFBRDADDR: u64 = 35097;
#[allow(dead_code)]
pub const SIOCGIFCOUNT: u64 = 35128;
#[allow(dead_code)]
pub const SIOCGIFDSTADDR: u64 = 35095;
#[allow(dead_code)]
pub const SIOCGIFENCAP: u64 = 35109;
#[allow(dead_code)]
pub const SIOCGIFFLAGS: u64 = 35091;
#[allow(dead_code)]
pub const SIOCGIFHWADDR: u64 = 35111;
#[allow(dead_code)]
pub const SIOCGIFINDEX: u64 = 35123;
#[allow(dead_code)]
pub const SIOCGIFMAP: u64 = 35184;
#[allow(dead_code)]
pub const SIOCGIFMEM: u64 = 35103;
#[allow(dead_code)]
pub const SIOCGIFMETRIC: u64 = 35101;
#[allow(dead_code)]
pub const SIOCGIFMTU: u64 = 35105;
#[allow(dead_code)]
pub const SIOCGIFNAME: u64 = 35088;
#[allow(dead_code)]
pub const SIOCGIFNETMASK: u64 = 35099;
#[allow(dead_code)]
pub const SIOCGIFPFLAGS: u64 = 35125;
#[allow(dead_code)]
pub const SIOCGIFSLAVE: u64 = 35113;
#[allow(dead_code)]
pub const SIOCGIFTXQLEN: u64 = 35138;
#[allow(dead_code)]
pub const SIOCGMIIPHY: u64 = 35143;
#[allow(dead_code)]
pub const SIOCGMIIREG: u64 = 35144;
#[allow(dead_code)]
pub const SIOCGPGRP: u64 = 35076;
#[allow(dead_code)]
pub const SIOCGSKNS: u64 = 35148;
#[allow(dead_code)]
pub const SIOCINQ: u64 = 21531;
#[allow(dead_code)]
pub const SIOCOUTQ: u64 = 21521;
#[allow(dead_code)]
pub const SIOCOUTQNSD: u64 = 35147;
#[allow(dead_code)]
pub const SIOCPROTOPRIVATE_BEG: u64 = 35296;
#[allow(dead_code)]
pub const SIOCPROTOPRIVATE_END: u64 = 35311;
#[allow(dead_code)]
pub const SIOCSHWTSTAMP: u64 = 35248;
#[allow(dead_code)]
pub const SIOCSIFADDR: u64 = 35094;
#[allow(dead_code)]
pub const SIOCSIFBRDADDR: u64 = 35098;
#[allow(dead_code)]
pub const SIOCSIFDSTADDR: u64 = 35096;
#[allow(dead_code)]
pub const SIOCSIFENCAP: u64 = 35110;
#[allow(dead_code)]
pub const SIOCSIFFLAGS: u64 = 35092;
#[allow(dead_code)]
pub const SIOCSIFHWADDR: u64 = 35108;
#[allow(dead_code)]
pub const SIOCSIFHWBROADCAST: u64 = 35127;
#[allow(dead_code)]
pub const SIOCSIFLINK: u64 = 35089;
#[allow(dead_code)]
pub const SIOCSIFMAP: u64 = 35185;
#[allow(dead_code)]
pub const SIOCSIFMEM: u64 = 35104;
#[allow(dead_code)]
pub const SIOCSIFMETRIC: u64 = 35102;
#[allow(dead_code)]
pub const SIOCSIFMTU: u64 = 35106;
#[allow(dead_code)]
pub const SIOCSIFNAME: u64 = 35107;
#[allow(dead_code)]
pub const SIOCSIFNETMASK: u64 = 35100;
#[allow(dead_code)]
pub const SIOCSIFPFLAGS: u64 = 35124;
#[allow(dead_code)]
pub const SIOCSIFSLAVE: u64 = 35120;
#[allow(dead_code)]
pub const SIOCSIFTXQLEN: u64 = 35139;
#[allow(dead_code)]
pub const SIOCSMIIREG: u64 = 35145;
#[allow(dead_code)]
pub const SIOCSPGRP: u64 = 35074;
#[allow(dead_code)]
pub const SIOCWANDEV: u64 = 35146;
#[allow(dead_code)]
pub const SOCK_CLOEXEC: u64 = 524288;
#[allow(dead_code)]
pub const SOCK_DCCP: u64 = 6;
#[allow(dead_code)]
pub const SOCK_DGRAM: u64 = 2;
#[allow(dead_code)]
pub const SOCK_NONBLOCK: u64 = 2048;
#[allow(dead_code)]
pub const SOCK_PACKET: u64 = 10;
#[allow(dead_code)]
pub const SOCK_RAW: u64 = 3;
#[allow(dead_code)]
pub const SOCK_RDM: u64 = 4;
#[allow(dead_code)]
pub const SOCK_SEQPACKET: u64 = 5;
#[allow(dead_code)]
pub const SOCK_STREAM: u64 = 1;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_OPT_CMSG: u64 = 1024;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_OPT_ID: u64 = 128;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_OPT_TSONLY: u64 = 2048;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_RAW_HARDWARE: u64 = 64;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_RX_HARDWARE: u64 = 4;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_RX_SOFTWARE: u64 = 8;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_SOFTWARE: u64 = 16;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_SYS_HARDWARE: u64 = 32;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_TX_ACK: u64 = 512;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_TX_HARDWARE: u64 = 1;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_TX_SCHED: u64 = 256;
#[allow(dead_code)]
pub const SOF_TIMESTAMPING_TX_SOFTWARE: u64 = 2;

#[allow(dead_code)]
pub const MCAST_BLOCK_SOURCE: u64 = 43;
#[allow(dead_code)]
pub const MCAST_EXCLUDE: u64 = 0;
#[allow(dead_code)]
pub const MCAST_INCLUDE: u64 = 1;
#[allow(dead_code)]
pub const MCAST_JOIN_GROUP: u64 = 42;
#[allow(dead_code)]
pub const MCAST_JOIN_SOURCE_GROUP: u64 = 46;
#[allow(dead_code)]
pub const MCAST_LEAVE_GROUP: u64 = 45;
#[allow(dead_code)]
pub const MCAST_LEAVE_SOURCE_GROUP: u64 = 47;
#[allow(dead_code)]
pub const MCAST_MSFILTER: u64 = 48;
#[allow(dead_code)]
pub const MCAST_UNBLOCK_SOURCE: u64 = 44;

#[allow(dead_code)]
pub const ARPHRD_ETHER: u64 = 1;
#[allow(dead_code)]
pub const ARPHRD_FDDI: u64 = 774;
#[allow(dead_code)]
pub const ARPHRD_IEEE802: u64 = 6;
#[allow(dead_code)]
pub const ATF_COM: u64 = 2;
#[allow(dead_code)]
pub const ATF_DONTPUB: u64 = 64;
#[allow(dead_code)]
pub const ATF_NETMASK: u64 = 32;
#[allow(dead_code)]
pub const ATF_PERM: u64 = 4;
#[allow(dead_code)]
pub const ATF_PUBL: u64 = 8;
#[allow(dead_code)]
pub const ATF_USETRAILERS: u64 = 16;
#[allow(dead_code)]
pub const IPPROTO_IP: u64 = 0;
#[allow(dead_code)]
pub const IP_ADD_MEMBERSHIP: u64 = 35;
#[allow(dead_code)]
pub const IP_ADD_SOURCE_MEMBERSHIP: u64 = 39;
#[allow(dead_code)]
pub const IP_BIND_ADDRESS_NO_PORT: u64 = 24;
#[allow(dead_code)]
pub const IP_BLOCK_SOURCE: u64 = 38;
#[allow(dead_code)]
pub const IP_CHECKSUM: u64 = 23;
#[allow(dead_code)]
pub const IP_DROP_MEMBERSHIP: u64 = 36;
#[allow(dead_code)]
pub const IP_DROP_SOURCE_MEMBERSHIP: u64 = 40;
#[allow(dead_code)]
pub const IP_FREEBIND: u64 = 15;
#[allow(dead_code)]
pub const IP_HDRINCL: u64 = 3;
#[allow(dead_code)]
pub const IP_IPSEC_POLICY: u64 = 16;
#[allow(dead_code)]
pub const IP_MINTTL: u64 = 21;
#[allow(dead_code)]
pub const IP_MSFILTER: u64 = 41;
#[allow(dead_code)]
pub const IP_MTU: u64 = 14;
#[allow(dead_code)]
pub const IP_MTU_DISCOVER: u64 = 10;
#[allow(dead_code)]
pub const IP_MULTICAST_ALL: u64 = 49;
#[allow(dead_code)]
pub const IP_MULTICAST_IF: u64 = 32;
#[allow(dead_code)]
pub const IP_MULTICAST_LOOP: u64 = 34;
#[allow(dead_code)]
pub const IP_MULTICAST_TTL: u64 = 33;
#[allow(dead_code)]
pub const IP_NODEFRAG: u64 = 22;
#[allow(dead_code)]
pub const IP_OPTIONS: u64 = 4;
#[allow(dead_code)]
pub const IP_PASSSEC: u64 = 18;
#[allow(dead_code)]
pub const IP_PKTINFO: u64 = 8;
#[allow(dead_code)]
pub const IP_PKTOPTIONS: u64 = 9;
#[allow(dead_code)]
pub const IP_PMTUDISC_DO: u64 = 2;
#[allow(dead_code)]
pub const IP_PMTUDISC_DONT: u64 = 0;
#[allow(dead_code)]
pub const IP_PMTUDISC_INTERFACE: u64 = 4;
#[allow(dead_code)]
pub const IP_PMTUDISC_OMIT: u64 = 5;
#[allow(dead_code)]
pub const IP_PMTUDISC_PROBE: u64 = 3;
#[allow(dead_code)]
pub const IP_PMTUDISC_WANT: u64 = 1;
#[allow(dead_code)]
pub const IP_RECVERR: u64 = 11;
#[allow(dead_code)]
pub const IP_RECVOPTS: u64 = 6;
#[allow(dead_code)]
pub const IP_RECVORIGDSTADDR: u64 = 20;
#[allow(dead_code)]
pub const IP_RECVTOS: u64 = 13;
#[allow(dead_code)]
pub const IP_RECVTTL: u64 = 12;
#[allow(dead_code)]
pub const IP_RETOPTS: u64 = 7;
#[allow(dead_code)]
pub const IP_ROUTER_ALERT: u64 = 5;
#[allow(dead_code)]
pub const IP_TOS: u64 = 1;
#[allow(dead_code)]
pub const IP_TRANSPARENT: u64 = 19;
#[allow(dead_code)]
pub const IP_TTL: u64 = 2;
#[allow(dead_code)]
pub const IP_UNBLOCK_SOURCE: u64 = 37;
#[allow(dead_code)]
pub const IP_UNICAST_IF: u64 = 50;
#[allow(dead_code)]
pub const IP_XFRM_POLICY: u64 = 17;

#[allow(dead_code)]
pub const AF_NETBIOS: u64 = 17;

pub const INADDR_ANY: u32 = 0;
