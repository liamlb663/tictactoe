use core::panic;
use std::io;

use rand::seq::SliceRandom;

use crate::board::{Board, Player};

pub trait Bot {
    fn new(player: Player) -> Self;
    fn choose_next(&self, board: &Board) -> usize;
}

pub struct RandomBot;
impl Bot for RandomBot {
    fn new(_player: Player) -> Self {
        Self {}
    }

    fn choose_next(&self, board: &Board) -> usize {
        let open = board.list_open();
        *open.choose(&mut rand::thread_rng()).unwrap()
    }
}

pub struct Human {
    mark: char,
}
impl Bot for Human{ //Hehe
    fn new(player: Player) -> Self {
        Self {
            mark: match player {
                Player::X => 'x',
                Player::O => 'o',
            },
        }
    }

    fn choose_next(&self, board: &Board) -> usize {
        let open = board.list_open();

        loop {
            println!("Place a: {}", self.mark);
            board.print("  ");

            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Expected Input");
            println!();

            let trimmed = input.trim();
            let input = match trimmed.parse::<usize>() {
                Ok(i) => i,
                Err(..) => continue,
            } - 1;

            if !open.contains(&input) {
                continue;
            }

            return input;
        }
    }
}

pub struct DFSBot {
    player: Player
}
impl DFSBot {
    pub fn generate_moves(&self, board: &Board, player: Player) -> Vec<(usize, Board)> {
        let spaces = board.list_open();

        spaces.iter().map(|&x| {
            let mut board = board.clone();
            board.place(x, player);
            (x, board)
        }).collect()
    }

    pub fn minmax(&self, board: Board, active_player: Player) -> i8 {
        if board.is_complete() {
            let status = board.is_win();
            if let None = status {
                return 0;
            } else {
                let status: Player = status.unwrap();
                if status == self.player {
                    return 1;
                } else {
                    return -1;
                }
            }
        }

        let next_player = self.player.other();
        let moves = self.generate_moves(&board, active_player);

        let mut min_result = 1;
        let mut max_result = -1;

        for (_poss_move, poss_board) in moves {
            let poss_result = self.minmax(poss_board, next_player);

            if min_result > poss_result {
                min_result = poss_result;

                // AB pruning
                /*if self.player != active_player && min_result == -1 {
                    break;
                }*/
            }

            if max_result < poss_result {
                max_result = poss_result;

                // AB pruning
                /*if self.player != active_player && min_result == -1 {
                    break;
                }*/
            }
        }

        if self.player == active_player {
            max_result
        } else {
            min_result
        }
    }
}
impl Bot for DFSBot {
    fn new(player: Player) -> Self {
        Self { player }
    }

    fn choose_next(&self, board: &Board) -> usize {

        let next_player = self.player.other();

        let moves = self.generate_moves(&board, self.player);

        let mut processed_moves: Vec<(usize, i8)> = Vec::new();
        for (poss_move, poss_board) in moves {
            let poss_result = self.minmax(poss_board, next_player);

            processed_moves.push((poss_move, poss_result));
        }

        for processed_move in &processed_moves {
            if processed_move.1 == 1 {
                return processed_move.0;
            }
        }
        for processed_move in &processed_moves {
            if processed_move.1 == 0 {
                return processed_move.0;
            }
        }
        for processed_move in &processed_moves {
            if processed_move.1 == -1 {
                return processed_move.0;
            }
        }

        panic!();
    }
}
