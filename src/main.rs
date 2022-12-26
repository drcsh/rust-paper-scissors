use std::io;
use rand::Rng;

fn main() {
    let win = "win";
    let lose = "lose";
    let draw = "draw";
    let options = ["rock", "paper", "scissors", "r", "p", "s", "q", "quit"];
    
    let input_handler = io::stdin();
    let mut player_name = String::new();
    let mut player_move: String;
    let mut computer_move: String;
    let mut result_text: &str;
    let mut player_score = 0;
    let mut computer_score = 0;

    println!("What's your name?");
    input_handler
        .read_line(&mut player_name)
        .expect("Failed to read line");

    player_name = player_name.trim().to_string();

    println!("Hello, {}!", player_name);

    loop {
        println!();
        println!("Let's play Rust Paper Scissors! Pick (r)ock, (p)aper, or (s)cissors or (q)uit:");
        let mut player_input = String::new();
        input_handler.read_line(&mut player_input).expect("Failed to read line");

        player_move = normalise_move(&player_input);

        if !options.contains(&&*player_move) {
            println!("Invalid input '{}', please pick from:", player_move);
            for value in options.iter(){
                println!("  {}", value);
            }
            continue;
        }

        if player_move == "quit" {
            break;
        }

        // Instead of picking a move for the computer and comparing the moves, 
        // it's simpler to just decide the result and work out the move from the result
        let result = rand::thread_rng().gen_range(1..4); 
        if result == 1 {
                player_score += 1;
                result_text = win;
                computer_move = computer_action(&player_move, false);
        } else if result == 2 {
                result_text = draw;
                computer_move = player_move.clone();
        } else {
                computer_score += 1;
                result_text = lose;
                computer_move = computer_action(&player_move, true);
        }

        println!(
            "{player_name} picked {player_move}, computer picked {computer_move}, result: {result_text}!",
        );
    }

    println!();
    println!("Final Scores:");
    println!("{}: {}", player_name, player_score);
    println!("Computer: {}", computer_score);
    println!("Thanks for playing!");

}


fn normalise_input(input: &String) -> String {
    // converts input to lowercase and removes whitespace
    input.to_lowercase().trim().to_string()
}


/**
     The player could enter the short or long forms of the inputs, and might accidentally
     capitalise them. This function normalises this to standard values.
**/
fn normalise_move(input: &String) -> String {

    let input_normalised = normalise_input(&input);

    // Note that .get returns an Option, so must be compared with Some()
    match input_normalised.get(0..1) {
        Some("r")=>return String::from("rock"),
        Some("p")=>return String::from("paper"),
        Some("s")=>return String::from("scissors"),
        Some("q")=>return String::from("quit"),
        _=> return input_normalised,
    }
}

/**
    Work out what move the computer makes based on the player move and whether the computer has
    won or not.
**/
fn computer_action(player_move: &String, computer_wins: bool) -> String {

    if computer_wins {
        match player_move.as_str() {
            "rock" => return String::from("paper"),
            "paper" => return String::from("scissors"),
            "scissors" => return String::from("rock"),
            &_ => panic!("Invalid player move. This shouldn't happen!")
        }

    } else {
        match player_move.as_str()  {
            "rock" => return String::from("scissors"),
            "paper" => return String::from("rock"),
            "scissors" => return String::from("paper"),
            &_ => panic!("Invalid player move. This shouldn't happen!")
        }
    }
}
