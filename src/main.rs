use std::io::{stdin, stdout, Write};

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum StatementError {
    // Success,
    UnrecognizedStatement,
    InvalidStatement,
}

fn main() {
    loop {
        print!("rdbms> ");
        let _ = stdout().flush(); // Flush the buffer immediately

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        let input = input.trim_end(); // removes '\r\n' or '\n'

        if input.starts_with('.') {
            // Meta command
            // e.g, .exit, .help, .list
            match parse_meta_command(&input) {
                MetaCommandResult::Success => {
                    execute_meta_command(&input);
                    break;
                }
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command: {}", input);
                }
            }
        } else {
            // Statement
            println!("Statement: {}", input);
            match parse_statement(&input) {
                Ok(statement) => {
                    execute_statement(statement);
                }
                Err(error) => match error {
                    StatementError::UnrecognizedStatement => {
                        println!("Unrecognized keyword at start of '{}'", input);
                    }
                    StatementError::InvalidStatement => {
                        println!("Syntax error. Could not parse statement '{}'", input);
                    }
                },
            }
        }
    }
}

fn parse_meta_command(input: &str) -> MetaCommandResult {
    let meta_commands = [".exit", ".help", ".list"];

    if meta_commands.contains(&input) {
        MetaCommandResult::Success
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}

fn execute_meta_command(input: &str) {
    match input {
        ".exit" => {
            println!("Goodbye.");
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command '{}'", input);
        }
    }
}

fn parse_statement(input: &str) -> Result<(Vec<&str>), StatementError> {
    let input_statements = input.split_whitespace().collect::<Vec<&str>>();
    return Ok(input_statements);
    // let statements = ["select", "insert", "update", "delete"];

    // if statements.contains(&input) {
    //     StatementResult::Success
    // } else {
    //     StatementError::UnrecognizedStatement
    // }
}

fn execute_statement(input: Vec<&str>) {
    println!("Executing statement: {:?}", input);
}
