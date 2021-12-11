extern crate core;

use self::core::generator::composite::ArgComposite;

use common::table::StateIds;
use args::wsl::*;

extern crate api;
use self::api::leafs::flag_leaf::Flag;

pub trait PollfdTemplate {
	fn pollfd() -> ArgComposite;
}
impl PollfdTemplate for ArgComposite {
	fn pollfd() -> ArgComposite {
		ArgComposite::new(
			8,
			"pollfd",
			vec![
				(0, rnd_fd(StateIds::FdGeneric.into())),
				(4, Box::new(Flag::new(0u16, (POLLIN | POLLPRI | POLLOUT | POLLRDHUP | POLLERR | POLLHUP | POLLNVAL | POLLRDNORM | POLLRDBAND | POLLWRNORM | POLLWRBAND) as u16))),
				(6, Box::new(Flag::new(0u16, (POLLIN | POLLPRI | POLLOUT | POLLRDHUP | POLLERR | POLLHUP | POLLNVAL | POLLRDNORM | POLLRDBAND | POLLWRNORM | POLLWRBAND) as u16))),
			])
	}
}
