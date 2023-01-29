extern crate minimax;
extern crate bitboard;

mod color;
mod chess_board;

use chess_board::ChessBoard;

use minimax::board::Board;
use minimax::{Minimax, Score, Team};

use std::io::{self, Write};
use std::num::NonZeroUsize;

fn main() {
    println!("Sorry, I'm a bit rusty at this game. Forgive me.");

    let mut minimax = Minimax::new(NonZeroUsize::new(1000000).unwrap());
    let mut board = ChessBoard::new();
    let mut turn = Team::Ally;

    loop {
        println!("\nCurrent board state:");
        board.print();

        if board.is_game_over() {
            let team = match board.score() {
                Score::Win => Team::Ally,
                Score::Lose => Team::Enemy,
                _ => unreachable!(),
            };
            println!("{:?} wins!", team);
            break;
        }

        let moves = match turn {
            Team::Ally => board.gen_ally_moves(),
            Team::Enemy => board.gen_enemy_moves(),
        };
        if moves.len() == 0 {
            println!("No more moves, {:?} wins!", turn);
            break;
        }

        match turn {
            Team::Ally => {
                loop {
                    let mut move_str = String::new();
                    print!("Enter your move: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut move_str).unwrap();
                    let move_str_len = move_str.len();
                    if move_str_len == 0 {
                        loop {}
                    }
                    move_str.truncate(move_str_len - 1);
                    match board.move_from_str(&move_str) {
                        Ok(mv) => {
                            if moves.contains(&mv) {
                                board.do_move(&mv);
                                break;
                            } else {
                                println!("Not a valid move");
                            }
                        }
                        Err(_) => {
                            println!("Bad move format");
                        }
                    }
                }
            }
            Team::Enemy => {
                println!("Computing best move...");
                let move_stats = minimax.minimax(&board, turn, 6);

                let best_move = move_stats.mv.unwrap();
                println!();
                println!("Score: {:?}", move_stats.score.score);
                println!("Turns: {}", move_stats.score.turns);
                println!("Nodes: {}", move_stats.nodes_visited);
                println!("My move is: {}", best_move);
                print!("I expect: ");
                for mv in move_stats.mvs.iter().rev().skip(1) {
                    print!("{} ", mv);
                }
                println!();
                board.do_move(&best_move);
            }
        }

        turn = turn.other_team();
    }
}
