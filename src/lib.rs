use wasm_bindgen::prelude::*;

mod component;
mod node;
mod reactive;
mod task_queue;

use crate::component::Component;
use crate::node::Node;
use crate::reactive::Reactive;
use crate::task_queue::TaskQueue;

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

struct List<T, I>
where
    T: ToString,
    I: IntoIterator<Item = T>,
{
    content: I,
}

impl<T, I> Component for List<T, I>
where
    T: ToString,
    I: IntoIterator<Item = T>,
{
    fn render(self, _: &mut TaskQueue) -> Node {
        Node::element(
            "ul",
            &[],
            self.content
                .into_iter()
                .map(|i| Node::element("li", &[], vec![Node::text(i)]))
                .collect(),
        )
    }
}

struct App;

impl Component for App {
    fn render(self, task_queue: &mut TaskQueue) -> Node {
        let mut clicks = Reactive::new(0);

        Node::element(
            "div",
            &[],
            vec![
                Node::element(
                    "h1",
                    &[("class", "header")],
                    vec![Node::text("My list of numbers")],
                ),
                List { content: (1..=10) }.render(task_queue),
                {
                    let mut rc = clicks.clone();

                    let el = Node::element("button", &[], vec![Node::text("Click here!")])
                        .add_event_listener("click", move |_| rc += 1);

                    {
                        let mut task_queue = task_queue.clone();

                        clicks.subscribe(move |count| {
                            let count = count.clone();

                            task_queue.queue(move || console_log!("{}", count));
                        });
                    }

                    el
                },
                {
                    let mut rc = clicks.clone();

                    let el = Node::element("button", &[], vec![Node::text("Click here #2!")])
                        .add_event_listener("click", move |_| rc += 2);

                    el
                },
            ],
        )
    }
}

#[wasm_bindgen]
pub fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log!("Hello, World!");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document
        .get_element_by_id(REUST)
        .expect(&format!("should have element with {} id", REUST));

    let mut task_queue = TaskQueue::new();
    div.append_child(&App.render(&mut task_queue).into())
        .expect("unable to mount app");
}
