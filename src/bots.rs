use std::io;

use rand::seq::SliceRandom;

use crate::board::Board;

pub trait Bot {
    fn new(player: u8) -> Self;
    fn choose_next(&self, board: &Board) -> usize;
}

pub struct RandomBot;
impl Bot for RandomBot {
    fn new(_player: u8) -> Self {
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
    fn new(player: u8) -> Self {
        Self {
            mark: match player {
                1 => 'x',
                2 => 'o',
                _ => panic!(),
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
    player: u8
}
impl DFSBot {
    fn generate_moves(&self, board: &Board, player: u8) -> Vec<(usize, Board)> {
        let spaces = board.list_open();

        spaces.iter().map(|&x| {
            let mut board = board.clone();
            board.place(x, player);
            (x, board)
        }).collect()
    }

    fn minmax(&self, board: Board, active_player: u8) -> i8 {
        if board.is_complete() {
            match board.is_win() {
                Some(x) => if x == self.player {
                    return 1;
                } else {
                    return -1;
                },
                None => return 0,
            }
        }

        board.print("");

        let moves = self.generate_moves(&board, active_player);

        let mut worst = 1;
        for m in moves {
            let move_worst = self.minmax(m.1, otherplayer(active_player));
            println!("Cell {}: Score {}", m.0+1, worst);
            if move_worst < worst {
                worst = move_worst;
            }
        }

        println!();

        worst
    }
}
impl Bot for DFSBot {
    fn new(player: u8) -> Self {
        Self { player }
    }

    fn choose_next(&self, board: &Board) -> usize {

        let moves = self.generate_moves(&board, self.player);
        let mut processed_moves: Vec<(i8, usize)> = Vec::new();

        for (cell, new_board) in moves {
            let worst = self.minmax(new_board, otherplayer(self.player));

            processed_moves.push((worst, cell));
        }

        processed_moves.sort_by(|a, b| i8::cmp(&a.0, &b.0));
        println!("{:?}", processed_moves);
        processed_moves.pop().unwrap().1
    }
}

fn otherplayer(player: u8) -> u8 {
    match player {
        1 => 2,
        2 => 1,
        _ => panic!(),
    }
}
