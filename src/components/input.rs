use crate::{state::EntryData, AppState};
use chrono::Utc;
use sycamore::prelude::*;

#[component]
pub fn Input<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let current_item = create_signal(cx, String::new());
    let add = move |_| {
        if !current_item.get().trim().is_empty() {
            let now = Utc::now();
            let entry_data = EntryData {
                time: now.format("%H:%M").to_string(),
                value: (*current_item.get()).clone(),
            };
            app_state.insert_with_date(now.date().to_string(), entry_data);
            current_item.modify().clear();
        }
    };

    view! {
       cx,
       textarea( maxlength = 500, bind:value = current_item ) {}
       button( on:click = add ) { "Add" }
    }
}
