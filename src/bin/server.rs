use connect4::{Table, Player};
use tokio::{
    net::TcpListener
};
use std::error::Error;
use connect4::PlayersConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {

        let mut player1 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Red
        };
        let mut player2 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Blue
        };

        tokio::spawn(async move {
            let mut players_move: u8;
            let mut table = Table::new();

            loop {
                PlayersConnection::write_to_both(&mut player1, &mut player2, table.to_string().as_bytes()).await.unwrap();

                for player in [&mut player1, &mut player2] {

                    player.write(table.to_string().as_bytes()).await.unwrap();
                    player.write(
                        format!("Your turn {}", player.colour)
                        .as_bytes()
                    ).await.unwrap();

                    players_move = player.read_move().await.unwrap();

                    table.player_played(player.colour, players_move).unwrap();
                    player.write(table.to_string().as_bytes()).await.unwrap();
                }
            }
        });
    }
}
