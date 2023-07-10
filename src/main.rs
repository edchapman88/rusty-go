use rusty_go::{
    board::BoardSize,
    game::{Game, Turn},
    human_player::HumanPlayer,
};
fn main() {
    // let mut board = Board::new(BoardSize::NineByNine);
    // board.place(Piece::White, Position::from((1, 3)));
    // board.place(Piece::Black, Position::from((1, 4)));
    // println!("{board}");

    let mut game = Game::new(
        BoardSize::NineByNine,
        Box::new(HumanPlayer::new(Turn::White)),
        Box::new(HumanPlayer::new(Turn::Black)),
    );
    game.start();
}
