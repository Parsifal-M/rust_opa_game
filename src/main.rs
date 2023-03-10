extern crate termion;
use std::io::{self, stdout, Write};
use termion::{clear, cursor, event::Key, input::TermRead};
use std::time::{Instant};

mod level1;
mod level2;

fn main() {
    println!("Do you want to play the OPA Game?");
    println!("1. Yes");
    println!("2. No");

    let stdin = io::stdin();
    let reader = stdin.keys();

    for key in reader {
        match key.unwrap() {
            Key::Char('1') => {
                let stdout = stdout();
                drop(stdout);

                let level_modules = vec![
                    level1::run as fn() -> Result<bool, String>,
                    level2::run as fn() -> Result<bool, String>,
                    /* ... add more levels here ... */
                ];

                let mut level = 0;
                let mut elapsed_times = Vec::new();
                loop {
                    if level == level_modules.len() {
                        println!("Congratulations! You passed all levels.");
                        println!("Level Times:");
                        for (index, time) in elapsed_times.iter().enumerate() {
                            println!("Level {}: {:?}", index+1, time);
                        }
                        return;
                    }

                    let module = level_modules[level];
                    // Clear the terminal and move the cursor to the top-left corner
                    print!("{}{}", clear::All, cursor::Goto(1, 1));
                    // Flush the output buffer to ensure that the terminal is cleared
                    io::stdout().flush().unwrap();

                    let start_time = Instant::now();
                    match module() {
                        Ok(true) => {
                            let elapsed_time = start_time.elapsed();
                            elapsed_times.push(elapsed_time);
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
                            let elapsed_time = start_time.elapsed();
                            elapsed_times.push(elapsed_time);
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
                println!("Thanks for playing, see you next time!");
                return;
            }
            _ => {}
        }
    }
}
