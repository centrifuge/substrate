use super::*;

/// Update storage from v1.0.0 to v2.0.0
///
/// In old version the staking module has several issue about handling session delay, the
/// current era was always considered the active one.
///
/// After the migration the current era will still be considered the active one for the era of
/// the upgrade. And the delay issue will be fixed when planning the next era.
// * create:
//   * ActiveEraStart
//   * ErasRewardPoints
//   * ActiveEra
//   * ErasStakers
//   * ErasStakersClipped
//   * ErasValidatorPrefs
//   * ErasTotalStake
//   * ErasStartSessionIndex
// * translate StakingLedger
// * removal of:
//   * Stakers
//   * SlotStake
//   * CurrentElected
//   * CurrentEraStart
//   * CurrentEraStartSessionIndex
//   * CurrentEraPointsEarned
pub fn migrate<T: Trait>() {
	mod deprecated {
		/// Deprecated storages used for migration from v1.0.0 to v2.0.0 only.

		use crate::{Trait, BalanceOf, SessionIndex, Exposure, UnlockChunk};
		use codec::{Encode, Decode, HasCompact};
		use frame_support::{decl_module, decl_storage};
		use sp_std::prelude::*;

		type Moment = u64;

		/// Reward points of an era. Used to split era total payout between validators.
		#[derive(Encode, Decode, Default)]
		pub struct EraPoints {
			/// Total number of points. Equals the sum of reward points for each validator.
			pub total: u32,
			/// The reward points earned by a given validator. The index of this vec corresponds to the
			/// index into the current validator set.
			pub individual: Vec<u32>,
		}

		#[derive(Encode, Decode)]
		pub struct OldStakingLedger<AccountId, Balance: HasCompact> {
			pub stash: AccountId,
			#[codec(compact)]
			pub total: Balance,
			#[codec(compact)]
			pub active: Balance,
			pub unlocking: Vec<UnlockChunk<Balance>>,
		}

		decl_module! {
    		pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
		}

		decl_storage! {
			pub trait Store for Module<T: Trait> as Staking {
				pub SlotStake: BalanceOf<T>;

				/// The currently elected validator set keyed by stash account ID.
				pub CurrentElected: Vec<T::AccountId>;

				/// The start of the current era.
				pub CurrentEraStart: Moment;

				/// The session index at which the current era started.
				pub CurrentEraStartSessionIndex: SessionIndex;

				/// Rewards for the current era. Using indices of current elected set.
				pub CurrentEraPointsEarned: EraPoints;

				/// Nominators for a particular account that is in action right now. You can't iterate
				/// through validators here, but you can find them in the Session module.
				///
				/// This is keyed by the stash account.
				pub Stakers: map hasher(opaque_blake2_256) T::AccountId => Exposure<T::AccountId, BalanceOf<T>>;

				/// Old upgrade flag.
				pub IsUpgraded: bool;
			}
		}
	}

	deprecated::IsUpgraded::kill();
	let current_era_start_index = deprecated::CurrentEraStartSessionIndex::get();
	let current_era = <Module<T> as Store>::CurrentEra::get().unwrap_or(0);
	let current_era_start = deprecated::CurrentEraStart::get();
	<Module<T> as Store>::ErasStartSessionIndex::insert(current_era, current_era_start_index);
	<Module<T> as Store>::ActiveEra::put(ActiveEraInfo {
		index: current_era,
		start: Some(current_era_start),
	});

	let current_elected = deprecated::CurrentElected::<T>::get();
	let mut current_total_stake = <BalanceOf<T>>::zero();
	for validator in &current_elected {
		let exposure = deprecated::Stakers::<T>::get(validator);
		current_total_stake += exposure.total;
		<Module<T> as Store>::ErasStakers::insert(current_era, validator, &exposure);

		let mut exposure_clipped = exposure;
		let clipped_max_len = T::MaxNominatorRewardedPerValidator::get() as usize;
		if exposure_clipped.others.len() > clipped_max_len {
			exposure_clipped.others.sort_unstable_by(|a, b| a.value.cmp(&b.value).reverse());
			exposure_clipped.others.truncate(clipped_max_len);
		}
		<Module<T> as Store>::ErasStakersClipped::insert(current_era, validator, exposure_clipped);

		let pref = <Module<T> as Store>::Validators::get(validator);
		<Module<T> as Store>::ErasValidatorPrefs::insert(current_era, validator, pref);
	}
	<Module<T> as Store>::ErasTotalStake::insert(current_era, current_total_stake);

	let points = deprecated::CurrentEraPointsEarned::get();
	<Module<T> as Store>::ErasRewardPoints::insert(current_era, EraRewardPoints {
		total: points.total,
		individual: current_elected.iter().cloned().zip(points.individual.iter().cloned()).collect(),
	});

	let res = <Module<T> as Store>::Ledger::translate_values(
		|old: deprecated::OldStakingLedger<T::AccountId, BalanceOf<T>>| StakingLedger {
			stash: old.stash,
			total: old.total,
			active: old.active,
			unlocking: old.unlocking,
			claimed_rewards: vec![]
		}
	);
	if let Err(e) = res {
		frame_support::print("Encountered error in migration of Staking::Ledger map.");
		frame_support::print("The number of removed key/value is:");
		frame_support::print(e);
	}


	// Kill old storages
	deprecated::Stakers::<T>::remove_all();
	deprecated::SlotStake::<T>::kill();
	deprecated::CurrentElected::<T>::kill();
	deprecated::CurrentEraStart::kill();
	deprecated::CurrentEraStartSessionIndex::kill();
	deprecated::CurrentEraPointsEarned::kill();

	StorageVersion::put(Releases::V4_0_0);
}

use frame_support::traits::MigrateAccount;

impl<T: Trait> MigrateAccount<T::AccountId> for Module<T> {
	fn migrate_account(a: &T::AccountId) {
		frame_support::runtime_print!("🕊️  Migrating Staking Account '{:?}'", a);
		if let Some(controller) = Bonded::<T>::migrate_key_from_blake(a) {
			frame_support::runtime_print!(
				"Migrating Staking stash account '{:?}' with controller '{:?}'", a, controller);
			Ledger::<T>::migrate_key_from_blake(controller);
			Payee::<T>::migrate_key_from_blake(a);
			Validators::<T>::migrate_key_from_blake(a);
			Nominators::<T>::migrate_key_from_blake(a);
			SlashingSpans::<T>::migrate_key_from_blake(a);
		} else if let Some(StakingLedger { stash, .. }) = Ledger::<T>::migrate_key_from_blake(a) {
			frame_support::runtime_print!(
				"Migrating Staking controller account '{:?}' with stash '{:?}'", a, &stash);
			Bonded::<T>::migrate_key_from_blake(&stash);
			Payee::<T>::migrate_key_from_blake(&stash);
			Validators::<T>::migrate_key_from_blake(&stash);
			Nominators::<T>::migrate_key_from_blake(&stash);
			SlashingSpans::<T>::migrate_key_from_blake(&stash);
		}
		frame_support::runtime_print!("🕊️  Done Staking Account '{:?}'", a);
	}
}
