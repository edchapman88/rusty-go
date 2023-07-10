use crate::{
    game::Turn,
    player::{Move, Player},
    policy::Policy,
};

#[derive(Debug)]
pub struct QPlayer {
    id: Turn,
    policy: Box<dyn Policy>,
}

impl QPlayer {
    // pub fn new(id: Turn) -> QPlayer {
    //     QPlayer {
    //         id,
    //         policy: QTable::new(states, actions),
    //     }
    // }
}

impl Player for QPlayer {
    fn make_move(&self, board: &crate::board::Board) -> Move {
        self.policy.make_action(&board.encode())
    }
}
