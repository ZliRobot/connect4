mod modules;
pub use modules::connections::PlayersConnection;
pub use modules::player::Player;
pub use modules::table::Table;

#[cfg(test)]
mod tests {
    use super::*;

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
