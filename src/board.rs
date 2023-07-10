use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: BoardSize,
    pub pieces: Vec<Piece>,
}

impl Board {
    pub fn new(size: BoardSize) -> Board {
        Board {
            pieces: vec![Piece::Blank; size.dim().pow(2) as usize],
            size,
        }
    }

    pub fn at(&self, pos: &Position) -> Piece {
        let index = pos.index(&self.size);
        for (i, p) in self.pieces.iter().enumerate() {
            if i == index {
                return p.clone();
            }
        }
        panic!("index out of bounds of board")
    }

    pub fn place(&mut self, piece: Piece, pos: Position) -> bool {
        if let Piece::Blank = &self.at(&pos) {
            let index = pos.index(&self.size);
            for (i, p) in self.pieces.iter_mut().enumerate() {
                if i == index {
                    *p = piece;
                    return true;
                }
            }
            panic!("index out of bounds of board")
        } else {
            false
        }
    }

    pub fn encode(&self) -> BoardState {
        let mut s = String::new();
        for p in &self.pieces {
            s += &p.to_string();
        }
        BoardState { code: s }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("   ");

        // number row
        for i in 0..self.size.dim() {
            if i < 10 {
                string += &(i.to_string() + " ");
            } else {
                string += &((i - 10).to_string() + " ");
            }
        }
        string += "\n    ";

        // top border
        for _ in 0..(2 * self.size.dim() - 1) {
            string += "_"
        }
        string += "\n";

        // grid
        for j in 0..self.size.dim() {
            if j < 10 {
                string += &(j.to_string() + "  |");
            } else {
                string += &(j.to_string() + " |");
            }

            for i in 0..self.size.dim() {
                string += &(self.at(&Position::from((i, j))).to_string() + "|");
            }
            string += "\n"
        }

        // bottom border
        string += "    ";
        for _ in 0..(2 * self.size.dim() - 1) {
            string += "-"
        }

        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Blank,
    White,
    Black,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Blank => write!(f, "-"),
            Piece::White => write!(f, "@"),
            Piece::Black => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoardSize {
    ThreeByThree,
    NineByNine,
    ThirteenByThirteen,
}

impl BoardSize {
    pub fn dim(&self) -> usize {
        match self {
            BoardSize::ThreeByThree => 3,
            BoardSize::NineByNine => 9,
            BoardSize::ThirteenByThirteen => 13,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pos: (usize, usize),
}

impl Position {
    pub fn from(pos: (usize, usize)) -> Position {
        Position { pos }
    }
    pub fn index(&self, board_size: &BoardSize) -> usize {
        self.pos.0 + self.pos.1 * board_size.dim()
    }
}

#[derive(Debug, Clone)]
pub struct BoardState {
    pub code: String,
}
