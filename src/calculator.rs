//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use std::collections::HashMap;

use crate::{
    errors::Error,
    monitor::Monitor,
    token::{Sub, SubMethod, Token, TokenType},
    utils::ChUtils,
};

pub struct Calculator {}
impl Calculator {
    // Calculate, is token-to-number compiler of application.
    // Loops through input and returns final answer.
    //
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
    //
    pub fn calculate(sub: Sub, input: &str) -> Result<f64, Error> {
        let mut result: f64 = 0.0;
        let tokens: Vec<Token> = sub.clone().tokens.clone();

        if tokens.is_empty() {
            return Err(Error::empty_tokens());
        }

        // In case of having one but sub-expression token
        // We have to use its sub tokens to calculate.
        if tokens.len() == 1 && tokens[0].clone().is_sub_exp() {
            return Calculator::calculate(tokens[0].clone().sub, input.clone());
        }

        let mut i: usize = 0;
        while i <= tokens.len() {
            if i > tokens.len() - 1 {
                let point = tokens.last().unwrap().index.1;
                return Err(Error::missing_some_tokens(input.clone().to_string(), point));
            }

            let token: Token = tokens[i].clone();
            if token.clone().is_illegal() {
                return Err(Error::illeagal_token(input.clone().to_string(), token));
            }

            let mut y: f64 = 0.0;
            let x: f64 = result.clone();
            let operation: TokenType =
                match Calculator::take_operation(i, tokens.clone(), input.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

            if token.clone().is_number() {
                y = match token.clone().literal.as_str().parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(Error::cannot_parse_to_number(
                            input.to_string(),
                            token.clone(),
                        ))
                    }
                };
            } else if token.clone().is_sub_exp() {
                y = match Calculator::calculate(token.clone().sub, input.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
            }

            // Update res by current X/Y/O.
            result = Calculator::execute_operation(x, y, operation);
            i += 2;
        }

        let result = match sub.clone().method {
            SubMethod::PAREN => result,
            SubMethod::ABS => result.abs(),
        };

        Ok(result)
    }

    fn take_operation(i: usize, tokens: Vec<Token>, input: &str) -> Result<TokenType, Error> {
        // At first loop, operation must to be PLUS.
        // Because, the [res] is zero and we have to
        // add some value before starting working on it.
        if i == 0 {
            return Ok(TokenType::PLUS);
        }

        if tokens.len() - 1 < i - 1 {
            let point = tokens.last().unwrap().index.1;
            return Err(Error::missing_some_tokens(input.clone().to_string(), point));
        }

        if tokens.clone()[i - 1].clone().is_illegal() {
            return Err(Error::illeagal_token(
                input.to_string(),
                tokens.clone()[i - 1].clone(),
            ));
        }

        let is_plus_or_minus = tokens.clone()[i - 1].clone().is_plus_or_minus();
        let is_div_or_prod = tokens.clone()[i - 1].clone().is_div_or_prod();
        let is_percentage = tokens.clone()[i - 1].clone().is_percentage();
        let is_power = tokens.clone()[i - 1].clone().is_power();

        if is_plus_or_minus || is_div_or_prod || is_percentage || is_power {
            return Ok(tokens.clone()[i - 1].clone().typ);
        }

        return Err(Error::invalid_order());
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
            (TokenType::PLUS.to_string(0), x + y),
            (TokenType::MINUS.to_string(0), x - y),
            (TokenType::PRODUCT.to_string(0), x * y),
            (TokenType::DIVIDE.to_string(0), x / y),
            (TokenType::PERCENTAGE.to_string(0), (x / 100.0) * y),
            (TokenType::POWER.to_string(0), f64::powf(x, y)),
        ]);

        match operations.get(&operation.to_string(0)) {
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
            ("", Err(Error::empty_tokens())),
            ("-25 + 5", Ok(-20.0)),
            ("42 * 5", Ok(210.0)),
            ("- 2 * 7 / 5 + - 20 / - 5", Ok(1.2000000000000002)),
            ("(5 - 9) - 10", Ok(-14.0)),
            ("((10 - 5) - (10 / 2)) / 2", Ok(0.0)),
            ("(2 + 5) * (5 - 9 / (8 - 5)) + 5", Ok(19.0)),
            ("50 % 5", Ok(2.5)),
            ("5 ^ 2", Ok(25.0)),
            ("4 ^ 2 ^ 2 + 4", Ok(260.0)),
            ("2(20 + 3 ^ 3) ^ 2 + 82", Ok(4500.0)),
            ("[2 - 12] - 10", Ok(0.0)),
            ("7 * [5 - 9 / [5 - 8]]", Ok(14.0)),
            ("7 * [5 - 9 / [5 - 8]]", Ok(14.0)),
        ]);

        for (input, expected) in test_data {
            let sub = match Lexer::lex(input.clone()) {
                Ok(v) => v,
                Err(_) => Sub::empty(),
            };

            let result = Calculator::calculate(sub, input.clone());
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
