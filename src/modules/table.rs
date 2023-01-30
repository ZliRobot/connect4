use crate::Player;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

pub const LEN: usize = 7;
pub const FOUR: usize = 4;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Table {
    pub data: [[Option<Player>; LEN]; LEN],
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: [[None; LEN]; LEN],
        }
    }

    pub fn player_played(&mut self, player: Player, column: u8) -> Result<(), String> {
        let column = column as usize;
        if column >= LEN {
            return Err(format!(
                "Invalid move. Player can play in columns 0-{}",
                LEN - 1
            ));
        }

        let mut row = 0;
        while row < LEN && self.data[column][row].is_some() {
            row += 1
        }
        if row >= LEN {
            return Err(format!("Invalid move. Column {} is full", column));
        };

        self.data[column][row] = Some(player);
        Ok(())
    }

    pub fn check_for_victory(&self, player: Player) -> bool {
        // Check rows.
        if (0..LEN)
            .map(|row| {
                (0..LEN - 3)
                    .map(move |column| {
                        (0..FOUR)
                            .map(|step| self.data[column + step][row])
                            .all(|field| field == Some(player))
                    })
                    .any(|x| x)
            })
            .any(|x| x)
        {
            return true;
        }

        // Check columns.
        if (0..LEN)
            .map(|column| {
                (0..LEN - 3)
                    .map(move |row| {
                        (0..4)
                            .map(|step| self.data[column][row + step])
                            .all(|field| field == Some(player))
                    })
                    .any(|x| x)
            })
            .any(|x| x)
        {
            return true;
        }

        // Check major diagonals.
        if (0..LEN)
            .map(|diagonal| {
                let position = LEN as i8 - FOUR as i8 - diagonal as i8;
                let (row, column, diagonal_length) = if position >= 0 {
                    (position as usize, LEN - 1, LEN - position as usize)
                } else {
                    (
                        0,
                        LEN - (-position) as usize - 1,
                        LEN - (-position) as usize,
                    )
                };
                (row, column, diagonal_length)
            })
            .map(|(row, column, diagonal_length)| {
                (0..diagonal_length - FOUR + 1)
                    .map(|starting_offset| {
                        (0..FOUR)
                            .map(|step| {
                                self.data[column - starting_offset - step]
                                    [row + starting_offset + step]
                            })
                            .all(|field| field == Some(player))
                    })
                    .any(|x| x)
            })
            .any(|x| x)
        {
            return true;
        }

        // Check minor diagonals.
        if (0..LEN)
            .map(|diagonal| {
                let position = LEN as i8 - FOUR as i8 - diagonal as i8;
                let (row, column, diagonal_length) = if position >= 0 {
                    (position as usize, 0, LEN - position as usize)
                } else {
                    (0, (-position) as usize, LEN - (-position) as usize)
                };
                (row, column, diagonal_length)
            })
            .map(|(row, column, diagonal_length)| {
                (0..diagonal_length - FOUR + 1)
                    .map(|starting_offset| {
                        (0..FOUR)
                            .map(|step| {
                                self.data[column + starting_offset + step]
                                    [row + starting_offset + step]
                            })
                            .all(|field| field == Some(player))
                    })
                    .any(|x| x)
            })
            .any(|x| x)
        {
            return true;
        };

        false
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in (0..LEN).rev() {
            writeln!(f)?;
            for column in 0..LEN {
                write!(
                    f,
                    "{}",
                    match self.data[column][row] {
                        None => "⚫",
                        Some(Player::Blue) => "🔵",
                        Some(Player::Red) => "🔴",
                    }
                )?;
            }
        }
        writeln!(f)?;
        writeln!(f, "0️⃣ 1️⃣ 2️⃣ 3️⃣ 4️⃣ 5️⃣ 6️⃣")
    }
}
