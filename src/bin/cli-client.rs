use connect4::{Player, Table};

fn main() {
    let mut table = Table::new();
    let mut users_move: usize;
    let mut player = Player::Red;
    println!("{}", table);

    loop {
        player = player.next();
        println!("{} player playing: ", player);

        users_move = input_move();

        while let Err(e) = table.player_played(player, users_move) {
            println!("Error: {:?}", e);
            users_move = input_move();
        }

        println!("{}", table);

        if table.check_for_victory(player) {
            println!("{} player  wins", player);
            return;
        }
    }
}

fn input_move() -> usize {
    let mut line = String::new();
    while line.trim().parse::<usize>().is_err() {
        line.clear();
        _ = std::io::stdin().read_line(&mut line);
    }
    line.trim().parse::<usize>().unwrap()
}
