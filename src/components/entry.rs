use std::collections::HashSet;

use sycamore::builder::prelude::*;
use sycamore::prelude::*;

use crate::state::{AppState, EntryData};

use super::modal::{EditModal, Modal};

fn format_line(line: &String) -> String {
    let mut tag_map: HashSet<char> = HashSet::new();
    let mut new_line = String::new();
    for c in line.chars() {
        match c {
            '*' => {
                if tag_map.insert(c) {
                    new_line.push_str("<b>")
                } else {
                    new_line.push_str("</b>");
                    tag_map.remove(&c);
                }
            }
            '_' => {
                if tag_map.insert(c) {
                    new_line.push_str("<i>")
                } else {
                    new_line.push_str("</i>");
                    tag_map.remove(&c);
                }
            }

            _ => new_line.push(c),
        }
    }
    for c in tag_map {
        new_line.push_str(match c {
            '*' => "</b>",
            '_' => "</i>",
            _ => "",
        });
    }
    new_line
}
#[component(inline_props)]
fn EntryLine<G: Html>(cx: Scope, line: String) -> View<G> {
    let mut line = line;
    if line.starts_with("# ") {
        line = line.replace("# ", "");
        view! { cx,
            h3 { (line.clone()) }
        }
    } else if line.starts_with("## ") {
        line = line.replace("## ", "");
        view! { cx,
            h4 { (line.clone()) }
        }
    } else {
        p().dangerously_set_inner_html(format_line(&line)).view(cx)
    }
}

#[component(inline_props)]
pub fn Entry<G: Html>(cx: Scope, entry_data: EntryData) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let entry_data = create_ref(cx, entry_data);
    let lines = entry_data
        .value
        .map(cx, |x| x.lines().map(|l| l.to_string()).collect());
    let time = format!("at {}", entry_data.time.format("%R"));
    let editing = create_signal(cx, false);
    let edit = |_| editing.set(true);
    let delete = |_| {
        app_state.delete_entry(entry_data.id);
    };

    view! { cx,
        div(class="entry"){
            div(class="entry-top"){
                p(class="entry-time") { (time)}
                button(on:click=delete, class="btn-delete") { "delete" }
            }
            div(on:dblclick = edit, class="entry-content"){
                Indexed(iterable = lines, view = |cx, line|
                    view!{cx,
                        EntryLine(line = line)
                    })
            }
            Modal(visibility=editing) {
                EditModal(value=entry_data.value.clone())
            }
        }
    }
}
