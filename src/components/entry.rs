use sycamore::prelude::*;

#[component(inline_props)]
pub fn Entry<G: Html>(cx: Scope, time: String, value: String) -> View<G> {
    view! { cx,
        div(class="entry"){
            p(class="entry-time") { (time.clone()) }
            p(class="entry-value"){ (value.clone()) }
        }
    }
}
