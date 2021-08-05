use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

type Task = Box<dyn FnMut()>;

struct TaskQueueState {
    resolved_promise: js_sys::Promise,
    update_scheduled: bool,
    queue: Vec<Task>,
}

#[derive(Clone)]
pub struct TaskQueue {
    state: Rc<RefCell<TaskQueueState>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(TaskQueueState {
                resolved_promise: js_sys::Promise::resolve(&JsValue::NULL),
                update_scheduled: false,
                queue: vec![],
            })),
        }
    }

    pub fn queue(&mut self, task: impl FnMut() + 'static) {
        self.state.borrow_mut().queue.push(Box::new(task));

        if !self.state.borrow().update_scheduled {
            self.state.borrow_mut().update_scheduled = true;

            self.schedule_run_tasks();
        }
    }

    fn schedule_run_tasks(&mut self) {
        let state = self.state.clone();

        let closure = Closure::wrap(Box::new(move |_| {
            let mut state = state.borrow_mut();

            for task in &mut state.queue {
                task();
            }

            state.update_scheduled = false;
            state.queue.clear();
        }) as Box<dyn FnMut(_)>);

        let _ = self.state.borrow().resolved_promise.then(&closure);
        closure.forget();
    }
}
