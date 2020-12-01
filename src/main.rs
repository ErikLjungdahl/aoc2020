mod d1;

fn main() {
    match d1::main() {
        None => println!("No Result"),
        Some(a) => println!("{}", a) 
    }
}
