use crate::chess::{Chess, Outcome};

mod chess;
mod computer;
mod pos;

fn main() {
    let mut state = Chess::new();

    println!("{state}");

    while let Some(m) = computer::minimax(&state, 5).m {
        state.perform(m);
        println!("{state}");
    }

    match state.outcome().unwrap() {
        Outcome::Winner(color) => println!("{color} wins!"),
        Outcome::Stalemate => println!("it's a stalemate!"),
    }
}
