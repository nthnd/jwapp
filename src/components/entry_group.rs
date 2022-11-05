use chrono::NaiveDate;
use sycamore::prelude::*;

use crate::components::entry::Entry;
use crate::state::AppState;

#[component(inline_props)]
pub fn EntryGroup<G: Html>(cx: Scope, date: NaiveDate) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let date_str = date.format("%A %b %e, %Y");

    let entries = create_memo(cx, move || {
        (*app_state.entry_groups.get().get(&date).unwrap().get()).clone()
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
