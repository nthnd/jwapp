use sycamore::prelude::*;

#[component(inline_props)]
pub fn Entry<G: Html>(cx: Scope, time: String, value: String) -> View<G> {
    view! { cx,
        h2 { (time.clone()) }
        p { (value.clone()) }
    }
}
