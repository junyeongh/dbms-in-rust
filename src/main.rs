use std::io::{stdin, stdout, Write};

fn main() {
    // use `loop` for infinite loop instead of while true
    loop {
        // print_prompt();
        print!("rdbms> ");
        let _ = stdout().flush(); // Flush the buffer immediately

        let mut input = String::new();
        let result = stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                // remove trailing newline
                let input = input.trim_end();
                if input == ".exit" {
                    
                    break;
                } else {
                    println!("Unrecognized command '{}'", input);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
