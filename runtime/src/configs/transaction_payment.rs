use crate::*;
use pallet_transaction_payment::Config;

parameter_types! {
	pub const TransactionByteFee: Balance = 1;
}

impl Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
	type TransactionByteFee = TransactionByteFee;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}