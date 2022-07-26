// For better debugging (printout) support
use support::{ debug, dispatch };
use system::offchain;
use sp_runtime::transaction_validity::{
  TransactionValidity, TransactionLongevity, ValidTransaction, InvalidTransaction
};

pub trait Trait: timestamp::Trait + system::Trait {
  /// The overarching event type.
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
  type Call: From<Call<Self>>;

  type SubmitSignedTransaction: offchain::SubmitSignedTransaction<Self, <Self as Trait>::Call>;
  type SubmitUnsignedTransaction: offchain::SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;
}