use crate::{components::entry_group::EntryGroup, AppState};
use sycamore::prelude::*;

#[component]
pub fn List<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let groups = app_state
        .entry_groups
        .map(cx, |x| x.iter().map(|x| *x.0).collect());

    view! {
        cx,
        div(class="list-group"){
            Indexed(iterable=groups,
                view = |cx, group| view! {cx,
                EntryGroup(date = group)
                })
        }
    }
}
