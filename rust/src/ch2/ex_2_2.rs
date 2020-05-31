use std::rc::Rc;

use crate::tree;
use tree::Tree;
use tree::Tree::{Empty};

pub fn exercise_2_2() {
    let empty_tree = Rc::new(Empty);
    let test_tree = tree::insert(1, &Rc::clone(&empty_tree));
    println!("Empty tree: {:?}", empty_tree);
    println!("Test tree: {:?}", test_tree);
    // Not a great idea, since simple binary search trees have their
    // worst performance when presented with an ordered list
    let big_tree = tree::tree_from_vec(&[1,2,3,4,5]);
    println!("Big tree: {:?}", big_tree);
    assert!(tree::member(&4, &big_tree));
    //let short_tree = Node(1, )

}