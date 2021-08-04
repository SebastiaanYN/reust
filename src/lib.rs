use wasm_bindgen::prelude::*;

mod component;
mod node;

use crate::component::Component;
use crate::node::Node;

const REUST: &str = "__reust";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct List;

impl Component for List {
    fn render(&self) -> Node {
        Node::element(
            "ul",
            &[],
            (1..=5)
                .map(|i| Node::element("li", &[], vec![Node::text(i)]))
                .collect(),
        )
    }
}

struct App;

impl Component for App {
    fn render(&self) -> Node {
        let mut clicks = 0;

        Node::element(
            "div",
            &[],
            vec![
                Node::element(
                    "h1",
                    &[("class", "header")],
                    vec![Node::text("My list of numbers")],
                ),
                List.render(),
                Node::element("button", &[], vec![Node::text("Click here!")]).add_event_listener(
                    "click",
                    move |_| {
                        clicks += 1;
                        console_log!("{}", clicks);
                    },
                ),
            ],
        )
    }
}

#[wasm_bindgen]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log!("Hello, World!");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document
        .get_element_by_id(REUST)
        .expect(&format!("should have element with {} id", REUST));

    div.append_child(&App.render().into())
        .expect("unable to mount app");
}
