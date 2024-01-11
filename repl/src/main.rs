//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use colored::Colorize;
use mate_rs::{calculator::Calculator, lexer::Lexer, monitor::Monitor, token::Token};
use std::{io::{stdin, stdout, Write}, env};

fn main() {
    // TODO: take me from command line arguments
    let log_tokens: bool = false;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let input = args
            .iter()
            .skip(1)
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        return execute_calculator(input.as_str(), log_tokens);
    }

    loop {
        let mut input: String = String::new();

        print!("{} ", format!(">>>").bold()); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]").bold().red(), e.to_string().red()),
            Ok(_) => execute_calculator(input.clone().as_str(), log_tokens),
        };
    }
}

fn execute_calculator(input: &str, log_tokens: bool) {
    let sub = match Lexer::lex(input) {
        Ok(tt) => tt,
        Err(e) => return print_err(e.to_string()),
    };

    if log_tokens {
        print_tokens(sub.tokens.clone())
    }

    let result = Calculator::calculate(sub, &input);
    match result {
        Ok(v) => println!("{} \n", v.to_string().green().bold()),
        Err(e) => print_err(e.to_string()),
    };
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
