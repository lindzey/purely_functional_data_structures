use std::rc::Rc;
use std::cmp::Ordering;

// Q: if I put the Clone trait on Tree here, is it unnecessary in future functions using a Tree?
//   => No; it seems to force all functions to also specify that trait.
#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    Node(T, Rc<Tree<T>>, Rc<Tree<T>>),
}
use self::Tree::{Empty, Node};

impl<T> Tree<T> {
    fn left(&self) -> Rc<Tree<T>>{
        match self {
            Empty => Rc::new(Empty),
            Node(_, left, _) => Rc::clone(left),
        }
    }
    fn right(&self) -> Rc<Tree<T>> {
        match self {
            Empty => Rc::new(Empty),
            Node(_, _, right) => Rc::clone(right),
        }
    }
}

/// The member function as given in Figure 2.9 of PFDS
pub fn member<T: Ord>(input: &T, tree: &Rc<Tree<T>>) -> bool {
    match tree.as_ref() {
        Empty => false,
        Node(val, _, _) => {
            match input.cmp(&val) {
                Ordering::Greater => member(input, &tree.right()),
                Ordering::Less => member(input, &tree.left()),
                Ordering::Equal => true,
            }
        },
    }
}

/// The insert function as given in Figure 2.9 of PFDS
/// 
/// # Examples
/// ```
/// let tree = pfds::tree::tree_from_vec(&[1,2,3,4]);
/// assert!(!pfds::tree::member(&5, &tree));
/// let new_tree = pfds::tree::insert(5, &tree);
/// assert!(pfds::tree::member(&5, &new_tree));
/// ```
pub fn insert<T: Ord + Clone>(input: T, tree: &Rc<Tree<T>>) -> Rc<Tree<T>> {
    match tree.as_ref() {
        Empty => Rc::new(Node(input, Rc::new(Empty), Rc::new(Empty))),
        Node(val, _, _) => {
            match input.cmp(&val) {
                Ordering::Less => Rc::new(Node(val.clone(), insert(input, &tree.left()), tree.right())),
                Ordering::Greater => Rc::new(Node(val.clone(), tree.left(), insert(input, &tree.right()))),
                Ordering:: Equal => Rc::clone(&tree),
            }
        },
    }
}

/// Helper function for easily constructing a Tree
/// 
/// # Examples
/// ```
/// let tree = pfds::tree::tree_from_vec(&[1,4,2,3,5]);
/// assert!(pfds::tree::member(&4, &tree));
/// assert!(!pfds::tree::member(&6, &tree));
/// ```
pub fn tree_from_vec<T: Ord + Clone>(input: &[T]) -> Rc<Tree<T>> {
    if input.is_empty() {
        Rc::new(Empty)
    } else {
        insert(input[0].clone(), &Rc::clone(&tree_from_vec(&input[1..])))
    }
}