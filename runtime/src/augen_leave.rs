/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs
use parity_codec::{Decode, Encode};
use rstd::prelude::*;
use runtime_primitives::traits::Hash;
use support::{
  decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
};
use system::ensure_signed;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LeaveRecord<Hash> {
  id: Hash,
  data: Vec<u8>,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        RecordSubmitted(AccountId, Hash),
    }
);

/// This module's storage items.
decl_storage! {
  trait Store for Module<T: Trait> as AugenLeave {
    Records get(records): map T::AccountId => LeaveRecord<T::Hash>;
    Nonce: u64; // a nonce to generate random hash / id
  }
}

decl_module! {
  /// The module declaration.
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deposit_event<T>() = default;

    pub fn submit_record(origin, data: Vec<u8>) -> Result {
      let sender = ensure_signed(origin)?;

      let random_hash = <T as system::Trait>::Hashing::hash_of(&0);
      let new_record = LeaveRecord {
        id: random_hash,
        data: data
      };

      <Records<T>>::insert(&sender, new_record);

      <Nonce<T>>::mutate(|n| *n += 1);
      Self::deposit_event(RawEvent::RecordSubmitted(sender, random_hash));

      Ok(())
    }
  }
}

// decl_event!(
//   /// An event in this module.
//   pub enum Event<T>
//   where
//     AccountId = <T as system::Trait>::AccountId,
//   {
//     // Just a dummy event.
//     // Event `Something` is declared with a parameter of the type `u32` and `AccountId`
//     // To emit this event, we call the deposit funtion, from our runtime funtions
//     SomethingStored(u32, AccountId),
//   }
// );

// TODO: Straight from the template and need to learn when capable
// /// tests for this module
// #[cfg(test)]
// mod tests {
//   use super::*;

//   use primitives::{Blake2Hasher, H256};
//   use runtime_io::with_externalities;
//   use runtime_primitives::{
//     testing::{Digest, DigestItem, Header},
//     traits::{BlakeTwo256, IdentityLookup},
//     BuildStorage,
//   };
//   use support::{assert_ok, impl_outer_origin};

//   impl_outer_origin! {
//     pub enum Origin for Test {}
//   }

//   // For testing the module, we construct most of a mock runtime. This means
//   // first constructing a configuration type (`Test`) which `impl`s each of the
//   // configuration traits of modules we want to use.
//   #[derive(Clone, Eq, PartialEq)]
//   pub struct Test;
//   impl system::Trait for Test {
//     type Origin = Origin;
//     type Index = u64;
//     type BlockNumber = u64;
//     type Hash = H256;
//     type Hashing = BlakeTwo256;
//     type Digest = Digest;
//     type AccountId = u64;
//     type Lookup = IdentityLookup<u64>;
//     type Header = Header;
//     type Event = ();
//     type Log = DigestItem;
//   }
//   impl Trait for Test {
//     type Event = ();
//   }
//   type TemplateModule = Module<Test>;

//   // This function basically just builds a genesis storage key/value store according to
//   // our desired mockup.
//   fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
//     system::GenesisConfig::<Test>::default()
//       .build_storage()
//       .unwrap()
//       .0
//       .into()
//   }

//   #[test]
//   fn it_works_for_default_value() {
//     with_externalities(&mut new_test_ext(), || {
//       // Just a dummy test for the dummy funtion `do_something`
//       // calling the `do_something` function with a value 42
//       assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
//       // asserting that the stored value is equal to what we stored
//       assert_eq!(TemplateModule::something(), Some(42));
//     });
//   }
// }
