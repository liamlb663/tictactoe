
#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O
}
impl Player {
    pub fn other(&self) -> Self{
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub board: [Option<Player>; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
        }
    }

    pub fn print(&self, prefix: &str) {
        let pieces: Vec<char> = self.board.iter().map(|x| match x {
            None => '.',
            Some(Player::X) => 'x',
            Some(Player::O) => 'o',
        }).collect();

        for row in pieces.chunks(3) {
            print!("{}", prefix);
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
        //println!("{}", self.turn);
    }

    pub fn is_win(&self) -> Option<Player> {

        for i in 0..3 {
            //If Row is win
            if  self.board[i*3] != None &&
                self.board[i*3] == self.board[i*3+1] &&
                self.board[i*3] == self.board[i*3+2]
            {
                return self.board[i*3];
            }

            //If Col is win
            if  self.board[i] != None &&
                self.board[i] == self.board[i+3] &&
                self.board[i] == self.board[i+6]
            {
                return self.board[i];
            }
        }

        //Diagonal top-left to bottom-right
        if  self.board[0] != None &&
            self.board[0] == self.board[4] &&
            self.board[0] == self.board[8]
        {
            return self.board[0];
        }

        //Diagonal top-right to bottom-left
        if  self.board[2] != None &&
            self.board[2] == self.board[4] &&
            self.board[2] == self.board[6]
        {
            return self.board[2];
        }

        None
    }

    pub fn is_complete(&self) -> bool {
        if self.is_win().is_some() { return true; }
        self.board.iter().filter(|&&x| x == None).count() == 0
    }

    pub fn list_open(&self) -> Vec<usize> {
        self.board.iter().enumerate().filter_map(|(i, &val)| if val == None { Some(i) } else { None }).collect()
    }

    pub fn place(&mut self, cell: usize, player: Player) {
        if self.board[cell] != None {
            panic!();
        }
        self.board[cell] = Some(player);
    }
}
