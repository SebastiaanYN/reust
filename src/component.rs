use crate::Node;
use crate::TaskQueue;

pub trait Component {
    fn render(self, task_queue: &mut TaskQueue) -> Node;
}
