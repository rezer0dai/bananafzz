extern crate core;
use self::core::generator::composite::ArgComposite;

extern crate api;
use self::api::leafs::const_leaf::Const;
use self::api::leafs::bounded_leaf::Bounded;

use args::wsl::*;

pub trait SockAddrTemplate {
	fn sockaddr() -> ArgComposite;
}
impl SockAddrTemplate for ArgComposite {
	fn sockaddr() -> ArgComposite {
		ArgComposite::new(
			0x20,
			"sockaddr",
			vec![
        (0, Box::new(Bounded::ranges(vec![
						(AF_INET) as u16..=(AF_INET) as u16,
						(AF_INET6) as u16..=(AF_INET6) as u16,
//limiting logic due to toy example and not using code cov
/*
						(AF_NETBIOS) as u16..=(AF_NETBIOS) as u16,
						(AF_UNIX) as u16..=(AF_UNIX) as u16,
						(AF_PACKET) as u16..=(AF_PACKET) as u16,
						(AF_NETLINK) as u16..=(AF_NETLINK) as u16,
						(AF_UNIX) as u16..=(AF_UNIX) as u16,
						(AF_IPX) as u16..=(AF_IPX) as u16,
						(AF_X25) as u16..=(AF_X25) as u16,
						(AF_AX25) as u16..=(AF_AX25) as u16,
						(AF_ATMPVC) as u16..=(AF_ATMPVC) as u16,
						(AF_APPLETALK) as u16..=(AF_APPLETALK) as u16,
*/
          ]))),
				(2, Box::new(Bounded::one(6666..=6679u16))),
				(4, Box::new(Const::new32(INADDR_ANY))),
			])
	}
}
