use connect4::connections::ServerMessage;
use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use winconsole::console::flush_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = TcpStream::connect("localhost:8080").await.unwrap();
    let (mut reader, mut writer) = connection.split();
    let mut reader = BufReader::new(&mut reader);

    let mut buf = String::new();
    let mut players_move: u8;

    loop {
        buf.clear();
        let n = reader.read_line(&mut buf).await?;

        if n == 0 {
            println!("Connection closed");
            return Ok(());
        }

        let message = serde_json::from_str(&buf)?;

        match &message {
            ServerMessage::Table(table) => {
                println!("{}", table);
            }
            ServerMessage::YourTurn => {
                println!("Your turn");
                players_move = input_move();
                writer.write_u8(players_move).await?;
            }
            ServerMessage::Victory(player) => {
                println!("Player {} won!", player);
                break;
            }
            ServerMessage::InvalidMoveColumnFull => {
                println!("Column is full, play different column");
            }
            ServerMessage::InvalidMoveOutOfRange => {
                println!("Enter number in range 0-6");
                players_move = input_move();
                writer.write_u8(players_move).await?;
            }
            ServerMessage::Error(err) => {
                println!("{}", err);
            }
        }
    }

    _ = std::io::stdin().lines();
    Ok(())
}

fn input_move() -> u8 {
    let mut line = String::new();
    _ = flush_input();

    while line.trim().parse::<u8>().is_err() {
        line.clear();
        _ = std::io::stdin().read_line(&mut line);
    }
    line.trim().parse::<u8>().unwrap()
}
