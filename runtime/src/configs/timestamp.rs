use crate::*;
use pallet_timestamp::Config;

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}