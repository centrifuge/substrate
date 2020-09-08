use super::*;
use frame_support::traits::MigrateAccount;

mod deprecated {
	use crate::Trait;
	use frame_support::{decl_module, decl_storage};
	decl_module! {
		pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call where origin: T::Origin {}
	}

	decl_storage! {
		trait Store for Module<T: Trait<I>, I: Instance=DefaultInstance> as Balances {
			pub IsUpgraded: bool;
		}
	}
}

pub fn migrate<T: Trait<I>, I: Instance>() {
	deprecated::IsUpgraded::<I>::kill();
	StorageVersion::<I>::put(Releases::V2_0_0)
}

impl<T: Trait<I>, I: Instance> MigrateAccount<T::AccountId> for Module<T, I> {
	fn migrate_account(account: &T::AccountId) {
		Locks::<T, I>::migrate_key_from_blake(account);
	}
}
