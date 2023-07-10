use crate::{
    board::Position,
    game::Turn,
    player::{Move, Player},
};

#[derive(Debug, Clone)]
pub struct HumanPlayer {
    id: Turn,
}

impl HumanPlayer {
    pub fn new(id: Turn) -> HumanPlayer {
        HumanPlayer { id }
    }
}

impl Player for HumanPlayer {
    fn make_move(&self, board: &crate::board::Board) -> Move {
        println!("\n{:?}, make a move:", self.id);
        println!("\n {board}");
        println!("Enter a position: x,y ");
        println!("Or enter: pass");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // println!("{input}");
        input.pop();
        if input == "Pass" || input == "pass" {
            return Move::Pass;
        } else {
            let mut ij: Vec<u32> = input
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let j = ij.pop().expect("User should have provided x,y");
            let i = ij.pop().expect("User should have provided x,y");

            return Move::Play(Position::from((i as usize, j as usize)));
            // return Move::Play(Position::from((1, 2)));
        }
    }
}
