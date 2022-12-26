use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let win = "win";
    let lose = "lose";
    let draw = "draw";
    let options = ["rock", "paper", "scissors", "r", "p", "s", "q", "quit"];
    let computer_moves = ["rock", "paper", "scissors"];
    let input_handler = io::stdin();
    let mut player_name = String::new();
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

        let player_move = normalise_move(player_input);

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

        let computer_move = pick_random(&computer_moves);

        let mut result = String::from(win);

        if player_move == "rock" {
            if computer_move == "rock" {
                result = String::from(draw);
            } else if computer_move == "paper" {
                result = String::from(lose);
            }

        } else if player_move == "paper"{
            if computer_move == "paper" {
                result = String::from(draw);
            } else if computer_move == "scissors" {
                result = String::from(lose);
            }
        } else if player_move == "scissors" {
            if computer_move == "scissors" {
                result = String::from(draw);
            } else if computer_move == "rock" {
                result = String::from(lose);
            }
        }

        if result == win {
            player_score += 1;
        } else if result == lose {
            computer_score += 1;
        }

        println!(
            "{} picked {}, computer picked {}, result: {}!",
            player_name,
            player_move,
            computer_move,
            result
        );
    }

    println!();
    println!("Final Scores:");
    println!("{}: {}", player_name, player_score);
    println!("Computer: {}", computer_score);
    println!("Thanks for playing!");

}


fn normalise_input(input: String) -> String {
    // converts input to lowercase and removes whitespace
    input.to_lowercase().trim().to_string()
}

fn normalise_move(input: String) -> String {
    /*
     The player could enter the short or long forms of the inputs, and might accidentally
     capitalise them. This function normalises this to standard values.
     */

    let input_normalised = normalise_input(input);

    match input_normalised.get(0..1) {
        Some("r")=>return String::from("rock"),
        Some("p")=>return String::from("paper"),
        Some("s")=>return String::from("scissors"),
        Some("q")=>return String::from("quit"),
        _=> return input_normalised,
    }
}

fn pick_random(options: &[&str]) -> String {
    // Returns a random member from an array of strs
    let mut rng = thread_rng();
    let index = rng.gen_range(0..options.len());
    return String::from(options[index]);
}
