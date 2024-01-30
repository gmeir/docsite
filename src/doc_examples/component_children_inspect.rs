// ANCHOR: all
#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App() -> Element {
    // ANCHOR: Clickable_usage
    rsx! {
        Clickable {
            href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            "How to " i {"not"} " be seen"
        }
    }
    // ANCHOR_END: Clickable_usage
}

#[derive(Props)]
struct ClickableProps<'a> {
    href: &'a str,
    children: Element<'a>,
}

// ANCHOR: Clickable
fn Clickable(props: ClickableProps) -> Element {
    match cx.props.children {
        Some(VNode { dynamic_nodes, .. }) => {
            todo!("render some stuff")
        }
        _ => {
            todo!("render some other stuff")
        }
    }
}
// ANCHOR_END: Clickable
