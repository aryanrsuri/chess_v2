mod board;
mod game;
mod piece;
fn main() {
    let mut b = board::Board::new();
    println!("Terminal Chess Board!");
    loop {
        println!("{}", b);
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read");
        if response == "surrender" {
            break;
        }
        b.set(&response);
    }
}
