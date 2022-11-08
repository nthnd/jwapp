use crate::{state::EntryData, AppState};
use chrono::Utc;
use sycamore::prelude::*;
use uuid::Uuid;

#[component]
pub fn Input<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let current_item = create_signal(cx, String::new());
    let characters_typed = create_memo(cx, move || (*current_item.get()).len());

    let tags = create_signal(cx, String::new());
    let add = move |_| {
        if !current_item.get().trim().is_empty() {
            let now = Utc::now();
            let entry_data = EntryData {
                id: Uuid::new_v4(),
                time: now.time(),
                tags: create_rc_signal_from_rc(tags.get()),
                value: create_rc_signal_from_rc(current_item.get()),
            };
            app_state.insert_with_date(now.date_naive(), entry_data);
            current_item.modify().clear();
        }
    };

    view! {
       cx,
        div(class="input-area"){
            textarea(maxlength = 500, placeholder= "What's on your mind?", bind:value = current_item) {}
            input(type="text", bind:value = tags, placeholder="tags")
            div(class="input-menu"){
                button(on:click = add, class="btn-add" ) { "Add" }
                p() { (characters_typed.get()) "/500"}
            }
        }
    }
}
