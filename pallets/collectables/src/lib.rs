#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
pub use pallet::*;
use scale_info::TypeInfo;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

    #[pallet::storage]
    pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

    #[pallet::storage]
    pub(super) type KittiesOwned<T: Config> = StorageMap<
        Key = T::AccountId,
        Value = BoundedVec<[u8; 32], ConstU32<100>>,
        QueryKind = ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Created { owner: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// There are too many kitties in the world
        TooManyKitties,

        /// The kitty already exists
        DuplicateKitty,

        /// The user has too many kitties
        TooManyOwned,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        pub dna: [u8; 32],
        pub owner: T::AccountId,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let dna = Self::gen_dna();
            Self::mint(who, dna)?;
            Ok(())
        }
    }
}
