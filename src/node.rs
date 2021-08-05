use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct Node {
    n: web_sys::Node,
}

impl From<web_sys::Element> for Node {
    fn from(el: web_sys::Element) -> Self {
        Self {
            n: web_sys::Node::from(el),
        }
    }
}

impl From<web_sys::Text> for Node {
    fn from(txt: web_sys::Text) -> Self {
        Self {
            n: web_sys::Node::from(txt),
        }
    }
}

impl From<Node> for web_sys::Node {
    fn from(node: Node) -> Self {
        node.n
    }
}

impl Node {
    pub fn set_text<T: ToString>(&self, txt: T) {
        self.n.set_text_content(Some(&txt.to_string()));
    }

    pub fn add_event_listener(
        &self,
        event_name: &str,
        handler: impl FnMut(web_sys::Event) + 'static,
    ) {
        let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

        self.n
            .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
            .unwrap();

        cb.forget();
    }
}

pub fn element(name: &str, props: &[(&str, &str)], children: Vec<Node>) -> Node {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let el = document.create_element(name).unwrap();

    for (name, value) in props {
        el.set_attribute(name, &value).unwrap();
    }

    for child in children {
        el.append_child(&child.n).unwrap();
    }

    el.into()
}

pub fn text<T: ToString>(txt: T) -> Node {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    document.create_text_node(&txt.to_string()).into()
}
