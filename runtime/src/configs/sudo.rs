#![allow(unused_imports)]

use crate::*;

impl pallet_sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}
