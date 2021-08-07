use proc_macro_hack::proc_macro_hack;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[proc_macro_hack(support_nested)]
pub use html_macro::html;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

mod app;
mod component;
mod node;
mod reactive;
mod task_queue;

// use crate::component::Component;
use crate::node::Node;
use crate::node::{element, text};
use crate::reactive::Reactive;
use crate::task_queue::TaskQueue;

macro_rules! cloned {
    ( ($($x:ident),*) => $y:expr ) => {
        {
            $(
                #[allow(unused_mut)]
                let mut $x = $x.clone();
            )*

            $y
        }
    };
}

const REUST: &str = "__reust";

thread_local! {
    pub static TASK_QUEUE: RefCell<TaskQueue> = RefCell::new(TaskQueue::new());
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

    // div.append_child(&app::App.render().into())
    //     .expect("unable to mount app");

    let mut counter = Reactive::new(0);

    counter.subscribe(|count| console_log!("{}", count));

    let app = html!(
        <div>
            <h1>"Counter"</h1>

            <p>"Count: " {counter}</p>

            <button @click={cloned!((counter) => move |_| counter += 1)}>"Add 1"</button>
            <button @click={cloned!((counter) => move |_| counter += 2)}>"Add 2"</button>
        </div>
    );

    div.append_child(&app.into()).expect("unable to mount app");
}
