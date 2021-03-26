#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{ensure_signed};
use sp_std::{vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		/// Organizations
		pub Projects get(fn projects): map hasher(blake2_128_concat) T::AccountId => u32;
		/// Organizations
		pub PublicKeys get(fn public_keys): map hasher(blake2_128_concat) u32 => Vec<u8>;
		/// Project Id
		pub NextProjectId get(fn next_project_id): u32;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// project Added. [projectId, who]
		ProjectAdded(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Project ID Overflow
		ProjectsIdOverflow,
        /// Project ID does not exist
        NotAvailableProjectId,
        /// Not the Owner of a Project
        NotTheOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Create a new Project
		#[weight = 1000]
        pub fn create_project(origin, owner: T::AccountId) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let mut project_id = Self::next_project_id();
            project_id = project_id.checked_add(1).ok_or(Error::<T>::ProjectsIdOverflow)?;
            NextProjectId::put(project_id);
            Projects::<T>::insert(owner.clone(), project_id);
			// Emit an event.
            Self::deposit_event(RawEvent::ProjectAdded(project_id, owner));
            Ok(())
        }

        /// Assign a pub key to a Project DID
		#[weight = 1000]
		pub fn set_project_key(origin, project_id: u32, pub_key: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
          	ensure!( Projects::<T>::contains_key(sender.clone()), Error::<T>::NotAvailableProjectId);
            let owned_project_id = Self::projects(sender);
          	ensure!( owned_project_id == project_id, Error::<T>::NotTheOwner);
            PublicKeys::insert(project_id, pub_key);
            Ok(())
        }
    }
}
