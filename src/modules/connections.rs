use crate::{Player, Table};
use futures;
use tokio::io::{self, AsyncWriteExt, ErrorKind};
use tokio::net::TcpStream;

pub struct PlayersConnection {
    pub socket: TcpStream,
    pub colour: Player,
}

impl PlayersConnection {
    pub fn new(socket: TcpStream, colour: Player) -> PlayersConnection {
        PlayersConnection { socket, colour }
    }

    pub fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.try_read(buf)
    }

    pub async fn readable(&self) -> io::Result<()> {
        self.socket.readable().await
    }

    pub async fn write(&mut self, buf: &[u8]) -> io::Result<()> {
        self.socket.write_all(buf).await
    }

    pub async fn write_to_both(
        player1: &mut Self,
        player2: &mut Self,
        bytes: &[u8],
    ) -> io::Result<()> {
        futures::future::join_all([player1.write(bytes), player2.write(bytes)])
            .await
            .into_iter()
            .collect::<io::Result<()>>()
    }

    pub async fn read_move(&mut self) -> io::Result<u8> {
        loop {
            self.readable().await?;
            let mut players_move_buf = [0_u8; 128];
            let players_move: u8;
            match self.try_read(&mut players_move_buf) {
                Ok(0) => {
                    continue;
                }
                Ok(_) => {
                    players_move = players_move_buf[0];
                    if players_move < 7 {
                        return Ok(players_move);
                    } else {
                        self.write(format!("Invalid move. Your turn {}", self.colour).as_bytes())
                            .await
                            .unwrap();
                        continue;
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

pub async fn game_handler(
    mut player1: PlayersConnection,
    mut player2: PlayersConnection,
) -> Result<(), String> {
    let mut players_move: u8;
    let mut table = Table::new();

    let winner = 'game: loop {
        PlayersConnection::write_to_both(&mut player1, &mut player2, table.to_string().as_bytes())
            .await
            .map_err(|e| format!("{}", e))?;

        for player in [&mut player1, &mut player2] {
            player
                .write(table.to_string().as_bytes())
                .await
                .map_err(|e| format!("{}", e))?;

            player
                .write(format!("Your turn {}", player.colour).as_bytes())
                .await
                .map_err(|e| format!("{}", e))?;

            players_move = player.read_move().await.map_err(|e| format!("{}", e))?;

            table.player_played(player.colour, players_move)?;
            player
                .write(table.to_string().as_bytes())
                .await
                .map_err(|e| format!("{}", e))?;

            if table.check_for_victory(player.colour) {
                break 'game player.colour.clone();
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

    Ok(())
}
