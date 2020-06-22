#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as PoeModule {
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransferred(AccountId, Vec<u8>, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// This proof has already been claimed
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it
		NotProofOwner,
		/// The proof is too large,
		ProofTooLarge,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;


		#[weight = 10_000]
		fn create_claim(origin, proof: Vec<u8>) {
			const MAX_PROOF_SIZE: usize = 9;
			let sender = ensure_signed(origin)?;
			ensure!(proof.len() <= MAX_PROOF_SIZE, Error::<T>::ProofTooLarge);
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
			let current_block = <system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof, (sender.clone(), current_block));
			Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
		}

		#[weight = 10_000]
		fn revoke_claim(origin, proof: Vec<u8>) {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			Proofs::<T>::remove(&proof);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
		}

		#[weight = 10_000]
		fn transfer_claim(origin, proof: Vec<u8>, receiver: T::AccountId) {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			let current_block = <system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof, (receiver.clone(), current_block));
			Self::deposit_event(RawEvent::ClaimTransferred(sender, proof, receiver));
		}
	}
}
