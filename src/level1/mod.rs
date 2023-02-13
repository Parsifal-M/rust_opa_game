extern crate termion;

use self::termion::clear;
use std::fs;
use std::io;
use std::process::Command;

pub fn run() {
    print!("{}{}", clear::All, clear::CurrentLine);
    println!("It's the year 3000, you are the latest and greatest robot barista. In the year 3000 all orders are fed to you through a JSON file, its then up to you if the order is valid or not. Luckily you've been modded to be able to use OPA (Open Policy Agent) which will help you set some rules on the fly (nice!).\n");
    println!("Press Enter if you are ready!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    loop {
        print!("{}{}", clear::All, clear::CurrentLine);
        println!("As the proud employee of a specialty coffee shop, you can't believe your robot ears when a customer walks in and asks for a cola, completely disregarding the carefully crafted menu of premium coffee offerings.");
        println!("\n");
        println!("\nLevel 1 Order (level1.json):\n");

        let file_contents =
            fs::read_to_string("src/level1/level1.json").expect("Failed to read level1.json");
        println!("{}", file_contents);

        let output = Command::new("opa")
            .arg("eval")
            .arg("--format")
            .arg("pretty")
            .arg("--data")
            .arg("src/level1/level1.rego")
            .arg("--input")
            .arg("src/level1/level1.json")
            .arg("data.barista")
            .output()
            .expect("Failed to execute opa command");

        // convert stdout from bytes to string
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Print a border around the JSON output
        println!("{}", termion::color::Fg(termion::color::Green));
        println!("┌{}┐", "─".repeat(56));
        for line in file_contents.lines() {
            println!("│ {: <54} │", line);
        }
        println!("└{}┘", "─".repeat(56));
        println!("{}", termion::color::Fg(termion::color::Reset));

        // Print a border around the rego output
        println!("{}", termion::color::Fg(termion::color::Red));
        println!("┌{}┐", "─".repeat(56));
        for line in stdout.lines() {
            println!("│ {: <54} │", line);
        }
        println!("└{}┘", "─".repeat(56));
        println!("{}", termion::color::Fg(termion::color::Reset));

        if stdout.contains("Unfortunately, we do not serve Cola") {
            println!("Well done! You did it!");
        } else {
            println!("\nWhat would you like to do?");
            println!("1. Retry");
            println!("2. Exit");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            match choice.trim() {
                "1" => continue, // Retry the loop
                "2" => return,   // Exit the loop and quit the program
                _ => {
                    println!("Invalid choice");
                    return;
                }
            }
        }
    }
}
