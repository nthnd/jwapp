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
            "visibility: hidden;"
        } else {
            "visibility: visible;"
        }
    });
    view! {
        cx,
        div(class="modal", style= style) {
            div(class="modal-content"){
                div(class="modal-controls") {
                    (children)
                    button(class="btn-modal", on:click=hide){ "Ok" }
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
        p { "Simply type in the textarea and click \"Add\" to add an entry. Your entries will be sorted and organized by time." }
        p { "Prefix a line with \"# \" to make it a heading." }
    }
}

#[component(inline_props)]
pub fn EditModal<'a, G: Html>(cx: Scope, value: RcSignal<String>) -> View<G> {
    let value = create_ref(cx, value);
    view! {
        cx,
        div(class="input-area"){
            textarea(maxlength=500, bind:value = value )
        }
    }
}
