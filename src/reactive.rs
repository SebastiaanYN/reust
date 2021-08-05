use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Listener<T> = Box<dyn FnMut(&T)>;

struct ReactiveState<T> {
    value: T,
    listeners: Vec<Listener<T>>,
}

#[derive(Clone)]
pub struct Reactive<T> {
    state: Rc<RefCell<ReactiveState<T>>>,
}

impl<T> Reactive<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(ReactiveState {
                value: value,
                listeners: vec![],
            })),
        }
    }

    pub fn set(&mut self, value: T) {
        for listener in self.state.borrow_mut().listeners.iter_mut() {
            listener(&value);
        }

        (*self.state.borrow_mut()).value = value;
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
