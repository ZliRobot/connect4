use tokio::{net::TcpStream,io::ErrorKind, io::AsyncWriteExt};
use std::error::Error;
use winconsole::console::flush_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut connection = TcpStream::connect("localhost:8080").await.unwrap();
    let mut from_server: String;
    let mut players_move: u8;

    loop {
        loop {
            connection.readable().await?;
            let mut from_server_buf = vec![0_u8; 1024];
            match connection.try_read(&mut from_server_buf) {
                Ok(n) => {
                    from_server_buf.truncate(n);
                    from_server = String::from_utf8(from_server_buf.clone()).unwrap();
                    println!("{}", from_server);

                    if from_server.trim().contains("Your turn") {
                        players_move = input_move();
                        connection.write_u8(players_move).await.unwrap();
                    };
                    break;
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
}

fn input_move() -> u8 {
    let mut line = String::new();
    _ = flush_input();

    while line.trim().parse::<usize>().is_err() {
        line.clear();
        _ = std::io::stdin().read_line(&mut line);
    }
    line.trim().parse::<u8>().unwrap()
}
