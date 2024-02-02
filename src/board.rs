
#[derive(Clone)]
pub struct Board {
    pub board: [u8; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [0; 9],
        }
    }

    pub fn print(&self, prefix: &str) {
        let pieces: Vec<char> = self.board.iter().map(|x| match x {
            0 => '.',
            1 => 'x',
            2 => 'o',
            _ => panic!(),
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

    pub fn is_win(&self) -> Option<u8> {

        for i in 0..3 {
            //If Row is win
            if  self.board[i*3] != 0 &&
                self.board[i*3] == self.board[i*3+1] &&
                self.board[i*3] == self.board[i*3+2]
            {
                return Some(self.board[i*3]);
            }

            //If Col is win
            if  self.board[i] != 0 &&
                self.board[i] == self.board[i+3] &&
                self.board[i] == self.board[i+6]
            {
                return Some(self.board[i]);
            }
        }

        //Diagonal top-left to bottom-right
        if  self.board[0] != 0 &&
            self.board[0] == self.board[4] &&
            self.board[0] == self.board[8]
        {
            return Some(self.board[0]);
        }

        //Diagonal top-right to bottom-left
        if  self.board[2] != 0 &&
            self.board[2] == self.board[4] &&
            self.board[2] == self.board[6]
        {
            return Some(self.board[2]);
        }

        None
    }

    pub fn is_complete(&self) -> bool {
        if self.is_win().is_some() { return true; }
        self.board.iter().filter(|&&x| x == 0).count() == 0
    }

    pub fn list_open(&self) -> Vec<usize> {
        self.board.iter().enumerate().filter_map(|(i, &val)| if val == 0 { Some(i) } else { None }).collect()
    }

    pub fn place(&mut self, cell: usize, player: u8) {
        if self.board[cell] != 0 {
            panic!();
        }
        self.board[cell] = player;
    }
}
