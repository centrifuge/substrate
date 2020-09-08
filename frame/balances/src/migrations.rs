use super::*;
use frame_support::traits::MigrateAccount;

impl<T: Trait<I>, I: Instance> MigrateAccount<T::AccountId> for Module<T, I> {
	fn migrate_account(account: &T::AccountId) {
		Locks::<T, I>::migrate_key_from_blake(account);
	}
}
