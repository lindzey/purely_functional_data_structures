// mod list;
// Rather than bringing the list.rs, tree.rs, etc files in one-by-one,
// I'd like to be able to use everything that's in lib.rs.
// use crate::*;  // Error: cannot glob-import a module into itself
// use super::*;  // Error: too many leading super keywords
// extern crate rust;  // Error: can't find crate for Rust

// In order for this to work, I needed to explicitly list 
// this library in the [lib] section of Cargo.toml
extern crate pfds;
use pfds::*;


// QUESTION: How to be able to directly call ch2::exercise_2_1();?
//    or even just exercise_2_1() without explicitly `use`ing every
//    file in the ch2 directory?
// use ch2::ex_2_1::*;
// use ch2::ex_2_2::*;
// extern crate rust;
mod ch2;
// Rather than calling the functions as ch2::ex2_1::exercise_2_1
// (module, filename, function)
// I'd like to be able to bring the functions directly into scope.
use ch2::*;  // This makes it possible to call ex_2_1::exercise_2_1().


fn main() {
    println!("Running exercises for Chapter 2:");
    ex_2_1::exercise_2_1();
    ex_2_2::exercise_2_2();
}
