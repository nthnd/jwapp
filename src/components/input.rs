use crate::{state::EntryData, AppState};
use chrono::Utc;
use sycamore::prelude::*;

#[component]
pub fn Input<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let current_item = create_signal(cx, String::new());
    let characters_typed = create_memo(cx, move || current_item.clone().get().len());
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
        div(class="input-area"){
            textarea(maxlength = 500, placeholder= "What's on your mind?", bind:value = current_item) {}
            div(class="input-menu"){
                button(on:click = add, class="btn-add" ) { "Add" }
                p() { (characters_typed.get()) "/500"}
            }
        }
    }
}
