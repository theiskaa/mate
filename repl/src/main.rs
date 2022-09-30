//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use colored::Colorize;
use mate_rs::{calculator::Calculator, lexer::Lexer, monitor::Monitor, token::Token};
use std::io::{stdin, stdout, Write};

fn main() {
    // TODO: take me from command line arguments
    let log_tokens: bool = true;

    loop {
        let mut input: String = String::new();

        print!("{} ", format!(">>>").bold()); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]").bold().red(), e.to_string().red()),
            Ok(_) => {
                let tokens: Vec<Token> = match Lexer::lex(input.as_str()) {
                    Ok(tt) => tt,
                    Err(e) => return print_err(e.to_string()),
                };

                if log_tokens {
                    print_tokens(tokens.clone())
                }

                let result = Calculator::calculate(tokens.clone());
                match result {
                    Ok(v) => println!("{} \n", v.to_string().green().bold()),
                    Err(e) => print_err(e.to_string()),
                };
            }
        };
    }
}

fn print_err(msg: String) {
    println!(
        "{} {} \n",
        format!("[!]").bold().red(),
        msg.to_string().red()
    )
}

fn print_tokens(tt: Vec<Token>) {
    println!("---------------");
    for t in tt.iter() {
        println!("{}", t.to_string(0));
    }
    println!("---------------\n");
}
