use std::rc::Rc;

extern crate pfds;
use pfds::*;
use list::List;
use list::List::{Nil, Cons};


fn list_suffixes<T>(list: &Rc<List<T>>) -> List<Rc<List<T>>> {
    match list.as_ref() {
        Nil => Nil,
        Cons(_, cdr) => Cons(Rc::clone(list), Rc::new(list_suffixes(&cdr))),
    }
}

pub fn exercise_2_1() {
    // The overall list needs to be Rc because some functions need a ref
    // to the whole list, not just components of it.
    let input = Rc::new(list::list_from_vec(&[1, 2, 3, 4]));
    let answer = list_suffixes(&input);
    // Ugh. If I want something like this to work, I'll need to implement more traits on List.
    // assert_eq!(answer, list_from_vec(&vec![[1,2,3,4], [2,3,4], [3,4], [4], []]));
    println!("Input for Ex 2.1: {:}", input);
    println!("Answer for Ex: 2.1: {:}", answer);
}

