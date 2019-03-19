/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs
use parity_codec::{Decode, Encode};
use rstd::prelude::*;
use runtime_primitives::traits::Hash;
use support::{
  decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
};
use system::ensure_signed;

// TODO: Rust enum, that can be saved in the chain?
const STATUS_PENDING: u16 = 0;
const STATUS_APPROVED: u16 = 1;
const STATUS_REJECTED: u16 = 2;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LeaveRecord<Hash> {
  id: Hash,
  data: Vec<u8>,
  status: u16,
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
    // The storage
    Record get(records): map T::Hash => LeaveRecord<T::Hash>;

    // Lookup contracts
    RecordOwner get(record_owner): map T::Hash => T::AccountId;
    RecordHash get(all_record_by_index): map u128 => T::Hash;
    RecordCount get(all_records_count): u128;

    UserRecords get(user_records): map T::AccountId => Vec<T::Hash>;

    // a nonce to generate random hash / id
    Nonce: u128;
  }
}

decl_module! {
  /// The module declaration.
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deposit_event<T>() = default;

    pub fn submit_record(origin, data: Vec<u8>) -> Result {
      let sender = ensure_signed(origin)?;

      // Overflow check, when the number of total records is running over 2^128 which is billions of years to go :)
      let all_records_count = Self::all_records_count();
      let new_all_records_count = all_records_count.checked_add(1)
          .ok_or("Overflow adding a new record into the system")?;

      // New record random hash/id
      let nonce = <Nonce<T>>::get();
      let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
          .using_encoded(<T as system::Trait>::Hashing::hash);

      // Just to make sure the new ID is unique which should be always as SHA256 collision is billions of years even with high speed brute-force
      // https://crypto.stackexchange.com/a/47810
      ensure!(!<Record<T>>::exists(random_hash), "Record hash collides, the hash is already existed.");

      let new_record = LeaveRecord {
          id: random_hash,
          data: data,
          status: STATUS_PENDING
      };

      <Record<T>>::insert(random_hash, new_record);
      <RecordOwner<T>>::insert(random_hash, &sender);

      <RecordHash<T>>::insert(all_records_count, random_hash);
      <RecordCount<T>>::put(new_all_records_count);

      let mut user_records = Self::user_records(&sender).clone();
      let user_records_count = user_records.len();
      // Overflow check
      user_records_count.checked_add(1).ok_or("Overflow adding a new record for the user")?;

      user_records.push(random_hash);
      <UserRecords<T>>::insert(&sender, user_records);

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
