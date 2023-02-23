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
	#[pallet::getter(fn something1)]
	pub type Something1<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Something2<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn MyDefault3<T: Config>() -> u32 { 3 }

	#[pallet::storage]
	pub type Something3<T> = StorageValue<_, u32, ValueQuery, MyDefault3<T>>;

	// TODO: Get this to work
	// #[pallet::type_value]
	// pub fn MyDefault4<T: Config>() -> Result<u32, Error<T>> { Ok(0) }
	// #[pallet::storage]
	// pub type Something4<T> = StorageValue<_, u32, ResultQuery, MyDefault4<T>>;

	#[pallet::storage]
	pub type Something5<T> = StorageValue<_, u32, OptionQuery>;

	#[pallet::storage]
	pub type Something6<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;


	//Just to test to store a Client
	// It is recomendable to set boundaries, for example the name is good if is a BoundedVec
	#[derive(Encode, Decode, Default, TypeInfo, MaxEncodedLen, PartialEqNoBound, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct Client<T: Config> {
		/// Client Id.
		pub id: u32,
		/// Name client.
		pub name: BoundedVec<u8, T::MaxLengthName>
	}

	#[pallet::storage]
	pub(super) type Something7<T: Config> = StorageValue<_, Client<T>, OptionQuery>;

	#[pallet::storage]
	pub type SomeMap1<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	pub type SomeDoubleMap1<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, u32, 
		Blake2_128Concat, T::AccountId, 
		u32, 
		ValueQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
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
		pub fn do_something1(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something1<T>>::put(something);
			Self::deposit_event(Event::SomethingStored { something, who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something2(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something2<T>>::put(something);
			Self::deposit_event(Event::SomethingStored { something, who });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something3(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something3<T>>::put(something);
			Self::deposit_event(Event::SomethingStored { something, who });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something7(origin: OriginFor<T>, id: u32, name: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_name: BoundedVec<_, _> =
				name.try_into().map_err(|_| Error::<T>::NameTooLong)?;
			
			<Something7<T>>::put(Client {id, name: bounded_name});
			Self::deposit_event(Event::SomethingStored { something: id, who });
			Ok(())
		}

		// #[pallet::call_index(3)]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// pub fn do_something4(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		// 	<Something4<T>>::put(Ok(something));
		// 	Self::deposit_event(Event::SomethingStored { something, who });
		// 	Ok(())
		// }
	}
}
