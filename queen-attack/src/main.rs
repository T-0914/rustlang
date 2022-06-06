use queen_attack::{ChessPosition, Queen};

fn main() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
    println!(
        "white_queen.can_attack(&black_queen): {:#?}",
        white_queen.can_attack(&black_queen)
    );
}
