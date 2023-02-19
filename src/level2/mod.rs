extern crate termion;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;

use std::time::{Instant};

use termion::clear;
use termion::cursor;

pub fn run() -> Result<bool, String> {
    let start_time = Instant::now(); // Start the timer
    println!(
        "Someone has come in to get a Coffee and has asked for an extra shot of esspresso! You ask him for his Premium Membership card and he does not have one...
    It states very clearly on the door, that is only open to Premium Members."
    );

    // Clear the terminal and move the cursor to the top-left corner
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    // Flush the output buffer to ensure that the terminal is cleared
    io::stdout().flush().unwrap();

    loop {
        println!("Level 2 Order (level2.json):");
        let file_contents = fs
            ::read_to_string("src/level2/level2.json")
            .expect("Failed to read level2.json");

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
        // convert stdout from bytes to string
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Calculate the maximum width for the boxes
        let max_width =
            file_contents
                .lines()
                .chain(stdout.lines())
                .map(|line| line.len())
                .max()
                .unwrap_or(0) + 4; // Add 4 for the border

        // Print a border around the JSON output
        println!("{}", termion::color::Fg(termion::color::Green));
        println!("┌{}┐", "─".repeat(max_width - 2));
        for line in file_contents.lines() {
            println!("│ {: <width$} │", line, width = max_width - 4);
        }
        println!("└{}┘", "─".repeat(max_width - 2));
        println!("{}", termion::color::Fg(termion::color::Reset));

        // Print a border around the rego output
        println!("{}", termion::color::Fg(termion::color::Red));
        println!("┌{}┐", "─".repeat(max_width - 2));
        for line in stdout.lines() {
            println!("│ {: <width$} │", line, width = max_width - 4);
        }
        println!("└{}┘", "─".repeat(max_width - 2));
        println!("{}", termion::color::Fg(termion::color::Reset));

        if stdout.contains("Unfortunately, you are not a premium member, and cannot have an extra shot of espresso") {
            let elapsed_time = start_time.elapsed(); // Get elapsed time since timer started
            println!("Elapsed time: {:?}", elapsed_time); // Print the elapsed time
            return Ok(true);
        } else {
            println!("What would you like to do?");
            println!("1. Retry");
            println!("2. Exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "1" => {}
                "2" => {
                    return Ok(false);
                }
                _ => {}
            }
        }
    }
}