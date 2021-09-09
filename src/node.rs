use crate::Reactive;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct Node(web_sys::Node);

impl Node {
    pub fn append_children(&self, nodes: Vec<Node>) {
        for node in nodes {
            self.0
                .append_child(&node.into())
                .expect("unable to append child");
        }
    }

    pub fn set_text_content<T: ToString>(&self, txt: T) {
        self.0.set_text_content(Some(&txt.to_string()));
    }

    pub fn add_event_listener(
        &self,
        event_name: &str,
        handler: impl FnMut(web_sys::Event) + 'static,
    ) {
        let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

        self.0
            .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
            .unwrap();

        cb.forget();
    }
}

pub fn element(name: &str, props: &[(&str, &str)]) -> Node {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let el = document.create_element(name).unwrap();

    for (name, value) in props {
        el.set_attribute(name, &value).unwrap();
    }

    Node(web_sys::Node::from(el))
}

pub fn text<T: ToString>(txt: T) -> Node {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let el = document.create_text_node(&txt.to_string());

    Node(web_sys::Node::from(el))
}

impl From<Node> for web_sys::Node {
    fn from(node: Node) -> Self {
        node.0
    }
}

pub trait IntoNodes {
    fn as_nodes(&self) -> Vec<Node>;
}

impl IntoNodes for Node {
    fn as_nodes(&self) -> Vec<Node> {
        vec![self.clone()]
    }
}

impl<'a> IntoNodes for &'a str {
    fn as_nodes(&self) -> Vec<Node> {
        vec![text(self)]
    }
}

impl IntoNodes for String {
    fn as_nodes(&self) -> Vec<Node> {
        self.as_str().as_nodes()
    }
}

impl<T: IntoNodes> IntoNodes for Vec<T> {
    fn as_nodes(&self) -> Vec<Node> {
        self.into_iter()
            .map(IntoNodes::as_nodes)
            .flatten()
            .collect()
    }
}

impl<T: ToString> IntoNodes for Reactive<T> {
    fn as_nodes(&self) -> Vec<Node> {
        self.to_string().as_nodes()
    }
}
