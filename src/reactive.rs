use crate::TASK_QUEUE;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Listener<T> = Box<dyn FnMut(&T)>;

struct ReactiveState<T> {
    value: T,
    listeners: Vec<Listener<T>>,
    update_scheduled: bool,
}

#[derive(Clone)]
pub struct Reactive<T> {
    state: Rc<RefCell<ReactiveState<T>>>,
}

impl<T: 'static + Clone> Reactive<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(ReactiveState {
                value: value,
                listeners: vec![],
                update_scheduled: false,
            })),
        }
    }

    pub fn set(&mut self, value: T) {
        self.state.borrow_mut().value = value;

        if !self.state.borrow().update_scheduled {
            self.state.borrow_mut().update_scheduled = true;

            TASK_QUEUE.with(|queue| {
                let state = self.state.clone();

                queue.borrow_mut().queue(move || {
                    let value = state.borrow().value.clone();

                    for listener in state.borrow_mut().listeners.iter_mut() {
                        listener(&value);
                    }

                    state.borrow_mut().update_scheduled = false;
                })
            });
        }
    }

    pub fn value(&self) -> Ref<'_, T> {
        Ref::map(self.state.borrow(), |state| &state.value)
    }

    pub fn subscribe(&mut self, listener: impl FnMut(&T) + 'static) {
        self.state.borrow_mut().listeners.push(Box::new(listener));
    }
}

macro_rules! add_operator {
    ($oper:ident, $method:ident, $assign:ident, $assign_method:ident) => {
        impl<T> std::ops::$assign<T> for Reactive<T>
        where
            T: 'static + Copy + std::ops::$oper<Output = T>,
        {
            fn $assign_method(&mut self, other: T) {
                let updated = std::ops::$oper::$method(*self.value(), other);
                self.set(updated);
            }
        }
    };
}

add_operator! { Add, add, AddAssign, add_assign }
add_operator! { BitAnd, bitand, BitAndAssign, bitand_assign }
add_operator! { BitOr, bitor, BitOrAssign, bitor_assign }
add_operator! { BitXor, bitxor, BitXorAssign, bitxor_assign }
add_operator! { Div, div, DivAssign, div_assign }
add_operator! { Rem, rem, RemAssign, rem_assign }
add_operator! { Shl, shl, ShlAssign, shl_assign }
add_operator! { Shr, shr, ShrAssign, shr_assign }
add_operator! { Sub, sub, SubAssign, sub_assign }
