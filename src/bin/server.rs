use connect4::{Table, Player};
use tokio::io::{self, AsyncWriteExt, ErrorKind};
use tokio::net::TcpStream;
use tokio::{
    net::TcpListener
};
use futures;
use std::error::Error;

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
                write_to_both(&mut player1, &mut player2, table.to_string().as_bytes()).await.unwrap();

                for mut player in [&mut player1, &mut player2] {

                    player.socket.write_all(table.to_string().as_bytes()).await.unwrap();
                    player.socket.write_all(
                        format!("Your turn {}", player.colour)
                        .as_bytes()
                    ).await.unwrap();

                    players_move = read_players_move(&mut player).await.unwrap();

                    table.player_played(player.colour, players_move).unwrap();
                    player.socket.write_all(table.to_string().as_bytes()).await.unwrap();
                }
            }
        });
    }
}

async fn write_to_both(player1: &mut PlayersConnection, player2: &mut PlayersConnection, bytes: &[u8]) -> io::Result<()> {
    futures::future::join_all([
        player1.socket.write_all(bytes),
        player2.socket.write_all(bytes)
    ]).await.into_iter().collect::<io::Result<()>>()
}

async fn read_players_move(player: &mut PlayersConnection) -> io::Result<u8> {
    loop {
        player.socket.readable().await?;
        let mut players_move_buf = [0_u8; 128];
        let players_move: u8;
        match player.socket.try_read(&mut players_move_buf) {
            Ok(0) => {
                continue;
            }
            Ok(_) => {
                players_move = players_move_buf[0];
                if players_move < 7 {
                    return Ok(players_move);
                } else {
                    player.socket.write_all(
                        format!("Invalid move. Your turn {}", player.colour)
                        .as_bytes()
                    ).await.unwrap();
                    continue;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
struct PlayersConnection {
    socket: TcpStream,
    colour: Player
}