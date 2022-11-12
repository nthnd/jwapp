mod components;
mod state;
mod theme;

use state::AppState;

use components::{
    entry::Tag,
    input::Input,
    list::List,
    modal::{HelpModal, Modal},
};
use sycamore::prelude::*;
use theme::Theme;
use web_sys::{console, window};

const ENTRY_KEY: &str = "journal_entries_sycamore";
const FIRST_KEY: &str = "journal_first_time_sycamore";

fn main() {
    sycamore::render(|cx| {
        let app_state = AppState {
            first_time: {
                let storage = window().unwrap().local_storage().unwrap().unwrap();
                if let Ok(Some(e)) = storage.get_item(FIRST_KEY) {
                    serde_json::from_str(&e).unwrap_or_else(|e| {
                        console::error_1(&e.to_string().into());
                        true
                    })
                } else {
                    console::log_1(&"No items in local storage".into());
                    true
                }
            },
            filter: create_rc_signal(Vec::new()),
            entry_groups: {
                let storage = window().unwrap().local_storage().unwrap().unwrap();
                if let Ok(Some(e)) = storage.get_item(ENTRY_KEY) {
                    serde_json::from_str(&e).unwrap_or_else(|e| {
                        console::error_1(&e.to_string().into());
                        Default::default()
                    })
                } else {
                    console::log_1(&"No items in local storage".into());
                    Default::default()
                }
            },
        };
        provide_context(cx, app_state);

        create_effect(cx, move || {
            let storage = window().unwrap().local_storage().unwrap().unwrap();
            let app_state = use_context::<AppState>(cx);
            app_state
                .entry_groups
                .modify()
                .retain(|_, v| !v.get().is_empty());
            for group in app_state.entry_groups.get().iter() {
                group.1.track();
            }
            storage
                .set_item(
                    ENTRY_KEY,
                    &serde_json::to_string(app_state.entry_groups.get().as_ref()).unwrap(),
                )
                .unwrap();
        });

        let should_render = create_memo(cx, move || {
            let app_state = use_context::<AppState>(cx);
            !app_state.entry_groups.get().is_empty()
        });

        let should_show_help = create_signal(cx, {
            let storage = window().unwrap().local_storage().unwrap().unwrap();
            let app_state = use_context::<AppState>(cx);
            let value = app_state.first_time;
            storage
                .set_item(FIRST_KEY, &serde_json::to_string(&false).unwrap())
                .unwrap();
            value
        });

        let show_help = |_| {
            should_show_help.set(true);
        };
        let filters = create_memo(cx, move || {
            let app_state = use_context::<AppState>(cx);
            (*app_state.filter.get()).clone()
        });

        view! {
            cx,
            nav() {
                a(href="#"){"Jwapp"}
                div(class="nav-btns"){
                    button(on:click=show_help, class="btn-help"){"Help"}
                    Theme()
                }

            }

            Modal(visibility = should_show_help) {
                HelpModal()
            }

            Input()
            div(class="container-filters") {
                Indexed(iterable=filters, view = |cx, tag| view!{
                    cx,
                    Tag(tag=( *tag.get() ).clone())
                })
            }
            (if *should_render.get() {
                view!{ cx,
                    List()
                }
            }else{
                view!{ cx,
                    p(class="list-group-empty") {"Start journaling..."}
                }
            })
        }
    });
}
