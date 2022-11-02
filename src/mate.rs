//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::{calculator::Calculator, errors::Error, lexer::Lexer};

// A main structure that takes string input, parses it via [Lexer],
// and calculates result via [Calculator].
pub struct Mate {}

impl Mate {
    // Takes a arithmetic expression as string, parses it to tokens, and calculates final result.
    // Detailed descriptions could be viewed at lexer source file and calculator source file.
    pub fn calculate(input: &str) -> Result<f64, Error> {
        let sub = match Lexer::lex(input.clone()) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        Calculator::calculate(sub, input.clone())
    }
}
