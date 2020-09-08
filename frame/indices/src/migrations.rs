use super::*;

pub fn migrate<T: Trait>() {
	use frame_support::migration::{StorageIterator, put_storage_value};
	for (key, value) in StorageIterator::<
		(T::AccountId, BalanceOf<T>)
	>::new(b"Indices", b"Accounts").drain() {
		put_storage_value(b"Indices", b"Accounts", &key, (value.0, value.1, false));
	}
}
