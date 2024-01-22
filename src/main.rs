
mod board;
mod bots;

use board::Board;
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

impl<Xbot: Bot, Ybot: Bot> Game<Xbot, Ybot> {
    fn new() -> Self {
        Self {
            board: Board::new(),
            xbot: Xbot::new(1),
            obot: Ybot::new(2),
        }
    }

    fn sim_game(&mut self) {
        while !self.board.is_complete() {
            self.board.place(self.xbot.choose_next(&self.board), 1);
            if self.board.is_complete() { break; }
            self.board.place(self.obot.choose_next(&self.board), 2);
        }

        self.board.print("");
    }
}

fn main () {
    let mut game = Game::<Human, Human>::new();

    game.sim_game();
}
