
mod board;
mod bots;

use std::time::Instant;

use board::Board;

use bots::DFSBot;
use bots::{Bot, RandomBot, Human};
use clap::{Parser, ArgEnum};

struct Game {
    xbot: Box<dyn Bot>,
    obot: Box<dyn Bot>,
    board: Board,
}

#[allow(dead_code)]
impl Game {
    fn new(bot1: Box<dyn Bot>, bot2: Box<dyn Bot>) -> Self {
        Self {
            board: Board::new(),
            xbot: bot1,
            obot: bot2,
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

        println!("X wins: {}", x_wins);
        println!("O wins: {}", o_wins);
        println!("Draws: {}", draws);

        println!("X%: {}", (100 * x_wins) as f64 / games as f64);
        println!("O%: {}", (100 * o_wins) as f64 / games as f64);
        println!("Draw%: {}", (100 * draws) as f64 / games as f64);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets the number of games to simulate
    #[clap(short, long, value_parser, default_value_t = 1)]
    games: u32,

    /// Sets the bot for player 1
    #[clap(short = '1', long = "player1", arg_enum, default_value = "random")]
    p1: BotType,

    /// Sets the bot for player 2
    #[clap(short = '2', long = "player2", arg_enum, default_value = "dfs")]
    p2: BotType,

    #[clap(short = 't', long = "time")]
    timed: bool,
}

#[derive(Debug, Clone, PartialEq, ArgEnum)]
enum BotType {
    Random,
    DFS,
    Human,
}

impl BotType {
    fn create_bot(&self, player: u8) -> Box<dyn Bot> {
        match self {
            BotType::Random => Box::new(RandomBot::new(player)),
            BotType::DFS => Box::new(DFSBot::new(player)),
            BotType::Human => Box::new(Human::new(player)),
        }
    }
}

fn main () {
    let args = Args::parse();

    let mut game = Game::new(args.p1.create_bot(1), args.p2.create_bot(2));

    // Start timing
    let start = Instant::now();

    if args.games == 1 {
        game.sim_game(true);
    } else {
        game.sim_games(args.games);
    }

    // End timing
    let duration = start.elapsed();

    // Print out the duration of the game(s)
    if args.timed {
        println!("\nTime taken: {:?}", duration);
    }
}
