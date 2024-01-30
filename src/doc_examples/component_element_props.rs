// ANCHOR: all
#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App() -> Element {
    // ANCHOR: Clickable_usage
    rsx! {
        Clickable {
            href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            body: rsx!("How to " i {"not"} " be seen"),
        }
    }
    // ANCHOR_END: Clickable_usage
}

// ANCHOR: Clickable
#[derive(Props)]
struct ClickableProps {
    href: String,
    body: Element
}

fn Clickable(props: ClickableProps) -> Element {
    rsx!(
        a {
            href: "{cx.props.href}",
            class: "fancy-button",
            &cx.props.body
        }
    )
}
// ANCHOR_END: Clickable
