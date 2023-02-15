extern crate termion;

use std::io::{self, stdout};
use termion::{input::TermRead, event::Key, raw::IntoRawMode};

mod level1;
// mod level2;

fn main() {
    println!("Do you want to play the OPA Game?");
    println!("1. Yes");
    println!("2. No");

    let stdin = io::stdin();
    let reader = stdin.keys();

    for key in reader {
        match key.unwrap() {
            Key::Char('1') => {
                let stdout = stdout().into_raw_mode().unwrap();
                drop(stdout);

                let level_modules = vec![
                    level1::run as fn() -> Result<bool, String>
                    // level2::run as fn() -> Result<bool, String>,
                    /* ... add more levels here ... */
                ];

                let mut level = 0;
                loop {
                    if level == level_modules.len() {
                        println!("Congratulations! You passed all levels.");
                        break;
                    }

                    let module = level_modules[level];
                    match module() {
                        Ok(true) => {
                            println!("Congratulations! You passed level {}.", level + 1);
                            println!("What would you like to do?");
                            println!("1. Continue to the next level");
                            println!("2. Exit");

                            let stdin = io::stdin();
                            let reader = stdin.keys();

                            // Wait for user input to continue or exit
                            for key in reader {
                                match key.unwrap() {
                                    Key::Char('1') => {
                                        level += 1;
                                        break;
                                    }
                                    Key::Char('2') => {
                                        println!("Thanks for playing!");
                                        return;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Ok(false) => {
                            println!("Quitting at level {}.", level + 1);
                            println!("Thanks for playing!");
                            return;
                        }
                        Err(error) => {
                            println!("Error: {}", error);
                            return;
                        }
                    }
                }
            }
            Key::Char('2') => {
                println!("Thanks for playing!");
                return;
            }
            _ => {}
        }
    }
}
