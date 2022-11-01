use crate::{components::entry_group::EntryGroup, AppState};
use sycamore::prelude::*;

use crate::state::EntryData;

#[component]
pub fn List<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let groups = create_memo(cx, || {
        app_state
            .entry_groups
            .get()
            .iter()
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect::<Vec<(String, RcSignal<Vec<EntryData>>)>>()
    });
    let should_render = create_memo(cx, || !app_state.entry_groups.get().is_empty());

    if *should_render.get() {
        view! {
            cx,
            Indexed(iterable=groups,
                view = |cx, group| view! {cx,
                    EntryGroup(date = group.0.clone())
                })
        }
    } else {
        view! {
            cx,
            p { "sorry, no notes "}
        }
    }
}
