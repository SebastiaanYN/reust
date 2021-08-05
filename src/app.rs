use crate::component::Component;
use crate::node::{element, text, Node};
use crate::reactive::Reactive;
use crate::task_queue::TaskQueue;

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
        element(
            "ul",
            &[],
            self.content
                .into_iter()
                .map(|i| element("li", &[], vec![text(i)]))
                .collect(),
        )
    }
}

pub struct App;

impl Component for App {
    fn render(self, task_queue: &mut TaskQueue) -> Node {
        let mut clicks = Reactive::new(0);

        clicks.subscribe(|count| console_log!("{}", count));

        element(
            "div",
            &[],
            vec![
                element(
                    "h1",
                    &[("class", "header")],
                    vec![text("My list of numbers")],
                ),
                List { content: (1..=10) }.render(task_queue),
                element(
                    "p",
                    &[],
                    vec![
                        text("Clicked "),
                        {
                            let el = text(clicks.value());

                            {
                                let el_clone = el.clone();
                                let mut task_queue_clone = task_queue.clone();

                                clicks.subscribe(move |count| {
                                    let count_clone = count.clone();
                                    let el_clone_clone = el_clone.clone();

                                    task_queue_clone
                                        .queue(move || el_clone_clone.set_text(count_clone));
                                });
                            }

                            el
                        },
                        text(" times"),
                    ],
                ),
                {
                    let el = element("button", &[], vec![text("Add 1")]);

                    {
                        let mut clicks_clone = clicks.clone();

                        el.add_event_listener("click", move |_| clicks_clone += 1);
                    }

                    el
                },
                {
                    let el = element("button", &[], vec![text("Add 2")]);

                    {
                        let mut clicks_clone = clicks.clone();

                        el.add_event_listener("click", move |_| clicks_clone += 2);
                    }

                    el
                },
            ],
        )
    }
}
