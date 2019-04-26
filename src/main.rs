use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        // shell ps1 variable
        print!("> ");
        stdout().flush().expect("Stdout flushed");

        // read the input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline character
        // everything after first whitespace is treated
        // as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // execute the command
        match command {
            // handle cd command
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },

            // handle exit or quit command
            "exit" | "quit" => { 
                return
            },
            
            // handle any other command
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => { child.wait().expect("Process finished"); },
                    Err(e) => eprintln!("{}", e),
                }                  
            },
        }
    }
}