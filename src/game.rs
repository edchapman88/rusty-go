use crate::board::{Board, BoardSize, Piece, Position};
use crate::player::{Move, Player};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub white: Box<dyn Player>,
    pub black: Box<dyn Player>,
    pub status: Status,
    pub pass_flag: Option<Turn>,
}

impl Game {
    pub fn new(size: BoardSize, white: Box<dyn Player>, black: Box<dyn Player>) -> Game {
        Game {
            board: Board::new(size),
            white,
            black,
            status: Status::Playing(Turn::Black),
            pass_flag: None,
        }
    }
    pub fn start(&mut self) {
        while let Status::Playing(turn) = &self.status {
            match turn {
                Turn::Black => {
                    if let Move::Play(pos) = self.black.make_move(&self.board) {
                        if let Some(Turn::Black) = self.pass_flag {
                            // black passed last turn, but white played continuing the game
                            // black has moved this time, so cancel the pass flag
                            self.pass_flag = None;
                        }
                        self.board.place(Piece::Black, pos);
                    } else {
                        // Black passes
                        if let Some(Turn::White) = self.pass_flag {
                            // Both Players passed
                            self.status = Status::GameOver;
                            break;
                        } else {
                            self.pass_flag = Some(Turn::Black)
                        }
                    }
                    self.status = Status::Playing(Turn::White);
                }
                Turn::White => {
                    if let Move::Play(pos) = self.white.make_move(&self.board) {
                        if let Some(Turn::White) = self.pass_flag {
                            // white passed last turn, but black played continuing the game
                            // white has moved this time, so cancel the pass flag
                            self.pass_flag = None;
                        }
                        self.board.place(Piece::White, pos);
                    } else {
                        // White passes
                        if let Some(Turn::Black) = self.pass_flag {
                            // Both Players passed
                            self.status = Status::GameOver;
                            break;
                        } else {
                            self.pass_flag = Some(Turn::White)
                        }
                    }
                    self.status = Status::Playing(Turn::Black);
                }
            }
        }

        println!("** Game Over **");
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    Playing(Turn),
    GameOver,
}

#[derive(Debug, Clone)]
pub enum Turn {
    White,
    Black,
}
