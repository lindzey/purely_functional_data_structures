use std::rc::Rc;

#[derive(Debug)]
pub enum Heap<T> {
    Empty,
    Node(usize, T, Rc<Heap<T>>, Rc<Heap<T>>),
}
use self::Heap::{Empty, Node};

impl<T> Heap<T> {
    fn left(&self) -> Rc<Heap<T>> {
        match self {
            Empty => Rc::new(Empty),
            Node(_, _, left, _) => Rc::clone(left),
        }
    }
    fn right(&self) -> Rc<Heap<T>> {
        match self {
            Empty => Rc::new(Empty),
            Node(_, _, _, right) => Rc::clone(right),
        }
    }
}

pub fn merge<T>(heap1: &Rc<Heap<T>>, heap2: &Rc<Heap<T>>) -> Rc<Heap<T>> {
    unimplemented!();
}

fn rank<T>(heap: &Rc<Heap<T>>) -> usize {
    match heap.as_ref() {
        Empty => 0 as usize,
        Node(r, _, _, _) => *r,
    }
}

fn merge_helper<T: Clone>(val: &T, heap1: &Rc<Heap<T>>, heap2: &Rc<Heap<T>>) -> Rc<Heap<T>> {
    let r1 = rank(heap1);
    let r2 = rank(heap2);
    let node = if r1 >= r2 {
        Node(r2 + 1, val.clone(), Rc::clone(heap1), Rc::clone(heap2))
    } else {
        Node(r1 + 1, val.clone(), Rc::clone(heap2), Rc::clone(heap1))
    };
    Rc::new(node)
}

pub fn insert<T>(val: &T, heap: &Rc<Heap<T>>) {
    unimplemented!();
}

fn find_min<T>(heap: &Rc<Heap<T>>) -> Option<T> {
    unimplemented!();
}

fn delete_min<T>(heap: &Rc<Heap<T>>) -> Option<Rc<Heap<T>>> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_a_heap() {}
}
