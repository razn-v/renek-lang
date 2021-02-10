use std::io::{stdin, stdout, Write};

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

pub fn interactive() {
    loop {
        let mut lexer = Lexer::new();
        let input = get_input();

        match lexer.lex(input) {
            Ok(tokens) => {
                println!("{:?}", tokens);
                println!();

                let mut parser = Parser::new(tokens.clone().to_vec());
                parser.parse();
            },
            Err(err) => println!("{}", err),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    print!("→ ");
    stdout().flush().expect("Flush failed!");

    match stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim_end().ends_with('{') {
                loop {
                    let mut line = String::new();

                    print!("... ");
                    stdout().flush().expect("Flush failed!");

                    stdin()
                        .read_line(&mut line)
                        .expect("An error occured while trying to read input");

                    if line.trim_end() == "" {
                        break;
                    }

                    input.push_str(&line);
                }
            }
            return input;
        }
        Err(err) => panic!("An error occured while trying to read input: {}", err)
    }
}