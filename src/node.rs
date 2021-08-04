use wasm_bindgen::{prelude::Closure, JsCast};

pub type Attribute<'a> = (&'a str, &'a str);
pub type EventListenerCallback = &'static (dyn Fn());
pub type EventListener = (&'static str, EventListenerCallback);

pub enum Node<'a> {
    Text(String),
    Element {
        name: &'a str,
        props: &'a [Attribute<'a>],
        events: &'static [EventListener],
        children: Vec<Node<'a>>,
    },
}

impl<'a> Node<'a> {
    pub fn text<T: ToString>(text: T) -> Node<'a> {
        Node::Text(text.to_string())
    }

    pub fn element(
        name: &'a str,
        props: &'a [Attribute<'a>],
        events: &'static [EventListener],
        children: Vec<Node<'a>>,
    ) -> Node<'a> {
        Node::Element {
            name,
            props,
            events,
            children,
        }
    }

    pub fn render(&self, document: &web_sys::Document) -> web_sys::Node {
        match self {
            Node::Text(txt) => web_sys::Node::from(document.create_text_node(&txt)),
            Node::Element {
                name,
                props,
                events,
                children,
            } => {
                let el = document.create_element(name).unwrap();

                for (name, value) in *props {
                    el.set_attribute(name, &value).unwrap();
                }

                for (name, handler) in *events {
                    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn()>);
                    // let closure = handler;

                    el.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
                        .unwrap();
                    closure.forget();
                }

                for child in children {
                    el.append_child(&child.render(document)).unwrap();
                }

                web_sys::Node::from(el)
            }
        }
    }
}
