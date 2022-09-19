//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use colored::Colorize;
use mate_rs::mate::Mate;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut input: String = String::new();

        print!("{} ", format!(">>>").bold()); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]").bold().red(), e.to_string().red()),
            Ok(_) => {
                let result = Mate::calculate(input.as_str());
                match result {
                    Ok(v) => println!("{} \n", v.to_string().green().bold()),
                    Err(e) => {
                        println!("{} {} \n", format!("[!]").bold().red(), e.to_string().red())
                    }
                };
            }
        };
    }
}
