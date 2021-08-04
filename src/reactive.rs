use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Listener<'a, T> = &'a dyn Fn(&T);

struct ReactiveState<'a, T> {
    value: T,
    listeners: Vec<Listener<'a, T>>,
}

#[derive(Clone)]
pub struct Reactive<'a, T> {
    state: Rc<RefCell<ReactiveState<'a, T>>>,
}

impl<'a, T> Reactive<'a, T> {
    pub fn new(value: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(ReactiveState {
                value: value,
                listeners: vec![],
            })),
        }
    }

    pub fn set(&mut self, value: T) {
        (*self.state.borrow_mut()).value = value;

        for listener in self.state.borrow().listeners.iter() {
            listener(&self.state.borrow().value);
        }
    }

    pub fn value(&self) -> Ref<'_, T> {
        Ref::map(self.state.borrow(), |state| &state.value)
    }

    pub fn subscribe(&mut self, listener: Listener<'a, T>) {
        self.state.borrow_mut().listeners.push(listener);
    }
}

macro_rules! add_operator {
    ($oper:ident, $method:ident, $assign:ident, $assign_method:ident) => {
        impl<'a, T> std::ops::$assign<T> for Reactive<'a, T>
        where
            T: Copy + std::ops::$oper<Output = T>,
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
