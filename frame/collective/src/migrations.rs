use super::*;

pub fn migrate<T: Trait<I>, I: Instance>() {
	for p in Proposals::<T, I>::get().into_iter() {
		ProposalOf::<T, I>::migrate_key_from_blake(&p);
		Voting::<T, I>::migrate_key_from_blake(&p);
	}
}
