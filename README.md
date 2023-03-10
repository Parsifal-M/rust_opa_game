## OPA Game CLI
The OPA Game CLI is a command-line interface (CLI) tool written in Rust that allows users to play a game based on the Open Policy Agent. The game is designed to make learning policy making fun with a CLI-style game that includes increasingly difficult levels as the user progresses through the game.

# Installation
To use the OPA Game CLI tool, you must have Rust installed on your system. If Rust is not yet installed, download and install it from the official Rust website: https://www.rust-lang.org/tools/install

# Usage

To run the OPA Game CLI tool, use the following command once you are inside the directory:

```sh
cargo run
```

This will compile and run the tool. You should see a welcome message with two options:

```markdown
Do you want to play the OPA Game?
1. Yes
2. No
```

You can choose one of the options by typing the corresponding number and pressing Enter. If you choose option 1, the tool will display a message indicating that the game is starting. If you choose option 2, the tool will display a message indicating that you have exited the game.

# How Does It Work?

Very simple really. The OPA Game CLI tool is a Rust program that uses the command line to fire off OPA commands like:

```rust
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
```


 The game is designed to be a fun way to learn about policy making. The game is a CLI-style game that includes increasingly difficult levels as the user progresses through the game.

# License
The OPA Game CLI tool is licensed under the Apache 2.0 license. For more details, see the LICENSE file.

# Contributing
Contributions to this project are welcome from anyone. To contribute, feel free to fork the repository and submit a pull request. You can also open an issue if you find a bug or have a feature request.
