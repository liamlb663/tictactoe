
mod board;
mod bots;

use std::time::Instant;

use board::{Board, Player};

use bots::DFSBot;

#[allow(unused)]    //Unused to allow for bot selection
use bots::{Bot, RandomBot, Human};

struct Game<Xbot, Obot>
where
    Xbot: Bot,
    Obot: Bot,
{
    xbot: Xbot,
    obot: Obot,
    board: Board,
}

#[allow(dead_code)]
impl<Xbot: Bot, Ybot: Bot> Game<Xbot, Ybot> {
    fn new() -> Self {
        Self {
            board: Board::new(),
            xbot: Xbot::new(Player::X),
            obot: Ybot::new(Player::O),
        }
    }

    fn sim_game(&mut self, print: bool) -> Option<Player> {
        self.board = Board::new();

        loop {
            self.board.place(self.xbot.choose_next(&self.board), Player::X);
            if self.board.is_complete() { break; }

            if print {
                self.board.print("");
                println!();
            }

            self.board.place(self.obot.choose_next(&self.board), Player::O);
            if self.board.is_complete() { break; }

            if print {
                self.board.print("");
                println!();
            }
        }

        if print {
            self.board.print("");
            println!();
        }

        self.board.is_win()
    }

    fn sim_games(&mut self, games: u32) {
        let mut x_wins = 0u32;
        let mut o_wins = 0u32;
        let mut draws = 0u32;

        let step = (games as f64 * 0.1) as u32;

        for i in 0..games {
            match self.sim_game(false) {
                Some(Player::X) => x_wins += 1,
                Some(Player::O) => o_wins += 1,
                None => draws += 1,
            }

            if i % step == 0 && i != 0 {
                //println!("Reached {}%", i * 100 / games);
            }
        }

        println!("X wins: {}", x_wins);
        println!("O wins: {}", o_wins);
        println!("Draws: {}", draws);

        println!("X%: {}", (100 * x_wins) as f64 / games as f64);
        println!("O%: {}", (100 * o_wins) as f64 / games as f64);
        println!("Draw%: {}", (100 * draws) as f64 / games as f64);
    }
}

fn main () {
    let time_now = Instant::now();

    let mut game = Game::<DFSBot, RandomBot>::new();
    game.sim_games(1000);

    println!("{:?}", time_now.elapsed());
}
