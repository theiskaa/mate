//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use std::collections::HashMap;

use crate::{
    errors::Error,
    monitor::Monitor,
    token::{Token, TokenType},
    utils::ChUtils,
};

pub struct Calculator {}
impl Calculator {
    // Calculate, is token-to-number compiler of application.
    // Loops through input and returns final answer.
    // If there is an error, answer will be "ZERO", and error would be provided.
    // The input argument can be passed from function arguments, if it's not provided
    // from arguments, function uses default input of Calculator ──▶ [l.Input]
    //  ╭────────╮ ╭───────────╮ ╭────────╮
    //  │ NUMBER │ │ OPERATION │ │ NUMBER │
    //  ╰────────╯ ╰───────────╯ ╰────────╯
    //       ╰───╮       │        ╭───╯
    //           ▼       ▼        ▼
    //           X  [+, -, *, /]  Y
    //
    // For instance NUMBER(I) is 6, NUMBER(II) is 7,
    // and the operation is PRODUCT(Multiplication). Result of function would be ──▶ 6 * 7 = 42
    pub fn calculate(tokens: Vec<Token>) -> Result<f64, Error<'static>> {
        let mut result: f64 = 0.0;

        if tokens.clone().is_empty() {
            return Err(Error::new("Cannot calculate an empty token expressions"));
        }

        // In case of having one but sub-expression token
        // We have to use its sub tokens to calculate.
        if tokens.clone().len() == 1 && tokens.clone()[0].clone().is_sub_exp() {
            return Calculator::calculate(tokens.clone()[0].clone().sub_tokens);
        }

        let mut i: usize = 0;
        while i <= tokens.len() {
            let token: Token = tokens[i].clone();

            let mut y: f64 = 0.0;
            let x: f64 = result.clone();
            let operation: TokenType;

            if token.clone().is_illegal() {
                return Err(Error::new("Found an illegal character token"));
            }

            if token.clone().is_number() {
                y = match token.clone().literal.as_str().parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => return Err(Error::new("Cannot parse token literal to a number")),
                };
            } else if token.clone().is_sub_exp() {
                y = match Calculator::calculate(token.clone().sub_tokens) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
            }

            // At first loop, operation must to be PLUS.
            // Because, res is zero and we have to
            // add some value before starting working on it.
            if i == 0 {
                operation = TokenType::PLUS;
            } else if tokens.clone()[i - 1].clone().is_plus_or_minus()
                || tokens.clone()[i - 1].clone().is_div_or_prod()
            {
                operation = tokens.clone()[i - 1].clone().typ;
            } else {
                return Err(Error::new("Invalid order of token characters"));
            }

            // Update res by current X/Y/O.
            result = Calculator::execute_operation(x, y, operation);
            i += 2;
        }

        Ok(result)
    }

    // Executes the given [operation] for [X] and [Y]
    //
    //  Example:
    //  ╭───╮        ╭───╮        ╭───────────╮
    //  │ X │──▶ 48  │ Y │──▶ 42  │ Operation │──▶ MINUS
    //  ╰───╯        ╰───╯        ╰───────────╯
    //  ────────────────────────────────────────────────
    //                      ╭─────────╮    ╭───╮
    //  Answer would be ──▶ │ 48 - 42 │──▶ │ 6 │
    //                      ╰─────────╯    ╰───╯
    fn execute_operation(x: f64, y: f64, operation: TokenType) -> f64 {
        let operations: HashMap<String, f64> = HashMap::from([
            (TokenType::PLUS.to_string(), x + y),
            (TokenType::MINUS.to_string(), x - y),
            (TokenType::PRODUCT.to_string(), x * y),
            (TokenType::DIVIDE.to_string(), x / y),
        ]);

        match operations.get(&operation.to_string()) {
            None => 0.0,
            Some(v) => v.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use std::collections::HashMap;

    #[test]
    fn calculate() {
        let test_data: HashMap<&str, Result<f64, Error>> = HashMap::from([
            (
                "",
                Err(Error::new("Cannot calculate an empty token expressions")),
            ),
            ("-25 + 5", Ok(-20.0)),
            ("42 * 5", Ok(210.0)),
            ("- 2 * 7 / 5 + - 20 / - 5", Ok(1.2000000000000002)),
            ("(5 - 9) - 10", Ok(-14.0)),
            ("((10 - 5) - (10 / 2)) / 2", Ok(0.0)),
            ("(2 + 5) * (5 - 9 / (8 - 5)) + 5", Ok(19.0)),
        ]);

        for (input, expected) in test_data {
            let tokens = match Lexer::lex(input) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            };

            let result = Calculator::calculate(tokens);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn execute_operation() {
        let test_data: HashMap<String, (f64, f64, TokenType)> = HashMap::from([
            (String::from("42"), (6.0, 7.0, TokenType::PRODUCT)),
            (String::from("-10"), (20.0, 30.0, TokenType::MINUS)),
            (String::from("-20"), (-25.0, 5.0, TokenType::PLUS)),
            (String::from("50"), (200.0, 4.0, TokenType::DIVIDE)),
            (String::from("0"), (0.0, 0.0, TokenType::POINTER)),
        ]);

        for (expected, args) in test_data {
            let result = Calculator::execute_operation(args.0, args.1, args.2);
            assert_eq!(result.to_string(), expected);
        }
    }
}