mod board;
mod piece;
fn main() {
    let b = board::Board::new();
    println!("{}", b);
}
