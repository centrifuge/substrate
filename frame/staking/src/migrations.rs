use super::*;

pub fn clear_slash_data<T: Trait>() {
	<UnappliedSlashes<T>>::remove_all();
	<SlashingSpans<T>>::remove_all();
	<SpanSlash<T>>::remove_all();
}
