#![allow(warnings)]
use chrono::{Local, Utc};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::time::Instant;
mod encryption;
mod engine;
mod error;
mod interpreter;
mod p_std;
use p_std::uid::Uid;
mod session;
mod statements;
mod storage;
use crate::engine::Executor;
use hex;
use session::offSessionConn::SessionConn;
use zeroize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let mut session_status = SessionConn::new()?;
    loop {
        let command = match read_command(&mut rl) {
            Ok(cmd) => {
                let start = Instant::now();
                match Executor::execute(&cmd, &mut session_status) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR: {}", e);
                    }
                }
                let end = start.elapsed();
                println!("\nTime elapsed: {:.3} ms", end.as_secs_f64() * 1000.0);
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

        let prompt = if line_number == 1 || buffer.is_empty() {
            "sqpwman> "
        } else {
            "      -> "
        };

        // Read one line
        let line = rl.readline(prompt)?;

        if !buffer.is_empty() {
            buffer.push(' ');
        }
        buffer.push_str(line.trim());

        if buffer.ends_with(';') {
            rl.add_history_entry(&buffer)?;

            return Ok(buffer.trim_end_matches(';').trim().to_string());
        }

        if line_number == 1 {
            let upper = buffer.trim().to_uppercase();
            if matches!(upper.as_str(), "QUIT" | "EXIT" | "HELP" | "STATUS") {
                rl.add_history_entry(&buffer)?;
                return Ok(buffer);
            }
        }
    }
}
