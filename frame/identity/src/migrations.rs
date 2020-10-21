use super::*;
use frame_support::storage::migration::{put_storage_value, take_storage_value, StorageIterator};

// Change the storage name used by this pallet from `Sudo` to `Identity`.
//
// Since the format of the storage items themselves have not changed, we do not
// need to keep track of a storage version. If the runtime does not need to be
// upgraded, nothing here will happen anyway.
pub fn change_name_sudo_to_identity<T: Trait>() {
	sp_runtime::print("Migrating Identity.");

	for (hash, identity_of) in StorageIterator::<Registration<BalanceOf<T>>>::new(b"Sudo", b"IdentityOf").drain() {
		put_storage_value(b"Identity", b"IdentityOf", &hash, identity_of);
	}

	for (hash, super_of) in StorageIterator::<(T::AccountId, Data)>::new(b"Sudo", b"SuperOf").drain() {
		put_storage_value(b"Identity", b"SuperOf", &hash, super_of);
	}

	for (hash, subs_of) in StorageIterator::<(BalanceOf<T>, Vec<T::AccountId>)>::new(b"Sudo", b"SubsOf").drain() {
		put_storage_value(b"Identity", b"SubsOf", &hash, subs_of);
	}

	if let Some(registrars) = take_storage_value::<Vec<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>>>(b"Sudo", b"Registrars", &[]) {
		put_storage_value(b"Identity", b"Registrars", &[], registrars);
	}

	sp_runtime::print("Done Identity.");
}
