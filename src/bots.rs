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
    fn generate_moves(&self, board: &Board) -> Vec<(usize, Board)> {
        let spaces = board.list_open();

        spaces.iter().map(|&x| {
            let mut board = board.clone();
            board.place(x, self.player);
            (x, board)
        }).collect()
    }
}
impl Bot for DFSBot {
    fn new(player: u8) -> Self {
        Self { player }
    }

    fn choose_next(&self, board: &Board) -> usize {
        let moves = self.generate_moves(&board);

        for (space, board) in moves {
            if board.is_complete() {
                return space;
            }
        }

        let open = board.list_open();
        *open.choose(&mut rand::thread_rng()).unwrap()
    }
}
