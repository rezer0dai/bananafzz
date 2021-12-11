use std;

extern crate core;
use self::core::state::id::StateTableId;
use self::core::exec::id::CallTableId;

pub const FD_SIZE: usize = 4;
pub const SOCKET_FD_SIZE: usize = 4;

#[allow(dead_code)]
#[repr(u64)]
pub enum StateIds {
    FdGeneric = 0x1F0,
    FdSocket = 0x100,
}

impl From<StateTableId> for StateIds {
    fn from(id: StateTableId) -> StateIds {
        match id {
            StateTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
impl Into<StateTableId> for StateIds {
    fn into(self) -> StateTableId {
        StateTableId::Id(self as u64)
    }
}

/// WARNING : dont change order, just append!
///
/// - later you can regret it, as one month before you fuzz with different ids so now you can not
/// apply, gathered knowledge from before - genes / code-cov info, to you current fuzzing
#[allow(non_camel_case_types, dead_code)]
#[repr(u64)]
pub enum CallIds {
    dummy = 0,
    dup,

    close = 0x100,

    socket = 0x200,
    accept,
    bind,
    connect,
    listen,
    recv,
    recvfrom,
    send,
    sendto,
    shutdown,

    select,
    poll,

    getsockopt_inet_int = 0x300,
    setsockopt_inet_int,
    getsockopt_inet_buf,
    setsockopt_inet_buf,
    getsockopt_inet_opts,
    setsockopt_inet_opts,
    getsockopt_inet_mreq,
    setsockopt_inet_mreq,
    getsockopt_inet_mreqn,
    setsockopt_inet_mreqn,
    getsockopt_inet_mreqsrc,
    setsockopt_inet_mreqsrc,
    setsockopt_inet_msfilter,
    setsockopt_inet_MCAST_JOIN_GROUP,
    setsockopt_inet_MCAST_LEAVE_GROUP,
    setsockopt_inet_MCAST_MSFILTER,
    getsockopt_inet_pktinfo,
    setsockopt_inet_pktinfo,
    getsockopt_inet_mtu,
    setsockopt_inet_mtu,
    setsockopt_inet_group_source_req,
    setsockopt_sock_int,
    setsockopt_sock_void,
    ioctl_sock_inet_SIOCGIFADDR,
    ioctl_sock_inet_SIOCSIFADDR,
    ioctl_sock_inet_SIOCGIFBRDADDR,
    ioctl_sock_inet_SIOCSIFBRDADDR,
    ioctl_sock_inet_SIOCGIFNETMASK,
    ioctl_sock_inet_SIOCSIFNETMASK,
    ioctl_sock_inet_SIOCGIFDSTADDR,
    ioctl_sock_inet_SIOCSIFDSTADDR,
    ioctl_sock_inet_SIOCGIFPFLAGS,
    ioctl_sock_inet_SIOCSIFPFLAGS,
    ioctl_sock_inet_SIOCSIFFLAGS,
    ioctl_sock_ifreq,
    ioctl_sock_SIOCADDDLCI,
    ioctl_sock_SIOCDELDLCI,
    ioctl_sock_FIOSETOWN,
    ioctl_sock_SIOCSPGRP,
}
impl From<CallTableId> for CallIds {
    fn from(id: CallTableId) -> CallIds {
        match id {
            CallTableId::Id(id) => unsafe{ std::mem::transmute(id) },
        }
    }
}
impl Into<CallTableId> for CallIds {
    fn into(self) -> CallTableId {
        CallTableId::Id(self as u64)
    }
}
