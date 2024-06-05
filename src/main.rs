use std::env;
mod board;
mod piece;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut b = board::Board::new();
    println!("{}", b);
    b.set(&args[1]);
    println!("{}", b);
}
