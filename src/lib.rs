use wasm_bindgen::prelude::*;

mod node;
use node::Node;

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

fn list() -> Node<'static> {
    let numbers = vec![1, 2, 3, 4, 5];

    Node::element(
        "div",
        &[],
        &[],
        vec![
            Node::element(
                "h1",
                &[("class", "header")],
                &[],
                vec![Node::text("My list of numbers")],
            ),
            Node::element(
                "ul",
                &[("id", "my-list")],
                &[],
                numbers
                    .iter()
                    .map(|i| Node::element("li", &[], &[], vec![Node::text(i)]))
                    .collect(),
            ),
            Node::element(
                "button",
                &[],
                &[("click", &|| alert("click"))],
                vec![Node::text("Click here!")],
            ),
        ],
    )
}

#[wasm_bindgen]
pub fn main() {
    console_log!("Hello, World!");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document
        .get_element_by_id(REUST)
        .expect(&format!("should have element with {} id", REUST));

    let node = list().render(&document);
    div.append_child(&node).expect("unable to mount app");
}
