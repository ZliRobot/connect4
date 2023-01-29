use connect4::{connections, Player, PlayersConnection};
use rand;
use std::{error::Error, mem};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let mut player1 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Red,
        };
        let mut player2 = PlayersConnection {
            socket: listener.accept().await.unwrap().0,
            colour: Player::Blue,
        };

        // Randomize starting player
        if rand::random() {
            mem::swap(&mut player1, &mut player2);
        }

        tokio::spawn(connections::game_handler(player1, player2));
    }
}
