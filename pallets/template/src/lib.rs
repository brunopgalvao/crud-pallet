#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

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
	}

	#[pallet::storage]
	#[pallet::getter(fn something1)]
	pub type Something1<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Something2<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Something3<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type Something4<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type Something5<T> = StorageValue<_, u32, OptionQuery>;

	// TODO: Make this work!
	// #[pallet::storage]
	// pub type Something6<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

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
	}
}
