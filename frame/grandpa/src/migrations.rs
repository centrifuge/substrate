use super::*;
pub fn migrate<T: Trait>() {
	for i in 0..=CurrentSetId::get() {
		SetIdSession::migrate_key_from_blake(i);
	}
}
