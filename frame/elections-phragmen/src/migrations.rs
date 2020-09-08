use super::*;
use frame_support::traits::MigrateAccount;

mod deprecated {
	use super::*;
	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
	}
	decl_storage! {
		trait Store for Module<T: Trait> as PhragmenElection {
			/// Votes of a particular voter, with the round index of the votes.
			pub VotesOf get(fn votes_of): map hasher(opaque_blake2_256) T::AccountId => Vec<T::AccountId>;
			/// Locked stake of a voter.
			pub StakeOf get(fn stake_of): map hasher(opaque_blake2_256) T::AccountId => BalanceOf<T>;
		}
	}
}

impl<T: Trait> MigrateAccount<T::AccountId> for Module<T> {
	fn migrate_account(a: &T::AccountId) {
		// Note: only migrates the hasher, migration is completed in `migration.rs`
		if deprecated::StakeOf::<T>::contains_key(a) {
			let locked_balance: BalanceOf<T> = deprecated::StakeOf::<T>::get(a);
			let votes: Vec<T::AccountId> = deprecated::VotesOf::<T>::get(a);
			Voting::<T>::insert(a, (locked_balance, votes));
			deprecated::StakeOf::<T>::remove(a);
			deprecated::VotesOf::<T>::remove(a);
		}
	}
}
