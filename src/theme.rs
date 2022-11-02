use sycamore::prelude::*;

macro_rules! css_colors {
    ($doc: ident, --$name: ident : #$color:tt;) => {
        $doc.style().set_property(format!("--{}", stringify!($name)).as_str(), format!("#{}", stringify!( $color ) ).as_str()).unwrap();
    };
    ($doc: ident, --$name:ident : #$color:tt; $(--$iname:ident : #$icolor: tt;)+) => {
        css_colors!($doc, --$name : #$color;);
        css_colors!($doc, $(--$iname : #$icolor;) +);
    }
}

#[component]
pub fn Theme<G: Html>(cx: Scope) -> View<G> {
    let device_theme = web_sys::window()
        .and_then(|x| x.match_media("(prefers-color-scheme: dark)").unwrap())
        .unwrap()
        .matches();

    let selected_theme = create_signal(cx, String::from("system"));

    create_effect(cx, move || {
        let doc = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
            .unwrap();

        //light

        if (*selected_theme.get() == "system" && !device_theme) || (*selected_theme.get() == "day")
        {
            css_colors! ( doc,
                --bg: #f5f5f5;
                --altbg: #d0d6db;
                --fg: #333333;
                --altfg: #555555;
                --border: #cccccc;
                --accent: #54c367;
                --altaccent: #adadce;
            );
        }
        // dark
        else if (*selected_theme.get() == "system" && device_theme)
            || (*selected_theme.get() == "night")
        {
            css_colors! ( doc,
                --bg: #292d3e;
                --altbg: #3b424e;
                --fg: #ffffff;
                --altfg: #bbbbbb;
                --border: #676a70;
                --accent: #34934a;
                --altaccent: #4b4d74;
            );
        }
    });
    view! {
        cx,
        div{
            label(for="theme"){"Theme: "}
            select(name="theme", id="theme", bind:value = selected_theme){
                option(value="system"){"System"}
                option(value="day"){"Day"}
                option(value="night"){"Night"}
            }
        }
    }
}
