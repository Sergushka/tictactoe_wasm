use std::fmt::Display;

pub struct Game {
    state: [Player; 9],
    last: Player,
    finished: bool,
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Player {
    X,
    O,
    Empty,
}

impl Player {
    fn get_opposite(&self) -> Self {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
            _ => Player::Empty,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::O => f.write_str("O"),
            Player::X => f.write_str("X"),
            _ => f.write_str(" "),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: [Player::Empty; 9],
            last: Player::X,
            finished: false,
        }
    }

    pub fn get_state(&self) -> String {
        self.state.map(|p| p.to_string()).join("\n")
    }

    pub fn get_last_opposite(&self) -> String {
        self.last.get_opposite().to_string()
    }

    pub fn restart(&mut self) {
        self.state = [Player::Empty; 9];
        self.last = Player::X;
        self.finished = false;
    }

    fn flip_last(&mut self) {
        self.last = self.last.get_opposite();
    }

    pub fn make_turn(&mut self, i: usize) {
        if self.state[i] == Player::Empty && !self.finished {
            self.state[i] = self.last;
            self.flip_last();
            self.game_over();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn game_over(&mut self) {
        fn check_row(state: [Player; 9]) -> bool {
            let mut i = 0;
            while i < state.len() - 2 {
                let el = state[i];
                if el != Player::Empty && el == state[i + 1] && el == state[i + 2] {
                    return true;
                }
                i += 3;
            }
            false
        }
        fn check_col(state: [Player; 9]) -> bool {
            // 0 3 6
            // 1 4 7
            // 2 5 8
            for i in 0..state.len() / 3 {
                let el = state[i];
                if el != Player::Empty && el == state[i + 3] && el == state[i + 6] {
                    return true;
                }
            }
            false
        }
        fn check_dig(state: [Player; 9]) -> bool {
            // 0 4 8
            // 2 4 6
            if state[0] != Player::Empty && state[0] == state[4] && state[0] == state[8] {
                return true;
            }
            if state[2] != Player::Empty && state[2] == state[4] && state[2] == state[6] {
                return true;
            }
            false
        }
        self.finished = check_row(self.state) || check_col(self.state) || check_dig(self.state);
    }
}
