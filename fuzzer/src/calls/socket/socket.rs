extern crate core;

use self::core::exec::call::Call;
use self::core::exec::fd_info::CallInfo;
use self::core::generator::arg::Arg;

use common::table::CallIds;

extern crate api;
use self::api::leafs::bounded_leaf::Bounded;

use args::wsl::*;

type TSocket = unsafe extern "system" fn(
    domain: u64,
    sock_type: u64,
    protocol: u64,
    ) -> i32;

lazy_static! {
    static ref SOCKET: TSocket = unsafe{ std::mem::transmute::<_, TSocket>(generic::load_api("./uow", "socket_")) };
}

pub trait SocketExec {
	fn socket() -> Call;
}
impl SocketExec for Call {
	fn socket() -> Call {
		Call::new(
			CallIds::socket.into(),
			"socket",
			vec![
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
						(AF_INET) as u64..=(AF_INET) as u64,
						(AF_INET6) as u64..=(AF_INET6) as u64,
//limiting logic due to toy example and not using code cov
/*
						(AF_NETBIOS) as u64..=(AF_NETBIOS) as u64,
						(AF_UNIX) as u64..=(AF_UNIX) as u64,
						(AF_PACKET) as u64..=(AF_PACKET) as u64,
						(AF_UNIX) as u64..=(AF_UNIX) as u64,
						(AF_NETLINK) as u64..=(AF_NETLINK) as u64,
						(AF_IPX) as u64..=(AF_IPX) as u64,
						(AF_X25) as u64..=(AF_X25) as u64,
						(AF_AX25) as u64..=(AF_AX25) as u64,
						(AF_ATMPVC) as u64..=(AF_ATMPVC) as u64,
						(AF_APPLETALK) as u64..=(AF_APPLETALK) as u64,
*/
				]))),
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
						(SOCK_STREAM) as u64..=(SOCK_STREAM) as u64,
						(SOCK_NONBLOCK) as u64..=(SOCK_NONBLOCK) as u64,
						(SOCK_RAW) as u64..=(SOCK_RAW) as u64,
//just for debug purposes, we limit angles of freedom, will need code cov approach otherwise, or
//hardcore logic ~ multiple socket call implementations etc
/*
						(SOCK_PACKET) as u64..=(SOCK_PACKET) as u64,
						(SOCK_DGRAM) as u64..=(SOCK_DGRAM) as u64,
						(SOCK_CLOEXEC) as u64..=(SOCK_CLOEXEC) as u64,

						(SOCK_SEQPACKET) as u64..=(SOCK_SEQPACKET) as u64,

						(SOCK_RDM) as u64..=(SOCK_RDM) as u64,
*/
				]))),
				Arg::primitive_arg(
					Box::new(Bounded::ranges(vec![
            0u64..=7,
						6..=6,//TCP
						IPPROTO_ICMP..=IPPROTO_ICMP,
//
//						(2) as u64..=(2) as u64,//IGMP
//						(7) as u64..=(10) as u64,//TCP
//						(17) as u64..=(17) as u64,//UDP
//						(58) as u64..=(58) as u64,//ICMPV6
//						113u64..=113u64,//IPPROTO_RM
				]))),
			], |args| {
        
        let status = if let [domain, sock_type, protocol] = &mut args[..] {
          unsafe { SOCKET(
            domain.data_const_unsafe::<u64>().clone(),
            sock_type.data_const_unsafe::<u64>().clone(),
            protocol.data_const_unsafe::<u64>().clone()) }
        } else { -1 };

        CallInfo::new(-1 != status, unsafe { generic::any_as_u8_slice(&status) })
      })
	}
}
