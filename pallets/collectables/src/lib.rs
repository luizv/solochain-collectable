#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::tokens::fungible::{Inspect, Mutate};
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
        type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
    }

    // Allows easy access our Pallet's `Balance` type. Comes from `Fungible` interface.
    pub type BalanceOf<T> =
        <<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

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
        Created {
            owner: T::AccountId,
        },
        Transferred {
            from: T::AccountId,
            to: T::AccountId,
            kitty_id: [u8; 32],
        },
        PriceSet {
            owner: T::AccountId,
            kitty_id: [u8; 32],
            price: Option<BalanceOf<T>>,
        },
        Sold {
            buyer: T::AccountId,
            kitty_id: [u8; 32],
            price: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// There are too many kitties in the world
        TooManyKitties,

        /// The kitty already exists
        DuplicateKitty,

        /// The user has too many kitties
        TooManyOwned,

        /// The kitty does not exist
        NoKitty,

        /// Cannot transfer to self
        TransferToSelf,

        /// The owner does not match
        NotOwner,

        /// The kitty is not for sale
        NotForSale,

        /// Max price is too low
        MaxPriceTooLow,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        pub dna: [u8; 32],
        pub owner: T::AccountId,
        pub price: Option<BalanceOf<T>>,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let dna = Self::gen_dna();
            Self::mint(who, dna)?;
            Ok(())
        }

        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            kitty_id: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_transfer(who, to, kitty_id)?;
            Ok(())
        }

        pub fn set_price(
            origin: OriginFor<T>,
            kitty_id: [u8; 32],
            price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_set_price(who, kitty_id, price)?;
            Ok(())
        }

        pub fn buy_kitty(
            origin: OriginFor<T>,
            kitty_id: [u8; 32],
            max_price: BalanceOf<T>,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            Self::do_buy_kitty(buyer, kitty_id, max_price)?;
            Ok(())
        }
    }
}
