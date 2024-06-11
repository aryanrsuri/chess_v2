mod board;
mod game;
mod piece;
fn main() {
    let mut g = game::Game::new();
    println!("\n   Terminal Chess Board!");
    while g.state {
        println!("{}\n", g.board);
        println!("   {:?} to move:", g.turn);
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read");
        g.pass(&response);
    }
}
