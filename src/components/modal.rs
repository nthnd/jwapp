use sycamore::prelude::*;

#[derive(Prop)]
pub struct ModalProps<'a> {
    visibility: &'a Signal<bool>,
    help: bool,
    #[builder(default)]
    value: String,
}

#[component]
pub fn Modal<'a, G: Html>(cx: Scope<'a>, prop: ModalProps<'a>) -> View<G> {
    let visibility = prop.visibility;

    let hide = |_| visibility.set(false);
    let value = create_ref(cx, prop.value);
    let style = create_memo(cx, || {
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
                div(class="modal-text"){
                    (if prop.help{
                        HelpModal(cx)
                    }else{
                        view!{
                            cx,
                            (value.clone())
                        }
                    })
                }
                div(class="modal-controls") {
                    button(class="btn-modal", on:click=hide){ "Ok" }
                }
            }
        }
    }
}

#[component]
fn HelpModal<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        h2 {"How to use Jwapp"}
        p {
            "Simply type in the textarea and click \"Add\" to add an entry. Your entries will be sorted and organized by time."
        }
        p {
            "Prefix a line with \"# \" to make it a heading."
        }
    }
}
