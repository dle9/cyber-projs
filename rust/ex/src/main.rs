mod games; mod player;

fn main() {
    let mut player = player::Player::new("br".to_string());

    // create Games instance for the player
    let games = games::Games::new(&player);
}
