//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::{
    errors::Error,
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
        let tokens = &sub.tokens;

        if tokens.is_empty() {
            return Err(Error::empty_tokens());
        }

        if tokens.len() == 1 && tokens[0].is_sub_exp() {
            return Calculator::calculate(tokens[0].sub.clone(), input);
        }

        let mut i: usize = 0;
        while i <= tokens.len() {
            if i > tokens.len() - 1 {
                let point = tokens.last().map(|t| t.index.1).unwrap_or(0);
                return Err(Error::missing_some_tokens(input.to_string(), point));
            }

            let token = &tokens[i];
            if token.is_illegal() {
                return Err(Error::illegal_token(input.to_string(), token.clone()));
            }

            let mut y: f64 = 0.0;
            let x: f64 = result;
            let operation: TokenType = Calculator::take_operation(i, tokens, input)?;

            let y_point = token.index.1;
            if token.is_number() {
                y = match token.literal.as_str().parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(Error::cannot_parse_to_number(
                            input.to_string(),
                            token.clone(),
                        ))
                    }
                };
            } else if token.is_sub_exp() {
                y = Calculator::calculate(token.sub.clone(), input)?;
            } else if token.is_function() {
                // Function token - the next token should be its argument
                if i + 1 >= tokens.len() {
                    return Err(Error::missing_some_tokens(input.to_string(), y_point));
                }
                let arg_token = &tokens[i + 1];
                let arg = if arg_token.is_sub_exp() {
                    Calculator::calculate(arg_token.sub.clone(), input)?
                } else if arg_token.is_number() {
                    arg_token.literal.parse::<f64>().map_err(|_| {
                        Error::cannot_parse_to_number(input.to_string(), arg_token.clone())
                    })?
                } else {
                    return Err(Error::missing_some_tokens(input.to_string(), arg_token.index.1));
                };
                y = Calculator::execute_function(token.typ.clone(), arg, input, y_point)?;
                i += 1; // Skip the argument token
            }

            result = Calculator::execute_operation(x, y, operation, input, y_point)?;
            i += 2;
        }

        let result = match sub.method {
            SubMethod::PAREN => result,
            SubMethod::ABS => result.abs(),
        };

        Ok(result)
    }

    fn take_operation(i: usize, tokens: &[Token], input: &str) -> Result<TokenType, Error> {
        if i == 0 {
            return Ok(TokenType::PLUS);
        }

        if i > tokens.len() {
            let point = tokens.last().map(|t| t.index.1).unwrap_or(0);
            return Err(Error::missing_some_tokens(input.to_string(), point));
        }

        let prev_token = &tokens[i - 1];
        if prev_token.is_illegal() {
            return Err(Error::illegal_token(input.to_string(), prev_token.clone()));
        }

        if prev_token.is_plus_or_minus()
            || prev_token.is_div_or_prod()
            || prev_token.is_percentage()
            || prev_token.is_power()
        {
            return Ok(prev_token.typ.clone());
        }

        Err(Error::invalid_order())
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
    fn execute_operation(
        x: f64,
        y: f64,
        operation: TokenType,
        input: &str,
        y_point: i32,
    ) -> Result<f64, Error> {
        // Check for division by zero
        if operation == TokenType::DIVIDE && y == 0.0 {
            return Err(Error::division_by_zero(input.to_string(), y_point));
        }

        let result = match operation {
            TokenType::PLUS => x + y,
            TokenType::MINUS => x - y,
            TokenType::PRODUCT => x * y,
            TokenType::DIVIDE => x / y,
            TokenType::PERCENTAGE => (x / 100.0) * y,
            TokenType::POWER => f64::powf(x, y),
            _ => 0.0,
        };

        Ok(result)
    }

    // Executes a math function on the given argument.
    fn execute_function(
        func: TokenType,
        arg: f64,
        input: &str,
        point: i32,
    ) -> Result<f64, Error> {
        let result = match func {
            TokenType::SQRT => {
                if arg < 0.0 {
                    return Err(Error::new(format!(
                        "error: cannot take square root of negative number: {arg}"
                    )));
                }
                arg.sqrt()
            }
            TokenType::SIN => arg.sin(),
            TokenType::COS => arg.cos(),
            TokenType::TAN => arg.tan(),
            TokenType::LOG => {
                if arg <= 0.0 {
                    return Err(Error::new(format!(
                        "error: logarithm undefined for non-positive number: {arg}"
                    )));
                }
                arg.log10()
            }
            TokenType::LN => {
                if arg <= 0.0 {
                    return Err(Error::new(format!(
                        "error: natural log undefined for non-positive number: {arg}"
                    )));
                }
                arg.ln()
            }
            TokenType::EXP => arg.exp(),
            TokenType::FLOOR => arg.floor(),
            TokenType::CEIL => arg.ceil(),
            TokenType::ROUND => arg.round(),
            _ => {
                return Err(Error::missing_some_tokens(input.to_string(), point));
            }
        };

        Ok(result)
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
            let sub = match Lexer::lex(input) {
                Ok(v) => v,
                Err(_) => Sub::empty(),
            };

            let result = Calculator::calculate(sub, input);
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
            let result = Calculator::execute_operation(args.0, args.1, args.2, "test", 0);
            assert_eq!(result.unwrap().to_string(), expected);
        }
    }

    #[test]
    fn division_by_zero() {
        let result = Calculator::execute_operation(10.0, 0.0, TokenType::DIVIDE, "10 / 0", 5);
        assert!(result.is_err());

        let test_cases: Vec<&str> = vec![
            "10 / 0",
            "5 + 10 / 0",
            "(10 + 5) / 0",
            "100 / (5 - 5)",
            "42 * 2 / 0",
        ];

        for input in test_cases {
            let sub = match Lexer::lex(input) {
                Ok(v) => v,
                Err(_) => Sub::empty(),
            };

            let result = Calculator::calculate(sub, input);
            assert!(
                result.is_err(),
                "Expected error for division by zero in: {}",
                input
            );
        }
    }

    #[test]
    fn valid_division() {
        let test_cases: HashMap<&str, f64> = HashMap::from([
            ("10 / 2", 5.0),
            ("100 / 4", 25.0),
            ("9 / 3", 3.0),
            ("1 / 2", 0.5),
        ]);

        for (input, expected) in test_cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn percentage_operator_precedence() {
        let test_cases: HashMap<&str, f64> = HashMap::from([
            ("50 % 5", 2.5),
            ("100 % 10", 10.0),
            ("200 % 50", 100.0),
            ("10 + 50 % 5", 12.5),
            ("50 % 5 + 10", 12.5),
            ("100 % 10 + 50 % 5", 12.5),
            ("(100 + 100) % 25", 50.0),
            ("50 % 10 * 2", 10.0),
            ("2 * 50 % 10", 10.0),
        ]);

        for (input, expected) in test_cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn edge_cases() {
        let valid_cases: HashMap<&str, f64> = HashMap::from([
            ("0", 0.0),
            ("0.0", 0.0),
            ("0.5", 0.5),
            (".5", 0.5),
            ("-0.5", -0.5),
            ("1.5 + 2.5", 4.0),
            ("((5))", 5.0),
            ("(((10 + 5)))", 15.0),
            ("[[-5]]", 5.0),
            ("2 * -3", -6.0),
            ("10 / -2", -5.0),
            ("-10 / -2", 5.0),
            ("2 ^ 10", 1024.0),
            ("2 ^ 0", 1.0),
            ("0 ^ 5", 0.0),
            ("1000000 + 1000000", 2000000.0),
            ("0.001 + 0.002", 0.003),
            ("10 - 10", 0.0),
            ("5 * 0", 0.0),
            ("   5 + 5   ", 10.0),
            ("5+5", 10.0),
            ("(5)", 5.0),
            ("[5]", 5.0),
            ("[-5]", 5.0),
        ]);

        for (input, expected) in valid_cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input);
            assert!(
                result.is_ok(),
                "Expected success for input: {}, got error: {:?}",
                input,
                result
            );
            let value = result.unwrap();
            assert!(
                (value - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                value
            );
        }
    }

    #[test]
    fn error_cases() {
        let error_cases: Vec<&str> = vec![
            "",
            "()",
            "[]",
            "(5 + 3",
            "5 + 3)",
            "[5 + 3",
            "5 + 3]",
            "( ]",
            "[ )",
            "((5 + 3)",
            "5 / 0",
            "(5 - 5) / (2 - 2)",
        ];

        for input in error_cases {
            let result = match Lexer::lex(input) {
                Ok(sub) => Calculator::calculate(sub, input),
                Err(e) => Err(e),
            };
            assert!(
                result.is_err(),
                "Expected error for input: {}, got: {:?}",
                input,
                result
            );
        }
    }

    #[test]
    fn complex_nested_expressions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("((2 + 3) * (4 - 1)) / 3", 5.0),
            ("2 * (3 + 4 * (5 - 2))", 30.0),
            ("[5 - 10] + [3 - 8]", 10.0),
            ("([5 - 10] + [3 - 8]) * 2", 20.0),
            ("2 ^ (1 + 1) ^ 2", 16.0),
            ("100 % (10 + 10)", 20.0),
            ("(2 + 3) ^ 2", 25.0),
            ("[-5 + 2] * [3 - 7]", 12.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn math_functions_basic() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt(16)", 4.0),
            ("sqrt(25)", 5.0),
            ("sqrt(0)", 0.0),
            ("sqrt(1)", 1.0),
            ("sqrt(2)", std::f64::consts::SQRT_2),
            ("sin(0)", 0.0),
            ("cos(0)", 1.0),
            ("tan(0)", 0.0),
            ("ln(1)", 0.0),
            ("exp(0)", 1.0),
            ("exp(1)", std::f64::consts::E),
            ("log(10)", 1.0),
            ("log(100)", 2.0),
            ("floor(3.7)", 3.0),
            ("floor(3.2)", 3.0),
            ("floor(-3.7)", -4.0),
            ("ceil(3.2)", 4.0),
            ("ceil(3.7)", 4.0),
            ("ceil(-3.2)", -3.0),
            ("round(3.5)", 4.0),
            ("round(3.4)", 3.0),
            ("round(-3.5)", -4.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn math_functions_in_expressions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt(16) + 5", 9.0),
            ("5 + sqrt(16)", 9.0),
            ("sqrt(16) * 2", 8.0),
            ("2 * sqrt(16)", 8.0),
            ("sqrt(16) + sqrt(9)", 7.0),
            ("sqrt(9 + 7)", 4.0),
            ("sqrt(16) ^ 2", 16.0),
            ("floor(3.7) + ceil(3.2)", 7.0),
            ("sin(0) + cos(0)", 1.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn math_functions_nested() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt(sqrt(81))", 3.0),
            ("sqrt(sqrt(256))", 4.0),
            ("floor(sqrt(10))", 3.0),
            ("ceil(sqrt(10))", 4.0),
            ("round(sqrt(10))", 3.0),
            ("sqrt(floor(16.9))", 4.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn math_functions_case_insensitive() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("SQRT(16)", 4.0),
            ("Sqrt(16)", 4.0),
            ("SIN(0)", 0.0),
            ("Sin(0)", 0.0),
            ("COS(0)", 1.0),
            ("Cos(0)", 1.0),
            ("FLOOR(3.7)", 3.0),
            ("Floor(3.7)", 3.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn math_functions_errors() {
        let error_cases: Vec<&str> = vec![
            "sqrt(-1)",
            "ln(0)",
            "ln(-1)",
            "log(0)",
            "log(-1)",
        ];

        for input in error_cases {
            let result = match Lexer::lex(input) {
                Ok(sub) => Calculator::calculate(sub, input),
                Err(e) => Err(e),
            };
            assert!(
                result.is_err(),
                "Expected error for input: {}, got: {:?}",
                input,
                result
            );
        }
    }

    #[test]
    fn functions_with_nested_parentheses() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt((5 + 3) * 2)", 4.0),
            ("sqrt((10 - 2) * 2)", 4.0),
            ("sqrt(((16)))", 4.0),
            ("floor((10 + 5) / 4)", 3.0),
            ("ceil((10 + 5) / 4)", 4.0),
            ("round((7 + 3) / 3)", 3.0),
            ("sin((0))", 0.0),
            ("cos((0))", 1.0),
            ("sqrt((2 + 2) * (3 + 1))", 4.0),
            ("log((50 + 50))", 2.0),
            ("ln((1))", 0.0),
            ("exp((0))", 1.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn functions_inside_parentheses() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("(sqrt(16))", 4.0),
            ("(sqrt(16) + 5)", 9.0),
            ("(sqrt(16) + 5) * 2", 18.0),
            ("(sqrt(16) + sqrt(9))", 7.0),
            ("(sqrt(16) + sqrt(9)) * 2", 14.0),
            ("((sqrt(16)))", 4.0),
            ("(sin(0) + cos(0))", 1.0),
            ("(sin(0) + cos(0)) * 10", 10.0),
            ("(floor(3.7) + ceil(3.2))", 7.0),
            ("2 * (sqrt(16) + 1)", 10.0),
            ("(sqrt(9) + sqrt(16)) ^ 2", 49.0),
            ("(ln(1) + exp(0))", 1.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn deeply_nested_functions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt(sqrt(256))", 4.0),
            ("sqrt(sqrt(sqrt(256)))", 2.0),
            ("floor(ceil(3.2))", 4.0),
            ("ceil(floor(3.7))", 3.0),
            ("round(sqrt(10))", 3.0),
            ("sqrt(floor(16.9))", 4.0),
            ("floor(sqrt(10) + ceil(3.2))", 7.0),
            ("sin(cos(0))", 0.8414709848078965),
            ("cos(sin(0))", 1.0),
            ("sqrt((sqrt(16) + sqrt(9)) * 2)", 14.0_f64.sqrt()),
            ("exp(ln(2))", 2.0),
            ("ln(exp(2))", 2.0),
            ("log(exp(ln(10)))", 1.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-9,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn functions_with_absolute_value() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt([-16 + 32])", 4.0),
            ("sqrt([-9 + 25])", 4.0),
            ("[sqrt(16) - 10]", 6.0),
            ("[sin(0) - 5]", 5.0),
            ("sqrt([5 - 21])", 4.0),
            ("[floor(3.7) - ceil(8.2)]", 6.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn complex_mixed_expressions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("2 * sqrt(16) + 3 * sqrt(9)", 17.0),
            ("sqrt(16) * sqrt(9)", 12.0),
            ("sqrt(16) / sqrt(4)", 2.0),
            ("sqrt(16) ^ 2 + sqrt(9) ^ 2", 25.0),
            ("(sqrt(16) + sqrt(9)) * (sqrt(4) + sqrt(1))", 21.0),
            ("floor(3.7) * ceil(2.1) + round(2.5)", 12.0),
            ("sqrt(16) + floor(5.9) - ceil(2.1)", 6.0),
            ("2 ^ sqrt(16)", 16.0),
            ("sqrt(2 ^ 4)", 4.0),
            ("sin(0) * 100 + cos(0) * 50", 50.0),
            ("exp(0) + ln(1) + log(1)", 1.0),
            ("sqrt(sqrt(256) + sqrt(81))", 5.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-9,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn functions_with_operations_as_arguments() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("sqrt(10 + 6)", 4.0),
            ("sqrt(20 - 4)", 4.0),
            ("sqrt(4 * 4)", 4.0),
            ("sqrt(32 / 2)", 4.0),
            ("sqrt(2 ^ 4)", 4.0),
            ("floor(10 / 3)", 3.0),
            ("ceil(10 / 3)", 4.0),
            ("round(10 / 3)", 3.0),
            ("log(10 * 10)", 2.0),
            ("ln(exp(1) * exp(1))", 2.0),
            ("sqrt(sqrt(16) * sqrt(16))", 4.0),
            ("floor(sqrt(16) + 0.5)", 4.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }
}
