use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
use colored::*;

fn main() {
    println!("{}","Welcome to the rock paper scissors !! ".yellow());

    let mut wins:u32 = 0;
    let mut losses:u32 = 0;
    
    loop {
        let mut player = String::new();
        println!("{}","Choose between Rock(r),Paper(p),Scissors(s)".green());

        io::stdin().read_line(&mut player).expect("Failed to read");

        println!("You played: {player}");

        let choices = [String::from("r"), String::from("p"), String::from("s")];
        let mut rng = thread_rng();
        let comp_choice = choices.choose(&mut rng).expect("Empty Range");

        println!("Computer played: {}", comp_choice);

        if &player.trim() == comp_choice {
            println!("{}","Draw".yellow());
        } else if player.trim() == "p" && comp_choice == "r" {
            println!("{}","You win".green());
            wins+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "r" && comp_choice == "s" {
            println!("{}","You win".green());
            wins+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "s" && comp_choice == "p" {
            println!("{}","You win".green());
            wins+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "r" && comp_choice == "p" {
            println!("{}","You lose".red());
            losses+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "p" && comp_choice == "s" {
            println!("{}","You lose".red());
            losses+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "s" && comp_choice == "r" {
            println!("{}","You lose".red());
            losses+=1;
            println!("Current Wins:{wins}, Losses:{losses}");
        } else if player.trim() == "quit"{
            break;
        } else {
            println!("You idiot");
        }
    }
}