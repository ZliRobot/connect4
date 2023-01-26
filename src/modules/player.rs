use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
