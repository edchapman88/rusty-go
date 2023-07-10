use crate::{board::BoardState, player::Move};
use core::fmt::Debug;

pub trait Policy: Debug {
    fn make_action(&self, state: &BoardState) -> Move;
}
