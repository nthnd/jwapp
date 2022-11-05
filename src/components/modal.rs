use sycamore::prelude::*;

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
        h2 {"How to use Jwapp"}
        li { "Simply type in the textarea and click \"Add\" to add an entry. Your entries will be sorted and organized by time." }
        li { "Double click on an entry to edit it." }
        li { "Prefix a line with \"# \" to make it a heading." }
        li { "Surround anything within a paragraph with \"*\" to make it bold. " }
        li { "Surround anything within a paragraph with \"_\" to italicize it. " }
    }
}

#[component(inline_props)]
pub fn EditModal<'a, G: Html>(cx: Scope, value: RcSignal<String>) -> View<G> {
    let v = create_signal_from_rc(cx, value.get());
    let save = move |_| {
        value.set_rc(v.get());
    };
    view! {
        cx,
        textarea(class="edit-textarea", maxlength=500, bind:value = v)
        button(class="btn-save",on:click=save) { "Save" }
    }
}
