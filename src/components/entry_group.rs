use chrono::NaiveDate;
use sycamore::prelude::*;

use crate::components::entry::Entry;
use crate::state::AppState;

#[component(inline_props)]
pub fn EntryGroup<G: Html>(cx: Scope, date: NaiveDate) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let date_str = date.format("%A %b %e, %Y");

    let entries = create_memo(cx, move || {
        let mut es = (*app_state.entry_groups.get().get(&date).unwrap().get()).clone();
        if !app_state.filter.get().is_empty() {
            es.retain(|x| {
                app_state
                    .filter
                    .get()
                    .iter()
                    .map(|x| (*x.get()).clone())
                    .all(|t| (*x.tags.get()).split_whitespace().any(|x| x == t))
            });
        }
        es
    });

    view! {
        cx,
        section(class="entry-group"){
            h1 { (date_str) }
            Indexed(iterable = entries,
                view = |cx, x| view!{ cx,
                Entry(entry_data = x)
                })
        }
    }
}
