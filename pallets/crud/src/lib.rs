#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;
use frame_support::{
    BoundedVec
};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Good practice to limit storage.
		#[pallet::constant]
		type MaxLengthName: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn number)]
	pub type Number<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn MyDefault3<T: Config>() -> u32 { 3 }

	#[pallet::storage]
	pub type NumberWithDefault<T> = StorageValue<_, u32, ValueQuery, MyDefault3<T>>;

	#[pallet::type_value]
	pub fn MyDefaultResult<T: Config>() -> Result<u32, Error<T>> { Ok(0) }
	#[pallet::storage]
	pub type NumberResultQuery<T> = StorageValue<_, u32, ResultQuery<Error::<T>::NoneValue>, MyDefaultResult<T>>;

	#[pallet::storage]
	pub type NumberOptionQuery<T> = StorageValue<_, u32, OptionQuery>;

	#[pallet::storage]
	pub type AccountData<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	// It is recommendable to set boundaries, for example the name is good if is a BoundedVec
	#[derive(Encode, Decode, Default, TypeInfo, MaxEncodedLen, PartialEqNoBound, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct Client<T: Config> {
		/// Client Id.
		pub id: u32,
		/// Name client.
		pub name: BoundedVec<u8, T::MaxLengthName>
	}

	#[pallet::storage]
	pub type ClientData<T: Config> = StorageValue<_, Client<T>, OptionQuery>;


	#[pallet::storage]
	pub type CountedMap<T> = CountedStorageMap<_, Blake2_128Concat, u32, u32>;

	#[pallet::storage]
	pub type SomeMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	pub type SomeDoubleMap<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, u32, 
		Blake2_128Concat, T::AccountId, 
		u32, 
		ValueQuery
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NumberStored { number: u32, who: T::AccountId },
		NumberWithDefaultStored { number: u32, who: T::AccountId },
		NumberResultQueryStored { number: u32, who: T::AccountId },
		NumberOptionQueryStored { number: u32, who: T::AccountId },
		AccountDataStored { account: T::AccountId, who: T::AccountId },
		ClientDataStored { client: u32, who: T::AccountId },
		SomeMapStored { account: T::AccountId, number: u32, who: T::AccountId }
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		NameTooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_number(origin: OriginFor<T>, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Number<T>>::put(number);
			<NumberWithDefault<T>>::put(number);
			Self::deposit_event(Event::NumberStored { number, who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_number_with_default(origin: OriginFor<T>, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<NumberWithDefault<T>>::put(number);
			Self::deposit_event(Event::NumberWithDefaultStored { number, who });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_number_result_query(origin: OriginFor<T>, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<NumberResultQuery<T>>::put(number);
			Self::deposit_event(Event::NumberResultQueryStored { number, who });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_number_option_query(origin: OriginFor<T>, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<NumberOptionQuery<T>>::put(number);
			Self::deposit_event(Event::NumberOptionQueryStored { number, who });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_account_data(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<AccountData<T>>::put(account.clone());
			Self::deposit_event(Event::AccountDataStored { account: account.clone(), who });
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_client_data(origin: OriginFor<T>, id: u32, name: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_name: BoundedVec<_, _> =
				name.try_into().map_err(|_| Error::<T>::NameTooLong)?;
			
			<ClientData<T>>::put(Client {id, name: bounded_name});
			Self::deposit_event(Event::ClientDataStored { client: id, who });
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_counted_map(origin: OriginFor<T>, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let index = <CountedMap<T>>::count();
			<CountedMap<T>>::set(index, Some(number));
			Self::deposit_event(Event::NumberStored { number, who });
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_some_map(origin: OriginFor<T>, account: T::AccountId, number: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<SomeMap<T>>::set(account.clone(), number);
			Self::deposit_event(Event::SomeMapStored { account: account.clone(), number, who });
			Ok(())
		}
	}
}
