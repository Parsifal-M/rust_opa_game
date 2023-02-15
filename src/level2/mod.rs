extern crate termion;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;

use termion::clear;
use termion::cursor;

pub fn run() -> Result<bool, String> {

    println!("Someone has come in to get a Coffee and has asked for an extra shot of esspresso! You ask him for his Premium Membership card and he does not have one...
    It states very clearly on the door, that is only open to Premium Members.");
    
    // Clear the terminal and move the cursor to the top-left corner
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    // Flush the output buffer to ensure that the terminal is cleared
    io::stdout().flush().unwrap();

    loop {

        println!("Level 2 Order (level2.json):");
        let file_contents = fs::read_to_string("src/level2/level2.json").expect("Failed to read level2.json");

        let output = Command::new("opa")
            .arg("eval")
            .arg("--format")
            .arg("pretty")
            .arg("--data")
            .arg("src/level2/level2.rego")
            .arg("--input")
            .arg("src/level2/level2.json")
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
            return Ok(true);
        } else {

            println!("What would you like to do?");
            println!("1. Retry");
            println!("2. Exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "1" => {},
                "2" => return Ok(false),
                _ => {}
            }
        }
    }
}
