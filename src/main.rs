use std::io;

mod level1;
// mod level2;

fn main() {
    println!("Hello! Would you like to play the OPA Barista Game? (y/n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim().to_lowercase() == "y" {
        let level_modules = vec![
            level1::run as fn() -> Result<bool, String>,
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

                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    match input.trim() {
                        "1" => {
                            level += 1;
                        }
                        "2" => {
                            println!("Thanks for playing!");
                            break;
                        }
                        _ => {
                            println!("Invalid input. Exiting...");
                            break;
                        }
                    }
                }
                Ok(false) => {
                    println!("Quitting at level {}.", level + 1);
                    println!("Thanks for playing!");
                    break;
                }
                Err(error) => {
                    println!("Error: {}", error);
                    break;
                }
            }
        }
    } else {
        println!("Thanks for considering the OPA Barista Game. Goodbye!");
    }
}
