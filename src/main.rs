#![allow(dead_code)]
#![feature(iterator_fold_self)]

mod d6;

fn main() {
    println!("{:?}", d6::main());
    // match d1::main() {
    //     None => println!("No Result"),
    //     Some(a) => println!("{}", a)
    // }
}
