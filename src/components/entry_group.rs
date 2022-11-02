use sycamore::prelude::*;

use crate::components::entry::Entry;
use crate::state::AppState;

#[component(inline_props)]
pub fn EntryGroup<G: Html>(cx: Scope, date: String) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let cloned_date = date.clone();

    let entries = create_memo(cx, move || {
        (*app_state
            .entry_groups
            .get()
            .get(&cloned_date)
            .unwrap()
            .get())
        .clone()
    });

    view! {
        cx,
        h1 { (date.clone().replace('-', " ").replace("UTC", "")) }
        Indexed(iterable = entries,
            view = |cx, x| view!{ cx,
                Entry(time= x.time, value= x.value)
            })
    }
}
