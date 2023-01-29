use connect4::PlayersConnection;
use connect4::{Player, Table};
use std::error::Error;
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

        tokio::spawn(async move {
            let mut players_move: u8;
            let mut table = Table::new();

            let winner = 'game: loop {
                PlayersConnection::write_to_both(
                    &mut player1,
                    &mut player2,
                    table.to_string().as_bytes(),
                )
                .await
                .unwrap();

                for player in [&mut player1, &mut player2] {
                    player.write(table.to_string().as_bytes()).await.unwrap();

                    player
                        .write(format!("Your turn {}", player.colour).as_bytes())
                        .await
                        .unwrap();

                    players_move = player.read_move().await.unwrap();

                    table.player_played(player.colour, players_move).unwrap();
                    player.write(table.to_string().as_bytes()).await.unwrap();

                    if table.check_for_victory(player.colour) {
                        break 'game player.colour;
                    };
                }
            };

            PlayersConnection::write_to_both(
                &mut player1,
                &mut player2,
                format!("{} player won!", winner).to_string().as_bytes(),
            )
            .await
            .unwrap();
        });
    }
}
