#![allow(dead_code)]

mod d1;
mod d2;
mod d3;
mod d4;

fn main() {
    println!("{}", d4::main());
    // match d1::main() {
    //     None => println!("No Result"),
    //     Some(a) => println!("{}", a)
    // }
}
