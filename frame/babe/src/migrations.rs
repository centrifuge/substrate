use super::*;

pub fn migrate<T: Trait>() {
	for i in 0..=SegmentIndex::get() {
		UnderConstruction::migrate_key_from_blake(i);
	}
}
