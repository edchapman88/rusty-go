use crate::board::{Board, Position};
use std::fmt::Debug;

pub trait Player: Debug {
    fn make_move(&self, board: &Board) -> Move;
}

#[derive(Debug, Clone)]
pub enum Move {
    Play(Position),
    Pass,
}
