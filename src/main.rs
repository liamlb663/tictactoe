
mod board;
mod bots;

use board::Board;

use bots::DFSBot;
#[allow(unused)]
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
            xbot: Xbot::new(1),
            obot: Ybot::new(2),
        }
    }

    fn sim_game(&mut self, print: bool) -> Option<u8> {
        self.board = Board::new();

        loop {
            let x_choice = self.xbot.choose_next(&self.board);
            //println!("X Choice: {}", x_choice);
            self.board.place(x_choice, 1);
            if self.board.is_complete() { break; }

            if print {
                self.board.print("");
                println!();
            }

            self.board.place(self.obot.choose_next(&self.board), 2);
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
                Some(1) => x_wins += 1,
                Some(2) => o_wins += 1,
                Some(_) => panic!(),
                None => draws += 1,
            }

            if i % step == 0 && i != 0 {
                //println!("Reached {}%", i * 100 / games);
            }
        }

        /*println!("X wins: {}", x_wins);
        println!("O wins: {}", o_wins);
        println!("Draws: {}", draws);*/

        println!("X%: {}", (100 * x_wins) as f64 / games as f64);
        println!("O%: {}", (100 * o_wins) as f64 / games as f64);
        println!("Draw%: {}", (100 * draws) as f64 / games as f64);
    }
}

fn main () {
    let mut game = Game::<DFSBot, RandomBot>::new();
    game.sim_games(100);

    /*let dfsbot = DFSBot::new(2);
    let board = Board{ board: [
                                    2,1,2,
                                    1,1,0,
                                    0,0,0
                                ]};

    board.print("Input Board: ");
    println!("Choice: {}", dfsbot.choose_next(&board) + 1);
    println!("Minimax: {}", dfsbot.minmax(board, 2));*/
}
