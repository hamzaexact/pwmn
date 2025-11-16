#![allow(warnings)]

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

mod parser;
use parser::lexer::{self, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    loop {
        // Read command (handles multi-line)
        let command = match read_command(&mut rl) {
            Ok(cmd) => {
                let k = lexer::Lexer::tokenize(cmd.as_str());
                match k {
                    Ok(tokens) => {
                        println!("{:?}", tokens)
                    }
                    Err(E) => println!("ERROR: {}", E),
                }
                cmd
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // Skip empty
        if command.trim().is_empty() {
            continue;
        }

        // Handle exit
        if command.trim().eq_ignore_ascii_case("quit")
            || command.trim().eq_ignore_ascii_case("exit")
        {
            break;
        }
    }
    Ok(())
}

fn read_command(rl: &mut DefaultEditor) -> Result<String, ReadlineError> {
    let mut buffer = String::new();
    let mut line_number = 0;

    loop {
        line_number += 1;

        // Change prompt for continuation lines
        let prompt = if line_number == 1 {
            "sqpwman> "
        } else {
            "      -> "
        };

        // Read one line
        let line = rl.readline(prompt)?;

        // Add to buffer with space separator
        if !buffer.is_empty() {
            buffer.push(' ');
        }
        buffer.push_str(line.trim());

        // Check if command is complete

        // 1. Ends with semicolon? Done!
        if buffer.ends_with(';') {
            rl.add_history_entry(&buffer)?;
            // Remove semicolon before returning
            return Ok(buffer.trim_end_matches(';').trim().to_string());
        }

        // 2. Single keyword commands (don't need semicolon)
        if line_number == 1 {
            let upper = buffer.trim().to_uppercase();
            if matches!(upper.as_str(), "QUIT" | "EXIT" | "HELP" | "STATUS") {
                rl.add_history_entry(&buffer)?;
                return Ok(buffer);
            }
        }

        // Otherwise, continue reading next line
    }
}
