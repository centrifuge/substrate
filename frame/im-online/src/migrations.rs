use super::*;
use frame_support::Blake2_256;
use frame_support::traits::MigrateAccount;

pub fn migrate<T: Trait>() {
	let current_index = <pallet_session::Module<T>>::current_index();
	let key_count = Keys::<T>::get().len() as AuthIndex;
	for i in 0..key_count {
		ReceivedHeartbeats::migrate_keys::<Blake2_256, Blake2_256, _, _>(current_index, i);
	}
}

impl<T: Trait> MigrateAccount<T::AccountId> for Module<T> {
	fn migrate_account(a: &T::AccountId) {
		let current_index = <pallet_session::Module<T>>::current_index();
		if let Ok(v) = a.using_encoded(|mut d| T::ValidatorId::decode(&mut d)) {
			AuthoredBlocks::<T>::migrate_keys::<Blake2_256, Blake2_256, _, _>(current_index, v);
		}
	}
}
