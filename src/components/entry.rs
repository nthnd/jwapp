use std::collections::HashSet;

use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use web_sys::console;

use crate::state::{AppState, EntryData};

use super::modal::{EditModal, Modal};

#[component(inline_props)]
pub fn Tag<G: Html>(cx: Scope, tag: String) -> View<G> {
    let tag = create_ref(cx, tag);
    let app_state = use_context::<AppState>(cx);
    let apply_filter = move |_| {
        console::log_1(&"tag clicked".into());
        if (*app_state.filter.get())
            .iter()
            .any(|x| (*x.get()).clone() == *tag)
        {
            app_state
                .filter
                .modify()
                .retain(|x| &(*x.get()).clone() != tag);
            console::log_1(&format!("{:?}", app_state.filter.get()).into());
        } else {
            app_state
                .filter
                .modify()
                .push(create_rc_signal(tag.clone()));
        }
    };
    view! {
        cx,
        div(class="entry-tag", on:click=apply_filter){(tag.clone())}
    }
}

#[component(inline_props)]
fn Tags<G: Html>(cx: Scope, tags: RcSignal<String>) -> View<G> {
    let tags = create_ref(cx, tags);
    let tags_vec = create_signal(
        cx,
        tags.get()
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>(),
    );
    view! {
        cx,
        div(class="container-entry-tags"){
            Indexed(iterable = tags_vec, view = |cx,x| view!{cx,
                Tag(tag=x)
            })
        }
    }
}

fn format_line(line: &str) -> String {
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
            '-' => {
                if tag_map.insert(c) {
                    new_line.push_str("<i>")
                } else {
                    new_line.push_str("</i>");
                    tag_map.remove(&c);
                }
            }
            '_' => {
                if tag_map.insert(c) {
                    new_line.push_str("<u>")
                } else {
                    new_line.push_str("</u>");
                    tag_map.remove(&c);
                }
            }
            _ => new_line.push(c),
        }
    }
    for c in tag_map {
        new_line.push_str(match c {
            '*' => "</b>",
            '-' => "</i>",
            '_' => "</u>",
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
            Tags(tags=entry_data.tags.clone())
            div(on:dblclick = edit, class="entry-content"){
                Indexed(iterable = lines, view = |cx, line|
                    view!{cx,
                        EntryLine(line = line)
                    })
            }
            Modal(visibility=editing) {
                EditModal(id=entry_data.id)
            }
        }
    }
}
