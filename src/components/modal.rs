use sycamore::prelude::*;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Prop)]
pub struct ModalProps<'a, G: Html> {
    children: Children<'a, G>,
    visibility: &'a Signal<bool>,
}

#[component]
pub fn Modal<'a, G: Html>(cx: Scope<'a>, props: ModalProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let visibility = props.visibility;
    let hide = |_| visibility.set(false);
    let style = create_memo(cx, move || {
        if !*visibility.get() {
            "display: none;"
        } else {
            "display: grid;"
        }
    });
    view! {
        cx,
        div(class="modal", style= style) {
            div(class="modal-content"){
                (children)
                div(class="modal-controls") {
                    button(class="btn-modal", on:click=hide){ "back" }
                }
            }
        }
    }
}

#[component]
pub fn HelpModal<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        article{
            h2 {"How to use Jwapp"}
            p { "Simply type in the textarea and click \"Add\" to add an entry. Your entries will be sorted and organized by time." }
            p { "Double click on an entry to edit it." }
            p { "Prefix a line with \"# \" to make it a heading." }
            p { "Surround anything within a paragraph with \"*\" to make it bold. " }
            p { "Surround anything within a paragraph with \"-\" to italicize it. " }
            p { "Surround anything within a paragraph with \"_\" to underline it. " }
        }
    }
}

#[component(inline_props)]
pub fn EditModal<G: Html>(cx: Scope, id: Uuid) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let (value, tags) = app_state.get_entry_data(id).unwrap();
    let v = create_signal(cx, value);
    let t = create_signal(cx, tags);
    let save = move |_| {
        app_state.set_entry_data(id, (*v.get()).clone(), (*t.get()).clone());
    };
    view! {
        cx,
        textarea(class="edit-textarea", maxlength=500, bind:value = v)
        input(type="text", bind:value=t, placeholder="tags")
        button(class="btn-save",on:click=save) { "Save" }
    }
}
