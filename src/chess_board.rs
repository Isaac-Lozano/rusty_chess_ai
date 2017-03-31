use color::{Color, get_color_escape_code, get_color_reset_code};

use bitboard::{Bitboard, BitboardPiece};

use minimax::board::Board;
use minimax::Score;

macro_rules! try_move {
    // Putting $moves:expr in here makes it more
    // evident what's going on, however it's kinda redundant.
    ($moves:expr, $board:expr, $piece:expr, $x:expr, $y:expr) => {
        let file = $piece.file();
        let rank = $piece.rank();
        if (file as i32) + $x >= 0 && (file as i32) + $x < 6 &&
           (rank as i32) + $y >= 0 && (rank as i32) + $y < 8 &&
           $board.contains($piece.shift($x, $y))
        {
            $moves.push(ChessMove {
                from: $piece,
                to: $piece.shift($x, $y),
            });
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ChessMove {
    from: BitboardPiece,
    to: BitboardPiece,
}

impl ChessMove {
    fn flip_vertical(&mut self) {
        self.from = self.from.flip_vertical();
        self.to = self.to.flip_vertical();
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ChessBoard {
    pawns: Bitboard,
    knights: Bitboard,
    rooks: Bitboard,
    bishops: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
    enemies: Bitboard,
    allies: Bitboard,
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        /* NOTE: These look like they're flipped
         * horizontally, but this is intentional.
         */
        ChessBoard {
            pawns: Bitboard::new(0b00000000,
                                 0b11111111,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b11111111,
                                 0b00000000),
            rooks: Bitboard::new(0b10000001,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b10000001),
            knights: Bitboard::new(0b01000010,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b01000010),
            bishops: Bitboard::new(0b00100100,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00100100),
            queens: Bitboard::new(0b00001000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00001000),
            kings: Bitboard::new(0b00010000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00000000,
                                 0b00010000),
            enemies: Bitboard::new(0b11111111,
                                   0b11111111,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000,
                                   0b00000000),
            allies: Bitboard::new(0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b00000000,
                                  0b11111111,
                                  0b11111111),
        }
    }

    pub fn as_other(self) -> ChessBoard {
        ChessBoard {
            pawns: self.pawns.flip_vertical(),
            rooks: self.rooks.flip_vertical(),
            knights: self.knights.flip_vertical(),
            bishops: self.bishops.flip_vertical(),
            queens: self.queens.flip_vertical(),
            kings: self.kings.flip_vertical(),
            enemies: self.allies.flip_vertical(),
            allies: self.enemies.flip_vertical(),
        }
    }

    pub fn print(&self) {
        let mut is_white_space = true;

        for y in 0..8 {
            print!("{} ", 8 - y);

            for x in 0..8 {
                let piece = BitboardPiece::from_file_rank(x, 8 - y - 1);
                let background_color = if is_white_space {
                    Color::Blue
                } else {
                    Color::Black
                };

                if self.allies.contains(piece) {
                    print!("{}", get_color_escape_code(Color::Green, background_color));
                } else if self.enemies.contains(piece) {
                    print!("{}", get_color_escape_code(Color::Red, background_color));
                } else {
                    print!("{}", get_color_escape_code(Color::White, background_color));
                }

                if self.kings.contains(piece) {
                    print!(" K ");
                } else if self.knights.contains(piece) {
                    print!(" N ");
                } else if self.rooks.contains(piece) {
                    print!(" R ");
                } else if self.bishops.contains(piece) {
                    print!(" B ");
                } else if self.queens.contains(piece) {
                    print!(" Q ");
                } else if self.pawns.contains(piece) {
                    print!(" P ");
                } else {
                    print!("   ");
                }

                print!("{}", get_color_reset_code());
                is_white_space = !is_white_space;
            }

            println!();
            is_white_space = !is_white_space;
        }

        println!("   A  B  C  D  E  F  G  H");
    }

    pub fn move_from_str(&self, s: &str) -> Result<ChessMove, ()> {
        fn alpha_to_file(c: char) -> Option<u32> {
            let upper = match c.to_uppercase().next() {
                Some(ch) => ch,
                None => return None,
            };

            if upper < 'A' || upper > 'H' {
                None
            } else {
                Some(upper as u32 - 'A' as u32)
            }
        }

        let mut chars = s.chars();
        let from_file = chars.next().and_then(|c| alpha_to_file(c)).ok_or(())?;
        let from_rank = chars.next().and_then(|c| c.to_digit(10)).ok_or(())? - 1;
        let to_file = chars.next().and_then(|c| alpha_to_file(c)).ok_or(())?;
        let to_rank = chars.next().and_then(|c| c.to_digit(10)).ok_or(())? - 1;

        match chars.next() {
            Some(_) => return Err(()),
            None => {}
        }

        if from_file < 8 && from_rank < 8 && to_file < 8 && to_rank < 8 {
            Ok(ChessMove {
                from: BitboardPiece::from_file_rank(from_file as usize, from_rank as usize),
                to: BitboardPiece::from_file_rank(to_file as usize, to_rank as usize),
            })
        } else {
            Err(())
        }
    }

    fn gen_pawn_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for pawn in self.pawns.intersect(self.allies).pieces() {
            try_move!(moves, self.enemies, pawn, -1, 1);
            try_move!(moves,
                      self.enemies.union(self.allies).complement(),
                      pawn,
                      0,
                      1);
            try_move!(moves, self.enemies, pawn, 1, 1);
        }
        moves
    }

    fn gen_rook_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for rook in self.rooks.intersect(self.allies).pieces() {
            for to in self.allies
                .union(self.enemies)
                .horizontal_ray(rook)
                .union(self.allies.union(self.enemies).vertical_ray(rook))
                .intersect(self.allies.complement())
                .pieces() {
                moves.push(ChessMove {
                    from: rook,
                    to: to,
                });
            }
        }
        moves
    }

    fn gen_bishop_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for bishop in self.bishops.intersect(self.allies).pieces() {
            for to in self.allies
                .union(self.enemies)
                .diagonal_ray(bishop)
                .union(self.allies.union(self.enemies).anti_diagonal_ray(bishop))
                .intersect(self.allies.complement())
                .pieces() {
                moves.push(ChessMove {
                    from: bishop,
                    to: to,
                });
            }
        }
        moves
    }

    fn gen_queen_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for queen in self.queens.intersect(self.allies).pieces() {
            for to in self.allies
                .union(self.enemies)
                .horizontal_ray(queen)
                .union(self.allies.union(self.enemies).vertical_ray(queen))
                .union(self.allies.union(self.enemies).diagonal_ray(queen))
                .union(self.allies.union(self.enemies).anti_diagonal_ray(queen))
                .intersect(self.allies.complement())
                .pieces() {
                moves.push(ChessMove {
                    from: queen,
                    to: to,
                });
            }
        }
        moves
    }

    fn gen_knight_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for knight in self.knights.intersect(self.allies).pieces() {
            try_move!(moves, self.allies.complement(), knight, 2, 1);
            try_move!(moves, self.allies.complement(), knight, 1, 2);
            try_move!(moves, self.allies.complement(), knight, -1, 2);
            try_move!(moves, self.allies.complement(), knight, -2, 1);
            try_move!(moves, self.allies.complement(), knight, -2, -1);
            try_move!(moves, self.allies.complement(), knight, -1, -2);
            try_move!(moves, self.allies.complement(), knight, 1, -2);
            try_move!(moves, self.allies.complement(), knight, 2, -1);
        }
        moves
    }

    fn gen_king_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for king in self.kings.intersect(self.allies).pieces() {
            try_move!(moves, self.allies.complement(), king, 0, 1);
            try_move!(moves, self.allies.complement(), king, 0, -1);
            try_move!(moves, self.allies.complement(), king, -1, 0);
            try_move!(moves, self.allies.complement(), king, 1, 0);
            try_move!(moves, self.allies.complement(), king, 1, 1);
            try_move!(moves, self.allies.complement(), king, -1, -1);
            try_move!(moves, self.allies.complement(), king, -1, 1);
            try_move!(moves, self.allies.complement(), king, 1, -1);
        }
        moves
    }
}

impl Board for ChessBoard {
    type Move = ChessMove;

    fn gen_ally_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();
        moves.extend(self.gen_pawn_moves());
        moves.extend(self.gen_king_moves());
        moves.extend(self.gen_knight_moves());
        moves.extend(self.gen_bishop_moves());
        moves.extend(self.gen_queen_moves());
        moves.extend(self.gen_rook_moves());
        moves
    }

    fn gen_enemy_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();
        let flipped = self.as_other();

        moves.extend(flipped.gen_pawn_moves());
        moves.extend(flipped.gen_king_moves());
        moves.extend(flipped.gen_knight_moves());
        moves.extend(flipped.gen_bishop_moves());
        moves.extend(flipped.gen_queen_moves());
        moves.extend(flipped.gen_rook_moves());

        for mv in moves.iter_mut() {
            mv.flip_vertical();
        }

        moves
    }

    fn do_move(&mut self, mv: &Self::Move) {
        for board in vec![&mut self.pawns,
                          &mut self.rooks,
                          &mut self.knights,
                          &mut self.bishops,
                          &mut self.queens,
                          &mut self.kings,
                          &mut self.enemies,
                          &mut self.allies]
            .into_iter() {
            board.remove(mv.to);
            if board.contains(mv.from) {
                board.remove(mv.from);
                board.add(mv.to);
            }
        }
    }

    fn score(&self) -> Score {
        if self.kings.intersect(self.allies).is_empty() {
            Score::Lose
        } else if self.kings.intersect(self.enemies).is_empty() {
            Score::Win
        } else {
            /* King score is taken into account with above */
            let pawn_score = self.pawns.intersect(self.allies).num_pieces() as i32 -
                             self.pawns.intersect(self.enemies).num_pieces() as i32;
            let rook_score = self.rooks.intersect(self.allies).num_pieces() as i32 -
                             self.rooks.intersect(self.enemies).num_pieces() as i32;
            let knight_score = self.knights.intersect(self.allies).num_pieces() as i32 -
                               self.knights.intersect(self.enemies).num_pieces() as i32;
            let bishop_score = self.bishops.intersect(self.allies).num_pieces() as i32 -
                               self.bishops.intersect(self.enemies).num_pieces() as i32;
            let queen_score = self.queens.intersect(self.allies).num_pieces() as i32 -
                              self.queens.intersect(self.enemies).num_pieces() as i32;
            Score::Heuristic((pawn_score * 1) + (rook_score * 5) + (knight_score * 3) +
                             (bishop_score * 3) + (queen_score * 9))
        }
    }

    fn is_game_over(&self) -> bool {
        if self.kings.intersect(self.allies).is_empty() ||
           self.kings.intersect(self.enemies).is_empty() {
            true
        } else {
            false
        }
    }
}
