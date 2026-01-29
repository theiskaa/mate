//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use colored::Colorize;
use mate_rs::{
    calculator::Calculator, environment::Environment, lexer::Lexer, monitor::Monitor, token::Token,
};
use std::{
    env,
    io::{stdin, stdout, Write},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut log_tokens = false;
    let mut expression: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--tokens" => log_tokens = true,
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("mate v{VERSION}");
                return;
            }
            arg if !arg.starts_with('-') => {
                expression = Some(args[i..].join(" "));
                break;
            }
            _ => {
                eprintln!("{} Unknown option: {}", "[!]".bold().red(), args[i].red());
                return;
            }
        }
        i += 1;
    }

    // For one-off expressions, use empty environment
    if let Some(expr) = expression {
        let mut env = Environment::new();
        execute_calculator(&expr, log_tokens, &mut env);
        return;
    }

    // REPL mode - maintain environment across expressions
    let mut env = Environment::new();

    println!(
        "{}\n",
        "mate - A simple arithmetic expression interpreter".bold()
    );
    println!("Type 'help' for available commands, 'quit' to exit.\n");

    loop {
        print!("{} ", ">>>".bold());
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {}\n", "[!]".bold().red(), e.to_string().red()),
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match trimmed.to_lowercase().as_str() {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "h" | "?" => {
                        print_repl_help();
                    }
                    "clear" | "cls" => {
                        print!("\x1B[2J\x1B[1;1H");
                        let _ = stdout().flush();
                    }
                    "tokens" => {
                        log_tokens = !log_tokens;
                        println!(
                            "Token logging: {}\n",
                            if log_tokens { "enabled" } else { "disabled" }
                        );
                    }
                    "vars" | "variables" => {
                        print_variables(&env);
                    }
                    "reset" => {
                        env.clear();
                        println!("All variables cleared.\n");
                    }
                    _ => execute_calculator(trimmed, log_tokens, &mut env),
                }
            }
        };
    }
}

fn execute_calculator(input: &str, log_tokens: bool, env: &mut Environment) {
    let sub = match Lexer::lex(input) {
        Ok(tt) => tt,
        Err(e) => return print_err(&e.to_string()),
    };

    if log_tokens {
        print_tokens(&sub.tokens);
    }

    match Calculator::calculate_with_env(sub, input, env) {
        Ok(v) => println!("{}\n", v.to_string().green().bold()),
        Err(e) => print_err(&e.to_string()),
    };
}

fn print_err(msg: &str) {
    println!("{} {}\n", "[!]".bold().red(), msg.red())
}

fn print_variables(env: &Environment) {
    let names = env.names();
    if names.is_empty() {
        println!("No variables defined.\n");
        return;
    }
    println!("\n{}", "Variables:".bold());
    for name in names {
        if let Some(value) = env.get(name) {
            println!("  {} = {}", name.cyan(), value);
        }
    }
    println!();
}

fn print_tokens(tt: &[Token]) {
    println!("---------------");
    for t in tt.iter() {
        println!("{}", t.to_string(0));
    }
    println!("---------------\n");
}

fn print_help() {
    println!("mate - A simple arithmetic expression interpreter\n");
    println!("USAGE:");
    println!("    mate [OPTIONS] [EXPRESSION]\n");
    println!("OPTIONS:");
    println!("    -h, --help       Print this help message");
    println!("    -v, --version    Print version information");
    println!("    -t, --tokens     Show parsed tokens\n");
    println!("EXAMPLES:");
    println!("    mate 2 + 2");
    println!("    mate \"(5 + 3) * 2\"");
    println!("    mate -t \"10 / 2\"");
    println!("    mate \"sqrt(16) + 5\"");
    println!("    mate              # Start interactive REPL\n");
    println!("SUPPORTED OPERATIONS:");
    println!("    +    Addition");
    println!("    -    Subtraction");
    println!("    *    Multiplication");
    println!("    /    Division");
    println!("    %    Percentage (e.g., 50 % 10 = 5)");
    println!("    ^    Power (e.g., 2 ^ 3 = 8)");
    println!("    !    Factorial (e.g., 5! = 120)");
    println!("    =    Assignment (e.g., x = 5)");
    println!("    ()   Parentheses for grouping");
    println!("    []   Absolute value (e.g., [-5] = 5)\n");
    println!("VARIABLES:");
    println!("    x = 5        Assign value to variable");
    println!("    x + 2        Use variable in expression");
    println!("    y = x * 2    Assign expression result\n");
    println!("MATH FUNCTIONS:");
    println!("    sqrt(x)   Square root");
    println!("    sin(x)    Sine (radians)");
    println!("    cos(x)    Cosine (radians)");
    println!("    tan(x)    Tangent (radians)");
    println!("    log(x)    Base-10 logarithm");
    println!("    ln(x)     Natural logarithm");
    println!("    exp(x)    Exponential (e^x)");
    println!("    floor(x)  Round down");
    println!("    ceil(x)   Round up");
    println!("    round(x)  Round to nearest");
}

fn print_repl_help() {
    println!("\n{}", "Available commands:".bold());
    println!("  help, h, ?    Show this help message");
    println!("  quit, exit, q Exit the REPL");
    println!("  clear, cls    Clear the screen");
    println!("  tokens        Toggle token display");
    println!("  vars          Show all variables");
    println!("  reset         Clear all variables\n");
    println!("{}", "Supported operations:".bold());
    println!("  +    Addition");
    println!("  -    Subtraction");
    println!("  *    Multiplication");
    println!("  /    Division");
    println!("  %    Percentage (e.g., 50 % 10 = 5)");
    println!("  ^    Power (e.g., 2 ^ 3 = 8)");
    println!("  !    Factorial (e.g., 5! = 120)");
    println!("  =    Assignment (e.g., x = 5)");
    println!("  ()   Parentheses for grouping");
    println!("  []   Absolute value (e.g., [-5] = 5)\n");
    println!("{}", "Variables:".bold());
    println!("  x = 5        Assign value to variable");
    println!("  x + 2        Use variable in expression");
    println!("  y = x * 2    Assign expression result\n");
    println!("{}", "Math functions:".bold());
    println!("  sqrt, sin, cos, tan, log, ln, exp, floor, ceil, round\n");
    println!("{}", "Examples:".bold());
    println!("  2 + 2");
    println!("  x = 10");
    println!("  x * 2 + 5");
    println!("  radius = 7");
    println!("  3.14159 * radius ^ 2\n");
}
