use super::*;
use sp_runtime::traits::SaturatedConversion;

pub fn migrate<T: Trait>(accounts: Vec<T::AccountId>) {
	migrate_block_hash::<T>();
	migrate_accounts::<T>(accounts);

	// Remove the old `RuntimeUpgraded` storage entry.
	let mut runtime_upgraded_key = sp_io::hashing::twox_128(b"System").to_vec();
	runtime_upgraded_key.extend(&sp_io::hashing::twox_128(b"RuntimeUpgraded"));
	sp_io::storage::clear(&runtime_upgraded_key);
}

fn migrate_block_hash<T: Trait>() {
	// Number is current block - we obviously don't know that hash.
	// Number - 1 is the parent block, who hash we record in this block, but then that's already
	//  with the new storage so we don't migrate it.
	// Number - 2 is therefore the most recent block's hash that needs migrating.
	let block_num = Number::<T>::get();
	frame_support::runtime_print!("BlockNumber: {}", block_num.saturated_into::<u64>());
	BlockHash::<T>::migrate_key_from_blake(T::BlockNumber::zero());
	if block_num > One::one() {
		sp_runtime::print("🕊️  Migrating BlockHashes...");
		let mut n = block_num - One::one() - One::one();
		let mut migrations = 1;
		while !n.is_zero() {
			migrations += 1;
			if BlockHash::<T>::migrate_key_from_blake(n).is_none() {
				break;
			}
			n -= One::one();
		}
		frame_support::runtime_print!("🕊️  Done BlockHashes with {} migrations", migrations);
	} else {
		sp_runtime::print("🕊️  No BlockHashes to migrate...");
	}
}

fn migrate_accounts<T: Trait>(accounts: Vec<T::AccountId>) {
	sp_runtime::print("🕊️  Migrating Accounts...");
	let mut count = 0u32;
	for a in &accounts {
		if Account::<T>::migrate_key_from_blake(a).is_some() {
			// Inform other modules about the account.
			T::MigrateAccount::migrate_account(a);
			count += 1;
			if count % 1000 == 0 {
				sp_runtime::print(count);
			}
		}
	}
	sp_runtime::print(count);
	sp_runtime::print("🕊️  Done Accounts.");
}
