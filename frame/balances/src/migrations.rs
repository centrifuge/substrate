use super::*;

impl<T: Trait<I>, I: Instance> Module<T, I> {
	pub fn add_balance(account :T::AccountId, balance :T::Balance) -> DispatchResult {
		Self::try_mutate_account(&account, |acc, is_new| -> DispatchResult {
			if !is_new{
				acc.free = acc.free.checked_add(&balance).ok_or(Error::<T, I>::Overflow)?;
			}
			Ok(())
		})
	}
}

