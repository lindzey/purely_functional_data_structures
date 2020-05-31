use std::fmt;
use std::rc::Rc;

// This follows the example in the Rust book, but adding generics.
// https://doc.rust-lang.org/book/ch15-04-rc.html
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}
use self::List::{Cons, Nil};

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nil => write!(f, "Nil"),
            Cons(car, cdr) => write!(f, "[{}, {}]", car, cdr),
        }
    }
}

/// Helper function for easily constructing a List.
pub fn list_from_vec<T: Clone>(input: &[T]) -> List<T> {
    if input.is_empty() {
        Nil
    } else {
        Cons(input[0].clone(), Rc::new(list_from_vec(&input[1..])))
    }
}
