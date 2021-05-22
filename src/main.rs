use std::io;

fn main() {
    println!("What's your name?");

    let input_handler = io::stdin();
    let mut name = String::new();
    let mut player_move = String::new();
    input_handler
        .read_line(&mut name)
        .expect("Failed to read line");

    name = name.trim().to_string();

    println!("Hello, {}!", name);
    println!("Let's play Rust Paper Scissors! What do you choose? (r)ock, (p)aper, or (s)cissors?");

    input_handler.read_line(&mut player_move).expect("Failed to read line");

    println!("You picked {}!", player_move.trim());
}
