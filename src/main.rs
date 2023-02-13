use std::io;
use std::process::Command;

mod level1;
// mod level2;

enum OptionChoice {
    Play,
    Quit,
}

fn main() {
    println!("Hello! Would you like to play the \x1b[34mOPA\x1b[0m Game?");
    println!("1. \x1b[32mYes, Please!\x1b[0m");
    println!("2. \x1b[31mNo thanks, bye!\x1b[0m");

    let selected_option = match get_user_input() {
        Ok(choice) => match choice.trim() {
            "1" => OptionChoice::Play,
            "2" => OptionChoice::Quit,
            _ => {
                println!("\x1b[31mInvalid choice\x1b[0m");
                return;
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    match selected_option {
        OptionChoice::Play => {
            println!("Please select your level:");
            println!("1. Level 1");
            println!("2. Level 2");

            let level_choice = match get_user_input() {
                Ok(choice) => match choice.trim() {
                    "1" => level1::run(),
                    // "2" => level2::run(),
                    _ => {
                        println!("\x1b[31mInvalid choice\x1b[0m");
                        return;
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };
        },
        OptionChoice::Quit => println!("\x1b[31mOkay, bye!\x1b[0m"),
    }
}

fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
