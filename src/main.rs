mod components;
mod state;
mod theme;

use state::AppState;

use components::{input::Input, list::List, modal::Modal};
use sycamore::prelude::*;
use theme::Theme;
use web_sys::{console, window};

const LOCAL_STORAGE_KEY: &str = "journal_entries_sycamore";

fn main() {
    sycamore::render(|cx| {
        let storage = window().unwrap().local_storage().unwrap().unwrap();

        let app_state = AppState {
            entry_groups: if let Ok(Some(e)) = storage.get_item(LOCAL_STORAGE_KEY) {
                serde_json::from_str(&e).unwrap_or_else(|e| {
                    console::error_1(&e.to_string().into());
                    Default::default()
                })
            } else {
                console::log_1(&"No items in local storage".into());
                Default::default()
            },
        };
        provide_context(cx, app_state);

        create_effect(cx, move || {
            let app_state = use_context::<AppState>(cx);
            for group in app_state.entry_groups.get().iter() {
                group.1.track();
            }
            storage
                .set_item(
                    LOCAL_STORAGE_KEY,
                    &serde_json::to_string(app_state.entry_groups.get().as_ref()).unwrap(),
                )
                .unwrap();
        });

        let should_render = create_memo(cx, move || {
            let app_state = use_context::<AppState>(cx);
            !app_state.entry_groups.get().is_empty()
        });

        let should_show_help = create_signal(cx, true);
        let show_help = |_| {
            should_show_help.set(true);
        };
        view! {
            cx,
            nav() {
                a(href="#"){"Jwapp"}
                div(class="nav-btns"){
                    button(on:click=show_help, class="btn-help"){"Help"}
                    Theme()
                }

            }

            Modal(visibility = should_show_help, help = true)

            Input()
            (if *should_render.get() {
                view!{
                    cx,
                    List()
                }
            }else{
                view!{
                    cx,
                    p(class="list-group-empty") {"Start journaling..."}
                }
            })
        }
    });
}
