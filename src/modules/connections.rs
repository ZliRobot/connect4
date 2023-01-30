use crate::{Player, Table};
use futures;
use serde_derive::{Deserialize, Serialize};
use serde_json;
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

    pub async fn send_message(&mut self, msg: &ServerMessage) -> io::Result<()> {
        self.socket
            .write_all(&msg.serialized_bytes().map_err(|_| ErrorKind::InvalidData)?)
            .await
    }

    pub async fn send_message_to_both(
        player1: &mut Self,
        player2: &mut Self,
        msg: &ServerMessage,
    ) -> io::Result<()> {
        futures::future::join_all([player1.send_message(msg), player2.send_message(msg)])
            .await
            .into_iter()
            .collect::<io::Result<()>>()
    }

    pub async fn read_move_from_player(&mut self) -> io::Result<u8> {
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
                        self.send_message(&ServerMessage::InvalidMoveOutOfRange)
                            .await?;
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
        PlayersConnection::send_message_to_both(
            &mut player1,
            &mut player2,
            &ServerMessage::Table(table.clone()),
        )
        .await
        .map_err(|e| format!("{}", e))?;

        for player in [&mut player1, &mut player2] {
            player
                .send_message(&ServerMessage::Table(table.clone()))
                .await
                .map_err(|e| format!("{}", e))?;

            player
                .send_message(&ServerMessage::YourTurn)
                .await
                .map_err(|e| format!("{}", e))?;

            players_move = player
                .read_move_from_player()
                .await
                .map_err(|e| format!("{}", e))?;

            table.player_played(player.colour, players_move)?;
            player
                .send_message(&ServerMessage::Table(table.clone()))
                .await
                .map_err(|e| format!("{}", e))?;

            if table.check_for_victory(player.colour) {
                break 'game player.colour;
            };
        }
    };

    PlayersConnection::send_message_to_both(
        &mut player1,
        &mut player2,
        &ServerMessage::Victory(winner),
    )
    .await
    .unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Table(Table),
    YourTurn,
    Victory(Player),
    InvalidMoveColumnFull,
    InvalidMoveOutOfRange,
    Error(String),
}

impl ServerMessage {
    pub fn serialized_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes = serde_json::to_string(&self)
            .map_err(|e| format!("{}", e))?
            .as_bytes()
            .to_vec();
        bytes.push(b'\n');

        Ok(bytes)
    }
}
