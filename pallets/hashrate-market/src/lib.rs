#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod types;
use types::*;

use frame_support::{
	pallet_prelude::*,
	traits::{
		Currency, LockableCurrency,
		ExistenceRequirement::KeepAlive,
	},
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency mechanism.
		type Currency: LockableCurrency<Self::AccountId>;

		/// The maximum length of a machine metadata stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Details of a machine.
	#[pallet::storage]
	#[pallet::getter(fn cacher)]
	pub(super) type Machine<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountOf<T>,
		Blake2_128Concat,
		UUID,
		MachineDetails<BoundedString<T>, BalanceOf<T>>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A machine was added.
		MachineAdded { owner: AccountOf<T>, id: UUID, metadata: BoundedString<T> },
		/// A machine was removed.
		MachineRemoved { owner: AccountOf<T>, id: UUID },
		/// A machine was removed.
		OfferMaked { owner: AccountOf<T>, id: UUID, price: BalanceOf<T> },
		/// A machine was removed.
		OfferCanceled { owner: AccountOf<T>, id: UUID },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// A string is too long.
		StringTooLong,
		/// The machine already exists.
		AlreadyExists,
		/// The given machine ID is unknown.
		Unknown,
		/// The machine status is not the expected status.
		IncorrectStatus,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn add_machine(origin: OriginFor<T>, id: UUID, metadata: BoundedString<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Machine::<T>::contains_key(&who, &id), Error::<T>::AlreadyExists);

			Machine::<T>::insert(
				&who,
				&id,
				MachineDetails {
					metadata: metadata.clone(),
					status: MachineStatus::Idle,
					price: None,
				},
			);

			Self::deposit_event(Event::<T>::MachineAdded { owner: who, id, metadata });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn remove_machine(origin: OriginFor<T>, id: UUID) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let machine = Machine::<T>::get(&who, &id).ok_or(Error::<T>::Unknown)?;
			ensure!(machine.status != MachineStatus::Renting, Error::<T>::IncorrectStatus);

			Machine::<T>::remove(&who, &id);

			Self::deposit_event(Event::<T>::MachineRemoved { owner: who, id });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn make_offer(origin: OriginFor<T>, id: UUID, price: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Machine::<T>::try_mutate(&who, &id, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(details.status == MachineStatus::Idle, Error::<T>::IncorrectStatus);

				details.status = MachineStatus::ForRent;
				details.price = Some(price);

				Self::deposit_event(Event::<T>::OfferMaked { owner: who.clone(), id, price });
				Ok(())
			})
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn cancel_offer(origin: OriginFor<T>, id: UUID) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Machine::<T>::try_mutate(&who, &id, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(details.status == MachineStatus::ForRent, Error::<T>::IncorrectStatus);

				details.status = MachineStatus::Idle;
				details.price = None;

				Self::deposit_event(Event::<T>::OfferCanceled { owner: who.clone(), id });
				Ok(())
			})
		}
	}
}
