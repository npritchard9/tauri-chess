use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, TS, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[ts(export)]
pub enum Color {
    White,
    Black,
    Empty,
}

#[derive(Debug, Clone, TS, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[ts(export)]
pub enum PieceName {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[derive(Debug, Clone, Copy, TS, PartialEq, Eq, Serialize, Deserialize)]
#[ts(export)]
pub struct Piece {
    pub name: PieceName,
    pub color: Color,
    pub rank: usize,
    pub file: usize,
    pub controlled_by: ControlledBy,
}

impl Piece {
    fn new(name: PieceName, color: Color, rank: usize, file: usize) -> Self {
        Piece {
            name,
            color,
            rank,
            file,
            controlled_by: ControlledBy { white: 0, black: 0 },
        }
    }
}

#[derive(Debug, Clone, TS, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ts(export)]
pub struct ControlledBy {
    pub white: u8,
    pub black: u8,
}

#[derive(Debug, Serialize, TS, Deserialize, Clone)]
#[ts(export)]
pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub turn: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut squares: [[Piece; 8]; 8] =
            [[Piece::new(PieceName::Empty, Color::Empty, 0, 0); 8]; 8];
        for r in 2..6 {
            for c in 0..8 {
                squares[r][c] = Piece::new(PieceName::Empty, Color::Empty, r, c);
            }
        }
        // squares[0] and squares[1] are black pieces
        squares[0][0] = Piece::new(PieceName::Rook, Color::Black, 0, 0);
        squares[0][1] = Piece::new(PieceName::Knight, Color::Black, 0, 1);
        squares[0][2] = Piece::new(PieceName::Bishop, Color::Black, 0, 2);
        squares[0][3] = Piece::new(PieceName::Queen, Color::Black, 0, 3);
        squares[0][4] = Piece::new(PieceName::King, Color::Black, 0, 4);
        squares[0][5] = Piece::new(PieceName::Bishop, Color::Black, 0, 5);
        squares[0][6] = Piece::new(PieceName::Knight, Color::Black, 0, 6);
        squares[0][7] = Piece::new(PieceName::Rook, Color::Black, 0, 7);
        for (i, square) in squares[1].iter_mut().enumerate() {
            *square = Piece::new(PieceName::Pawn, Color::Black, 1, i);
        }
        // squares[6] and squares[7] are black pieces
        squares[7][0] = Piece::new(PieceName::Rook, Color::White, 7, 0);
        squares[7][1] = Piece::new(PieceName::Knight, Color::White, 7, 1);
        squares[7][2] = Piece::new(PieceName::Bishop, Color::White, 7, 2);
        squares[7][3] = Piece::new(PieceName::Queen, Color::White, 7, 3);
        squares[7][4] = Piece::new(PieceName::King, Color::White, 7, 4);
        squares[7][5] = Piece::new(PieceName::Bishop, Color::White, 7, 5);
        squares[7][6] = Piece::new(PieceName::Knight, Color::White, 7, 6);
        squares[7][7] = Piece::new(PieceName::Rook, Color::White, 7, 7);
        for (i, square) in squares[6].iter_mut().enumerate() {
            *square = Piece::new(PieceName::Pawn, Color::White, 6, i);
        }
        Board { squares, turn: 0 }
    }

    pub fn get_legal_moves(&mut self, rank: usize, file: usize) -> Vec<(usize, usize)> {
        let mut available: Vec<(usize, usize)> = vec![];
        let p = self.squares[rank][file];
        match p.name {
            // king will need special cases depending on ControlledBy
            PieceName::King => {
                for i in 0..8 {
                    // we are at our own spot
                    if i == 4 {
                        continue;
                    }
                    //convert to i8 to make sure we avoid overflows
                    let nr = rank as i8 + ((i as i8 % 3) - 1);
                    let nf = file as i8 + ((i as i8 / 3) - 1);

                    if (0..8).contains(&nr) && (0..8).contains(&nf) {
                        let mut curr = self.squares[nr as usize][nf as usize];
                        match p.color {
                            Color::White => {
                                curr.controlled_by.white += 1;
                                if curr.controlled_by.black == 0 && curr.color != Color::White {
                                    available.push((nr as usize, nf as usize));
                                }
                            }
                            Color::Black => {
                                curr.controlled_by.black += 1;
                                if curr.controlled_by.white == 0 && curr.color != Color::Black {
                                    available.push((nr as usize, nf as usize));
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Queen => {
                let moves: [(i8, i8); 8] = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                // maybe add the next one after the loop stops bc that
                // would be the piece that you could take
                for m in moves {
                    let mut nr = p.rank as i8 + m.0;
                    let mut nf = p.file as i8 + m.1;

                    // check
                    while (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color == Color::Empty
                    {
                        available.push((nr as usize, nf as usize));
                        nr += m.0;
                        nf += m.1;
                    }
                    if (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color != p.color
                    {
                        available.push((nr as usize, nf as usize));
                    }
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Rook => {
                let moves: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

                for m in moves {
                    let mut nr = p.rank as i8 + m.0;
                    let mut nf = p.file as i8 + m.1;

                    while (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color == Color::Empty
                    {
                        available.push((nr as usize, nf as usize));
                        nr += m.0;
                        nf += m.1;
                    }
                    if (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color != p.color
                    {
                        available.push((nr as usize, nf as usize));
                    }
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Bishop => {
                let moves: [(i8, i8); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

                for m in moves {
                    let mut nr = p.rank as i8 + m.0;
                    let mut nf = p.file as i8 + m.1;

                    while (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color == Color::Empty
                    {
                        available.push((nr as usize, nf as usize));
                        nr += m.0;
                        nf += m.1;
                    }
                    if (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && self.squares[nr as usize][nf as usize].color != p.color
                    {
                        available.push((nr as usize, nf as usize));
                    }
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Knight => {
                let moves: [(i8, i8); 8] = [
                    (-1, -2),
                    (-2, -1),
                    (-2, 1),
                    (-1, 2),
                    (1, 2),
                    (2, 1),
                    (2, -1),
                    (1, -2),
                ];
                for m in moves {
                    let nr = p.rank as i8 + m.0;
                    let nf = p.file as i8 + m.1;
                    if (0..8).contains(&nr)
                        && (0..8).contains(&nf)
                        && (self.squares[nr as usize][nf as usize].color == Color::Empty
                            || self.squares[nr as usize][nf as usize].color != p.color)
                    {
                        available.push((nr as usize, nf as usize));
                    }
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Pawn => {
                match p.color {
                    Color::White => {
                        // check if pawn on starting square
                        if p.rank == 6 {
                            let first_move_nr = p.rank - 2;
                            if self.squares[first_move_nr][p.file].color == Color::Empty {
                                available.push((first_move_nr, p.file))
                            }
                        }
                        // white moves "up" in array
                        let nr = p.rank - 1;
                        if self.squares[nr][p.file].color == Color::Empty {
                            available.push((nr, p.file))
                        }
                        // if opposing piece is 1 step in front diagonally we can attack it
                        // check this logic
                        if p.file > 0 && self.squares[nr][p.file - 1].color == Color::Black {
                            available.push((nr, (p.file - 1)));
                        }
                        if p.file < 7 && self.squares[nr][p.file + 1].color == Color::Black {
                            available.push((nr, (p.file + 1)));
                        }
                    }
                    Color::Black => {
                        // check if pawn on starting square
                        if p.rank == 1 {
                            let first_move_nr = p.rank + 2;
                            if self.squares[first_move_nr][p.file].color == Color::Empty {
                                available.push((first_move_nr, p.file))
                            }
                        }
                        // black moves "down" in array
                        let nr = p.rank + 1;
                        if nr < 8 && self.squares[nr][p.file].color == Color::Empty {
                            available.push((nr, p.file))
                        }
                        // if opposing piece is 1 step in front diagonally we can attack it
                        // check this logic
                        if p.file < 7 && self.squares[nr][p.file + 1].color == Color::White {
                            // doesnt rerender everything obviously, but it needs to
                            available.push((nr, (p.file + 1)));
                        }
                        if p.file > 0 && self.squares[nr][p.file - 1].color == Color::White {
                            available.push((nr, (p.file - 1)));
                        }
                    }
                    // nothing?
                    Color::Empty => (),
                }
                println!("available moves: {:?}", &available);
                available
            }
            PieceName::Empty => {
                println!("Empty: {p:#?}");
                available
            }
        }
    }

    // return true if move was made, else false
    // white moves are odd
    // black moves are even
    pub fn make_move(
        &mut self,
        moves: Vec<(usize, usize)>,
        r1: usize,
        f1: usize,
        r2: usize,
        f2: usize,
    ) -> bool {
        // index into enum to see if correct turn
        if self.turn % 2 == self.squares[r1][f1].color as usize {
            //let moves = self.get_legal_moves(r1, f1);
            if moves.contains(&(r2, f2)) {
                println!("Moving to {}, {}", r2, f2);
                self.squares[r2][f2] = self.squares[r1][f1];
                self.squares[r2][f2].rank = r2;
                self.squares[r2][f2].file = f2;
                self.squares[r1][f1] = Piece::new(PieceName::Empty, Color::Empty, r1, f1);
                self.turn += 1;
                true
            } else {
                println!(
                    "CANT MAKE MOVE: Available moves: {:?}, your move: ({}, {})",
                    &moves, r2, f2
                );
                false
            }
        } else {
            false
        }
    }
}
