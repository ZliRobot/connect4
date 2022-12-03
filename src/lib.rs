use std::fmt::Display;

pub const LEN: usize = 7;
pub const FOUR: usize = 4;

#[derive(Debug, Default)]
pub struct Table {
    pub data: [[Option<Player>; LEN]; LEN],
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: [[None; LEN]; LEN],
        }
    }

    pub fn player_played(&mut self, player: Player, column: usize) -> Result<(), String> {
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
            write!(f, "\n")?;
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
        write!(f, "\n")?;
        writeln!(f, "0️⃣ 1️⃣ 2️⃣ 3️⃣ 4️⃣ 5️⃣ 6️⃣")
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_empty_table() {
        let table = Table::default();
        println!("{}", table);
    }

    #[test]
    fn test_player_played() {
        let mut table = Table::new();
        let player = Player::Red;
        table.player_played(player, 0).unwrap();
        assert_eq!(table.data[0][0], Some(player));
        println!("{}", table);
    }

    #[test]
    fn test_multiple_players_played() {
        let mut table = Table::new();
        let red = Player::Red;
        let blue = Player::Blue;
        table.player_played(red, 3).unwrap();
        table.player_played(blue, 2).unwrap();
        println!("{}", table);
    }
}
