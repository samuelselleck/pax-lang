pub mod checkbox;
pub mod ellipse;
pub mod frame;
pub mod group;
pub mod image;
pub mod path;
pub mod rectangle;
pub mod scroller;
pub mod text;

fn patch_if_needed<R: From<T>, T: Clone + PartialEq<R>>(old_state: &mut Option<R>, patch: &mut Option<R>, new_value: T) -> bool {
    let no_update = old_state.as_ref().is_some_and(|c|  new_value == *c);
    if !no_update {
        *patch = Some(new_value.clone().into());
        *old_state = Some(new_value.into());
        true
    } else {
        false
    }
}