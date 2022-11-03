use sycamore::prelude::*;

#[component(inline_props)]
fn EntryLine<G: Html>(cx: Scope, line: String) -> View<G> {
    let mut line = line.clone();
    if line.starts_with("# ") {
        line = line.replace("# ", "");
        view! { cx,
            h3 { (line.clone()) }
        }
    } else {
        view! {
            cx,
            p{ (line.clone()) }
        }
    }
}

#[component(inline_props)]
pub fn Entry<G: Html>(cx: Scope, time: String, value: String) -> View<G> {
    let lines = create_signal(
        cx,
        value
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    view! { cx,
        div(class="entry"){
            p(class="entry-time") { "at "(time.clone()) }
            Indexed(iterable = &lines, view = |cx, line|
                view!{cx,
                    EntryLine(line = line)
                })
        }
    }
}
