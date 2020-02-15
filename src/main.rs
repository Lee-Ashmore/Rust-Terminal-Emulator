extern crate nix;

use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!(">> ");
        stdout().flush().expect("Could not flush Stdout");

        // Parse Input --------------------------------------------------------

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Create commands from input -----------------------------------------

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            // Match commands -------------------------------------------------

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                // Execute Commands -------------------------------------------
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // There is another command, so pipe to the next
                        Stdio::piped()
                    } else {
                        // There is no other command, send the data out
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // Handle output ------------------------------------------

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(error) => {
                            previous_command = None;
                            eprintln!("{}", error);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().expect("Kalibunga Dude!");
        }
    }
}
