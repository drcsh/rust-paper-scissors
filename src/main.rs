use std::io;

fn main() {
    println!("What's your name?");

    let options = ["rock", "paper", "scissors", "r", "p", "s", "q", "quit"];
    let mut computer_moves = ["rock", "paper", "scissors"];
    let input_handler = io::stdin();
    let mut name = String::new();
    let mut player_move = String::new();

    input_handler
        .read_line(&mut name)
        .expect("Failed to read line");

    name = name.trim().to_string();

    println!("Hello, {}!", name);
    println!("Let's play Rust Paper Scissors! Pick (r)ock, (p)aper, or (s)cissors or (q)uit");

    loop {
        player_move = String::new();
        input_handler.read_line(&mut player_move).expect("Failed to read line");
        player_move = normalise_input(player_move);

        if !options.contains(&&*player_move) {
            println!("Invalid input '{}', please pick from:", player_move);
            for value in options.iter(){
                println!("  {}", value);
            }
        }

        if player_move == "q" || player_move == "quit" {
            break;
        }

        println!("You picked {}!", player_move.trim());
    }

    println!("Thanks for playing!")

}


fn normalise_input(input: String) -> String{
    let mut output = input.to_lowercase().to_string();
    output = output.trim().to_string();

    output
}
