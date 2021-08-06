use std::cell::RefCell;
use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

mod app;
mod component;
mod node;
mod reactive;
mod task_queue;

use crate::component::Component;
use crate::node::Node;
use crate::task_queue::TaskQueue;

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

    div.append_child(&app::App.render().into())
        .expect("unable to mount app");
}
