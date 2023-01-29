use connect4::{connections, Player, PlayersConnection};
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let player1 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Red,
        };
        let player2 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Blue,
        };

        tokio::spawn(connections::game_handler(player1, player2));
    }
}
