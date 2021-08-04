use crate::Node;

pub trait Component {
    fn render(&self) -> Node;
}
