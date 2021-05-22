use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("What's your name?");

    let win = "win";
    let lose = "lose";
    let draw = "draw";
    let options = ["rock", "paper", "scissors", "r", "p", "s", "q", "quit"];
    let computer_moves = ["rock", "paper", "scissors"];
    let input_handler = io::stdin();
    let mut player_name = String::new();
    let mut computer_move = String::new();
    let mut result = String::new();

    input_handler
        .read_line(&mut player_name)
        .expect("Failed to read line");

    player_name = player_name.trim().to_string();

    println!("Hello, {}!", player_name);

    loop {
        println!("Let's play Rust Paper Scissors! Pick (r)ock, (p)aper, or (s)cissors or (q)uit");
        let mut player_move = String::new();
        input_handler.read_line(&mut player_move).expect("Failed to read line");
        player_move = normalise_move(player_move);

        if !options.contains(&&*player_move) {
            println!("Invalid input '{}', please pick from:", player_move);
            for value in options.iter(){
                println!("  {}", value);
            }
        }

        if player_move == "quit" {
            break;
        }

        computer_move = pick_random(&computer_moves);

        result = String::from(win);
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

        println!(
            "{} picked {}, computer picked {}, result: {}!",
            player_name,
            player_move,
            computer_move,
            result
        );
    }

    println!("Thanks for playing!")

}


fn normalise_input(input: String) -> String {
    // converts input to lowercase and removes whitespace
    input.to_lowercase().trim().to_string()
}

fn normalise_move(input: String) -> String {

    let input_normalised = normalise_input(input);

    if input_normalised == "rock" || input_normalised == "r" {
        return String::from("rock")
    } else if input_normalised == "paper" || input_normalised == "p" {
        return String::from("paper")
    }  else if input_normalised == "scissors" || input_normalised == "s" {
        return String::from("scissors")
    } else if input_normalised == "quit" || input_normalised == "q" {
        return String::from("quit")
    }
    return input_normalised

}

fn pick_random(options: &[&str]) -> String {
    // Returns a random member from an array of strs
    let mut rng = thread_rng();
    let index = rng.gen_range(0..options.len());
    return String::from(options[index]);
}

