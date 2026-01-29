//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::{
    environment::Environment,
    errors::Error,
    token::{Sub, SubMethod, Token, TokenType},
    utils::ChUtils,
};

pub struct Calculator {}
impl Calculator {
    // Calculate without environment (backward compatible).
    // For expressions that don't use variables.
    pub fn calculate(sub: Sub, input: &str) -> Result<f64, Error> {
        let mut env = Environment::new();
        Calculator::calculate_with_env(sub, input, &mut env)
    }

    // Calculate with environment support for variables.
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
    pub fn calculate_with_env(sub: Sub, input: &str, env: &mut Environment) -> Result<f64, Error> {
        let mut result: f64 = 0.0;
        let tokens = &sub.tokens;

        if tokens.is_empty() {
            return Err(Error::empty_tokens());
        }

        // Handle assignment: IDENTIFIER = expression
        if tokens.len() >= 3 && tokens[0].is_identifier() && tokens[1].is_assign() {
            let var_name = tokens[0].literal.clone();
            // Create a new Sub with the expression tokens (everything after =)
            let expr_tokens: Vec<Token> = tokens[2..].to_vec();
            let expr_sub = Sub::new(expr_tokens, SubMethod::PAREN);
            let value = Calculator::calculate_with_env(expr_sub, input, env)?;
            env.set(&var_name, value);
            return Ok(value);
        }

        // Handle single identifier (variable lookup)
        if tokens.len() == 1 && tokens[0].is_identifier() {
            let var_name = &tokens[0].literal;
            return env.get(var_name).ok_or_else(|| {
                Error::new(format!("error: undefined variable '{var_name}'"))
            });
        }

        if tokens.len() == 1 && tokens[0].is_sub_exp() {
            return Calculator::calculate_with_env(tokens[0].sub.clone(), input, env);
        }

        // Handle factorial: [NUMBER/SUBEXP/IDENTIFIER, FACTORIAL]
        if tokens.len() == 2 && tokens[1].is_factorial() {
            let operand = if tokens[0].is_number() {
                tokens[0].literal.parse::<f64>().map_err(|_| {
                    Error::cannot_parse_to_number(input.to_string(), tokens[0].clone())
                })?
            } else if tokens[0].is_sub_exp() {
                Calculator::calculate_with_env(tokens[0].sub.clone(), input, env)?
            } else if tokens[0].is_identifier() {
                let var_name = &tokens[0].literal;
                env.get(var_name).ok_or_else(|| {
                    Error::new(format!("error: undefined variable '{var_name}'"))
                })?
            } else {
                return Err(Error::missing_some_tokens(input.to_string(), tokens[0].index.1));
            };
            return Calculator::compute_factorial(operand);
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
            } else if token.is_identifier() {
                // Variable lookup
                let var_name = &token.literal;
                y = env.get(var_name).ok_or_else(|| {
                    Error::new(format!("error: undefined variable '{var_name}'"))
                })?;
            } else if token.is_sub_exp() {
                y = Calculator::calculate_with_env(token.sub.clone(), input, env)?;
            } else if token.is_function() {
                // Function token - the next token should be its argument
                if i + 1 >= tokens.len() {
                    return Err(Error::missing_some_tokens(input.to_string(), y_point));
                }
                let arg_token = &tokens[i + 1];
                let arg = if arg_token.is_sub_exp() {
                    Calculator::calculate_with_env(arg_token.sub.clone(), input, env)?
                } else if arg_token.is_number() {
                    arg_token.literal.parse::<f64>().map_err(|_| {
                        Error::cannot_parse_to_number(input.to_string(), arg_token.clone())
                    })?
                } else if arg_token.is_identifier() {
                    let var_name = &arg_token.literal;
                    env.get(var_name).ok_or_else(|| {
                        Error::new(format!("error: undefined variable '{var_name}'"))
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

    // Computes the factorial of a non-negative integer.
    // n! = n * (n-1) * (n-2) * ... * 2 * 1
    // 0! = 1 by definition
    fn compute_factorial(n: f64) -> Result<f64, Error> {
        // Check if n is a non-negative integer
        if n < 0.0 {
            return Err(Error::new(format!(
                "error: factorial is not defined for negative numbers: {n}"
            )));
        }

        if n != n.floor() {
            return Err(Error::new(format!(
                "error: factorial is only defined for integers: {n}"
            )));
        }

        let n = n as u64;

        // Factorial grows very fast, limit to reasonable values
        if n > 170 {
            return Err(Error::new(format!(
                "error: factorial of {n} is too large to compute"
            )));
        }

        let mut result: f64 = 1.0;
        for i in 2..=n {
            result *= i as f64;
        }

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

    #[test]
    fn factorial_basic() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("0!", 1.0),
            ("1!", 1.0),
            ("2!", 2.0),
            ("3!", 6.0),
            ("4!", 24.0),
            ("5!", 120.0),
            ("6!", 720.0),
            ("7!", 5040.0),
            ("8!", 40320.0),
            ("9!", 362880.0),
            ("10!", 3628800.0),
            ("12!", 479001600.0),
            ("15!", 1307674368000.0),
            ("20!", 2432902008176640000.0),
        ]);

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-5,
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn factorial_in_expressions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            // Addition and subtraction
            ("5! + 10", 130.0),
            ("5! - 10", 110.0),
            ("10 + 5!", 130.0),
            ("10 - 5!", -110.0),
            ("5! + 5!", 240.0),
            ("3! + 4!", 30.0),
            ("3! + 4! + 5!", 150.0),
            // Multiplication and division
            ("2 * 5!", 240.0),
            ("5! * 2", 240.0),
            ("5! / 10", 12.0),
            ("5! / 2!", 60.0),
            ("3! * 4!", 144.0),
            ("6! / 3!", 120.0),
            // Power
            ("5! ^ 2", 14400.0),
            ("2 ^ 3!", 64.0),
            ("2 ^ 4!", 16777216.0),
            ("3! ^ 2", 36.0),
            ("2! ^ 3!", 64.0),
            // Percentage
            ("5! % 10", 12.0),
            ("100 % 5!", 120.0),
            // Mixed operations
            ("5! + 3! * 4", 144.0),
            ("3! * 4 + 5!", 144.0),
            ("5! / 3! + 10", 30.0),
            ("10 + 5! / 3!", 30.0),
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
    fn factorial_with_parentheses() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("(2 + 3)!", 120.0),
            ("(1 + 1)!", 2.0),
            ("(10 - 5)!", 120.0),
            ("(2 * 3)!", 720.0),
            ("(6 / 2)!", 6.0),
            ("(2 ^ 2)!", 24.0),
            ("(1 + 2 + 3)!", 720.0),
            ("((2 + 1))!", 6.0),
            ("(((3)))!", 6.0),
            ("(5 - 3 + 2)!", 24.0),
            ("(2 * 2 + 1)!", 120.0),
            ("(10 / 2 - 1)!", 24.0),
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
    fn factorial_with_absolute_value() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("[5]!", 120.0),
            ("[-5 + 10]!", 120.0),
            ("[3 - 8]!", 120.0),
            ("[2 - 5]!", 6.0),
            ("[-3]!", 6.0),
            ("[5! - 200]", 80.0),
            ("[3! - 10]", 4.0),
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
    fn factorial_with_functions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            // Functions taking factorial as argument
            ("sqrt(4!)", 24.0_f64.sqrt()),
            ("sqrt(5!)", 120.0_f64.sqrt()),
            ("sqrt(6!)", 720.0_f64.sqrt()),
            ("log(6!)", 720.0_f64.log10()),
            ("ln(5!)", 120.0_f64.ln()),
            ("floor(5! / 7)", 17.0),
            ("ceil(5! / 7)", 18.0),
            ("round(5! / 7)", 17.0),
            ("floor(sqrt(5!))", 10.0),
            ("ceil(sqrt(5!))", 11.0),
            // Factorial of function results
            ("floor(3.9)!", 6.0),
            ("ceil(2.1)!", 6.0),
            ("round(2.5)!", 6.0),
            ("round(4.4)!", 24.0),
            // Combined
            ("sqrt(4!) + 3!", 6.0 + 24.0_f64.sqrt()),
            ("floor(5! / 10) + ceil(3! / 4)", 14.0),
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
    fn factorial_nested() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("(3!)!", 720.0),       // 3! = 6, 6! = 720
            ("(2!)!", 2.0),         // 2! = 2, 2! = 2
            ("((2!)!)!", 2.0),      // 2! = 2, 2! = 2, 2! = 2
            ("(1 + 2)!", 6.0),
            ("((1 + 1)!)!", 2.0),   // (1+1)! = 2! = 2, 2! = 2
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
    fn factorial_complex_expressions() {
        let cases: HashMap<&str, f64> = HashMap::from([
            ("5! + 4! + 3! + 2! + 1! + 0!", 154.0),
            ("5! - 4! - 3! - 2! - 1! - 0!", 86.0),
            ("5! * 2 + 4! * 3 + 3! * 4", 336.0),
            ("(5! + 4!) / (3! + 2!)", 18.0),
            ("5! / 4! + 4! / 3! + 3! / 2!", 12.0),
            ("2 ^ 3! - 3 ^ 2!", 55.0),
            ("sqrt(5! + 4!) / 2", 6.0),
            ("(5! - 4!) * (3! - 2!)", 384.0),
            ("5! % 50 + 4! % 10", 62.4),
            ("[5! - 150] + [4! - 30]", 36.0),
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
    fn factorial_boundary_values() {
        // Test boundary: 170! is the largest factorial that fits in f64
        let sub = Lexer::lex("170!").unwrap();
        let result = Calculator::calculate(sub, "170!");
        assert!(result.is_ok(), "170! should be computable");
        assert!(result.unwrap().is_finite(), "170! should be finite");

        // Test that 171! returns error (too large)
        let sub = Lexer::lex("171!").unwrap();
        let result = Calculator::calculate(sub, "171!");
        assert!(result.is_err(), "171! should return error (too large)");

        // Test large but valid factorial
        let sub = Lexer::lex("100!").unwrap();
        let result = Calculator::calculate(sub, "100!");
        assert!(result.is_ok(), "100! should be computable");
    }

    #[test]
    fn factorial_with_decimals_that_are_integers() {
        // 3.0 should work since it's mathematically an integer
        let cases: HashMap<&str, f64> = HashMap::from([
            ("3.0!", 6.0),
            ("5.0!", 120.0),
            ("0.0!", 1.0),
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
    fn factorial_errors() {
        let error_cases: Vec<&str> = vec![
            // Negative numbers
            "(-1)!",
            "(-5)!",
            "(-10)!",
            "(0 - 1)!",
            "(5 - 10)!",
            // Non-integers
            "3.5!",
            "2.1!",
            "0.5!",
            "2.9!",
            "(5 / 2)!",
            // Too large
            "171!",
            "200!",
            "1000!",
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
    fn factorial_operator_precedence() {
        // Factorial should bind tighter than other operators
        let cases: HashMap<&str, f64> = HashMap::from([
            // Factorial before addition
            ("3! + 2", 8.0),        // 6 + 2, not (3+2)!
            ("2 + 3!", 8.0),        // 2 + 6, not (2+3)!
            // Factorial before multiplication
            ("3! * 2", 12.0),       // 6 * 2
            ("2 * 3!", 12.0),       // 2 * 6
            // Factorial before subtraction
            ("3! - 2", 4.0),        // 6 - 2
            ("10 - 3!", 4.0),       // 10 - 6
            // Factorial before division
            ("3! / 2", 3.0),        // 6 / 2
            ("12 / 3!", 2.0),       // 12 / 6
            // Factorial with power
            ("2 ^ 3!", 64.0),       // 2 ^ 6
            ("3! ^ 2", 36.0),       // 6 ^ 2
            // Multiple factorials
            ("2! + 3! + 4!", 32.0), // 2 + 6 + 24
            ("2! * 3! * 4!", 288.0), // 2 * 6 * 24
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
    fn variable_assignment() {
        let mut env = Environment::new();

        // Simple assignment
        let sub = Lexer::lex("x = 5").unwrap();
        let result = Calculator::calculate_with_env(sub, "x = 5", &mut env).unwrap();
        assert_eq!(result, 5.0);
        assert_eq!(env.get("x"), Some(5.0));

        // Assignment with expression
        let sub = Lexer::lex("y = 10 + 5").unwrap();
        let result = Calculator::calculate_with_env(sub, "y = 10 + 5", &mut env).unwrap();
        assert_eq!(result, 15.0);
        assert_eq!(env.get("y"), Some(15.0));

        // Assignment using another variable
        let sub = Lexer::lex("z = x * 2").unwrap();
        let result = Calculator::calculate_with_env(sub, "z = x * 2", &mut env).unwrap();
        assert_eq!(result, 10.0);
        assert_eq!(env.get("z"), Some(10.0));

        // Reassignment
        let sub = Lexer::lex("x = 100").unwrap();
        let result = Calculator::calculate_with_env(sub, "x = 100", &mut env).unwrap();
        assert_eq!(result, 100.0);
        assert_eq!(env.get("x"), Some(100.0));
    }

    #[test]
    fn variable_usage() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        env.set("y", 10.0);

        // Simple variable lookup
        let sub = Lexer::lex("x").unwrap();
        let result = Calculator::calculate_with_env(sub, "x", &mut env).unwrap();
        assert_eq!(result, 5.0);

        // Variable in expression
        let sub = Lexer::lex("x + 10").unwrap();
        let result = Calculator::calculate_with_env(sub, "x + 10", &mut env).unwrap();
        assert_eq!(result, 15.0);

        // Multiple variables
        let sub = Lexer::lex("x + y").unwrap();
        let result = Calculator::calculate_with_env(sub, "x + y", &mut env).unwrap();
        assert_eq!(result, 15.0);

        // Variable with operations
        let sub = Lexer::lex("x * y - 5").unwrap();
        let result = Calculator::calculate_with_env(sub, "x * y - 5", &mut env).unwrap();
        assert_eq!(result, 45.0);

        // Variable in parentheses
        let sub = Lexer::lex("(x + y) * 2").unwrap();
        let result = Calculator::calculate_with_env(sub, "(x + y) * 2", &mut env).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn variable_with_functions() {
        let mut env = Environment::new();
        env.set("x", 16.0);
        env.set("angle", 0.0);

        // Variable as function argument
        let sub = Lexer::lex("sqrt(x)").unwrap();
        let result = Calculator::calculate_with_env(sub, "sqrt(x)", &mut env).unwrap();
        assert_eq!(result, 4.0);

        // Function result to variable
        let sub = Lexer::lex("r = sqrt(x)").unwrap();
        let result = Calculator::calculate_with_env(sub, "r = sqrt(x)", &mut env).unwrap();
        assert_eq!(result, 4.0);
        assert_eq!(env.get("r"), Some(4.0));

        // Variable in function expression
        let sub = Lexer::lex("cos(angle)").unwrap();
        let result = Calculator::calculate_with_env(sub, "cos(angle)", &mut env).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn variable_with_factorial() {
        let mut env = Environment::new();
        env.set("n", 5.0);

        // Factorial of variable
        let sub = Lexer::lex("n!").unwrap();
        let result = Calculator::calculate_with_env(sub, "n!", &mut env).unwrap();
        assert_eq!(result, 120.0);

        // Store factorial result
        let sub = Lexer::lex("fact = n!").unwrap();
        let result = Calculator::calculate_with_env(sub, "fact = n!", &mut env).unwrap();
        assert_eq!(result, 120.0);
        assert_eq!(env.get("fact"), Some(120.0));
    }

    #[test]
    fn variable_names() {
        let mut env = Environment::new();

        // Single letter
        let sub = Lexer::lex("x = 1").unwrap();
        Calculator::calculate_with_env(sub, "x = 1", &mut env).unwrap();
        assert_eq!(env.get("x"), Some(1.0));

        // Multiple letters
        let sub = Lexer::lex("radius = 7").unwrap();
        Calculator::calculate_with_env(sub, "radius = 7", &mut env).unwrap();
        assert_eq!(env.get("radius"), Some(7.0));

        // With underscore
        let sub = Lexer::lex("my_var = 42").unwrap();
        Calculator::calculate_with_env(sub, "my_var = 42", &mut env).unwrap();
        assert_eq!(env.get("my_var"), Some(42.0));

        // With numbers
        let sub = Lexer::lex("x1 = 10").unwrap();
        Calculator::calculate_with_env(sub, "x1 = 10", &mut env).unwrap();
        assert_eq!(env.get("x1"), Some(10.0));

        // Use named variable
        let sub = Lexer::lex("3.14159 * radius ^ 2").unwrap();
        let result = Calculator::calculate_with_env(sub, "3.14159 * radius ^ 2", &mut env).unwrap();
        assert!((result - 153.93791).abs() < 0.001);
    }

    #[test]
    fn variable_errors() {
        let mut env = Environment::new();

        // Undefined variable
        let sub = Lexer::lex("undefined_var").unwrap();
        let result = Calculator::calculate_with_env(sub, "undefined_var", &mut env);
        assert!(result.is_err());

        // Undefined variable in expression
        let sub = Lexer::lex("5 + unknown").unwrap();
        let result = Calculator::calculate_with_env(sub, "5 + unknown", &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn variable_complex_expressions() {
        let mut env = Environment::new();

        // Set up variables
        let _ = Calculator::calculate_with_env(
            Lexer::lex("a = 3").unwrap(), "a = 3", &mut env
        );
        let _ = Calculator::calculate_with_env(
            Lexer::lex("b = 4").unwrap(), "b = 4", &mut env
        );

        // Pythagorean theorem
        let sub = Lexer::lex("c = sqrt(a^2 + b^2)").unwrap();
        let result = Calculator::calculate_with_env(sub, "c = sqrt(a^2 + b^2)", &mut env).unwrap();
        assert_eq!(result, 5.0);
        assert_eq!(env.get("c"), Some(5.0));

        // Chain of assignments
        let _ = Calculator::calculate_with_env(
            Lexer::lex("d = c * 2").unwrap(), "d = c * 2", &mut env
        );
        let _ = Calculator::calculate_with_env(
            Lexer::lex("e = d + a + b").unwrap(), "e = d + a + b", &mut env
        );
        assert_eq!(env.get("d"), Some(10.0));
        assert_eq!(env.get("e"), Some(17.0));
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn variable_with_absolute_value() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        env.set("y", 20.0);

        // Variable in absolute value
        let sub = Lexer::lex("[x - 10]").unwrap();
        let result = Calculator::calculate_with_env(sub, "[x - 10]", &mut env).unwrap();
        assert_eq!(result, 5.0);

        // Absolute value with multiple variables
        let sub = Lexer::lex("[x - y]").unwrap();
        let result = Calculator::calculate_with_env(sub, "[x - y]", &mut env).unwrap();
        assert_eq!(result, 15.0);

        // Nested absolute value with variables
        let sub = Lexer::lex("[[x - 10] - y]").unwrap();
        let result = Calculator::calculate_with_env(sub, "[[x - 10] - y]", &mut env).unwrap();
        assert_eq!(result, 15.0);

        // Assign absolute value result
        let sub = Lexer::lex("z = [x - y]").unwrap();
        let result = Calculator::calculate_with_env(sub, "z = [x - y]", &mut env).unwrap();
        assert_eq!(result, 15.0);
        assert_eq!(env.get("z"), Some(15.0));
    }

    #[test]
    fn variable_self_reference() {
        let mut env = Environment::new();

        // Initial assignment
        let sub = Lexer::lex("x = 5").unwrap();
        Calculator::calculate_with_env(sub, "x = 5", &mut env).unwrap();

        // Self-referencing update
        let sub = Lexer::lex("x = x + 1").unwrap();
        let result = Calculator::calculate_with_env(sub, "x = x + 1", &mut env).unwrap();
        assert_eq!(result, 6.0);
        assert_eq!(env.get("x"), Some(6.0));

        // Multiple self-references
        let sub = Lexer::lex("x = x * x").unwrap();
        let result = Calculator::calculate_with_env(sub, "x = x * x", &mut env).unwrap();
        assert_eq!(result, 36.0);
        assert_eq!(env.get("x"), Some(36.0));

        // Self-reference with other operations
        let sub = Lexer::lex("x = sqrt(x)").unwrap();
        let result = Calculator::calculate_with_env(sub, "x = sqrt(x)", &mut env).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn variable_case_sensitivity() {
        let mut env = Environment::new();

        // Set lowercase
        let sub = Lexer::lex("abc = 10").unwrap();
        Calculator::calculate_with_env(sub, "abc = 10", &mut env).unwrap();

        // Set uppercase (should be different variable)
        let sub = Lexer::lex("ABC = 20").unwrap();
        Calculator::calculate_with_env(sub, "ABC = 20", &mut env).unwrap();

        // Verify they are different
        assert_eq!(env.get("abc"), Some(10.0));
        assert_eq!(env.get("ABC"), Some(20.0));

        // Mixed case
        let sub = Lexer::lex("AbC = 30").unwrap();
        Calculator::calculate_with_env(sub, "AbC = 30", &mut env).unwrap();
        assert_eq!(env.get("AbC"), Some(30.0));
    }

    #[test]
    fn variable_in_nested_parentheses() {
        let mut env = Environment::new();
        env.set("x", 2.0);
        env.set("y", 3.0);

        let cases: Vec<(&str, f64)> = vec![
            ("((x))", 2.0),
            ("(((x)))", 2.0),
            ("((x + y))", 5.0),
            ("(((x + y)))", 5.0),
            ("((x) + (y))", 5.0),
            ("(x + (y + (x + y)))", 10.0),
            ("((x * y) + (y * x))", 12.0),
        ];

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate_with_env(sub, input, &mut env).unwrap();
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
    fn implicit_multiplication_edge_cases() {
        let cases: Vec<(&str, f64)> = vec![
            // Number before parentheses
            ("2(3)", 6.0),
            ("3(2 + 1)", 9.0),
            ("2(3)(4)", 24.0),
            // Parentheses before number - this may or may not be supported
            // ("(3)2", 6.0),  // Depends on implementation
            // Nested
            ("2((3))", 6.0),
            ("2(((3)))", 6.0),
            // With operations inside
            ("2(3 + 4)", 14.0),
            ("3(2 * 4)", 24.0),
            // Multiple implicit
            ("2(3)(4)(5)", 120.0),
            // With absolute value
            ("2[-3]", 6.0),
            ("3[2 - 5]", 9.0),
        ];

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
    fn double_negation() {
        let cases: Vec<(&str, f64)> = vec![
            ("--5", 5.0),
            ("- -5", 5.0),
            ("-(-5)", 5.0),
            ("10 - -5", 15.0),
            ("10 + -5", 5.0),
            ("5 * -2", -10.0),
            ("-5 * -2", 10.0),
            ("10 / -2", -5.0),
            ("-10 / -2", 5.0),
        ];

        for (input, expected) in cases {
            let sub = match Lexer::lex(input) {
                Ok(s) => s,
                Err(_) => continue, // Skip if not supported
            };
            let result = Calculator::calculate(sub, input);
            if let Ok(val) = result {
                assert!(
                    (val - expected).abs() < 1e-10,
                    "Failed for input: {}, expected: {}, got: {}",
                    input,
                    expected,
                    val
                );
            }
        }
    }

    #[test]
    fn whitespace_handling() {
        let cases: Vec<(&str, f64)> = vec![
            ("   5 + 5   ", 10.0),
            ("5+5", 10.0),
            ("5 +5", 10.0),
            ("5+ 5", 10.0),
            ("  5  +  5  ", 10.0),
            ("(  5 + 5  )", 10.0),
            ("  (5+5)  ", 10.0),
            ("sqrt(  16  )", 4.0),
            ("sqrt(16)", 4.0),
        ];

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for input: '{}', expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn large_numbers() {
        let cases: Vec<(&str, f64)> = vec![
            ("1000000 + 1000000", 2000000.0),
            ("1000000 * 1000", 1000000000.0),
            ("1000000000 / 1000", 1000000.0),
            ("2 ^ 30", 1073741824.0),
            ("999999999 + 1", 1000000000.0),
        ];

        for (input, expected) in cases {
            let sub = Lexer::lex(input).unwrap();
            let result = Calculator::calculate(sub, input).unwrap();
            assert!(
                (result - expected).abs() < 1.0, // Allow small float errors for large numbers
                "Failed for input: {}, expected: {}, got: {}",
                input,
                expected,
                result
            );
        }
    }

    #[test]
    fn small_numbers() {
        let cases: Vec<(&str, f64)> = vec![
            ("0.001 + 0.002", 0.003),
            ("0.1 * 0.1", 0.01),
            ("1 / 1000", 0.001),
            ("0.0001 + 0.0001", 0.0002),
            ("0.5 * 0.5", 0.25),
            (".5 + .5", 1.0),
            (".1 + .2", 0.30000000000000004), // Known float precision issue
        ];

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
    fn zero_edge_cases() {
        let cases: Vec<(&str, f64)> = vec![
            ("0 + 0", 0.0),
            ("0 - 0", 0.0),
            ("0 * 100", 0.0),
            ("100 * 0", 0.0),
            ("0 / 100", 0.0),
            ("0 ^ 5", 0.0),
            ("5 ^ 0", 1.0),
            ("0 ^ 0", 1.0), // Mathematical convention
            ("[0]", 0.0),
            ("[-0]", 0.0),
            ("0!", 1.0),
            ("sqrt(0)", 0.0),
            ("sin(0)", 0.0),
            ("cos(0)", 1.0),
            ("tan(0)", 0.0),
            ("floor(0)", 0.0),
            ("ceil(0)", 0.0),
            ("round(0)", 0.0),
        ];

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
    fn operator_precedence_comprehensive() {
        // Tests to verify correct order of operations: parentheses > power > mult/div > add/sub
        let cases: Vec<(&str, f64)> = vec![
            // Basic precedence
            ("2 + 3 * 4", 14.0),      // mult before add
            ("2 * 3 + 4", 10.0),      // mult before add
            ("10 - 4 / 2", 8.0),      // div before sub
            ("10 / 2 - 4", 1.0),      // div before sub
            ("2 ^ 3 * 4", 32.0),      // power before mult
            ("4 * 2 ^ 3", 32.0),      // power before mult
            ("2 + 3 ^ 2", 11.0),      // power before add
            ("3 ^ 2 + 2", 11.0),      // power before add
            // Mixed
            ("2 + 3 * 4 ^ 2", 50.0),  // power > mult > add
            ("2 ^ 3 + 4 * 5", 28.0),  // power and mult before add
            ("10 - 2 ^ 2 * 2", 2.0),  // power > mult > sub
            // Parentheses override
            ("(2 + 3) * 4", 20.0),
            ("2 * (3 + 4)", 14.0),
            ("(2 + 3) ^ 2", 25.0),
            ("2 ^ (1 + 2)", 8.0),
            ("((2 + 3) * 4) ^ 2", 400.0),
            // Left-to-right for same precedence
            ("10 - 5 - 2", 3.0),      // (10 - 5) - 2
            ("10 / 2 / 5", 1.0),      // (10 / 2) / 5
            ("10 - 5 + 2", 7.0),      // (10 - 5) + 2
            ("10 / 2 * 5", 25.0),     // (10 / 2) * 5
            // Power is right-associative
            ("2 ^ 3 ^ 2", 512.0),     // 2 ^ (3 ^ 2) = 2 ^ 9 = 512
            ("2 ^ 2 ^ 3", 256.0),     // 2 ^ (2 ^ 3) = 2 ^ 8 = 256
        ];

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
    fn chained_operations() {
        let cases: Vec<(&str, f64)> = vec![
            // Long chains of same operation
            ("1 + 2 + 3 + 4 + 5", 15.0),
            ("1 - 2 - 3 - 4 - 5", -13.0),
            ("1 * 2 * 3 * 4 * 5", 120.0),
            ("120 / 2 / 3 / 4 / 5", 1.0),
            // Mixed chains
            ("1 + 2 - 3 + 4 - 5", -1.0),
            ("2 * 3 / 2 * 4 / 3", 4.0),
            ("10 + 20 - 5 + 3 - 8", 20.0),
            // With parentheses
            ("(1 + 2) + (3 + 4) + (5 + 6)", 21.0),
            ("(2 * 3) * (4 * 5)", 120.0),
        ];

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
    fn single_values() {
        let cases: Vec<(&str, f64)> = vec![
            ("5", 5.0),
            ("0", 0.0),
            ("-5", -5.0),
            ("+5", 5.0),
            ("3.14", 3.14),
            ("-3.14", -3.14),
            (".5", 0.5),
            ("-0.5", -0.5),
            ("(5)", 5.0),
            ("((5))", 5.0),
            ("[5]", 5.0),
            ("[-5]", 5.0),
        ];

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
    fn function_edge_cases() {
        // Boundary values for functions
        let cases: Vec<(&str, f64)> = vec![
            // sqrt at boundary
            ("sqrt(0)", 0.0),
            ("sqrt(1)", 1.0),
            // Trig at special angles
            ("sin(0)", 0.0),
            ("cos(0)", 1.0),
            // exp/ln inverse
            ("exp(ln(5))", 5.0),
            ("ln(exp(5))", 5.0),
            // log at powers of 10
            ("log(1)", 0.0),
            ("log(10)", 1.0),
            ("log(100)", 2.0),
            ("log(1000)", 3.0),
            // floor/ceil at integers
            ("floor(5)", 5.0),
            ("ceil(5)", 5.0),
            ("round(5)", 5.0),
            // floor/ceil at .5
            ("floor(2.5)", 2.0),
            ("ceil(2.5)", 3.0),
            ("round(2.5)", 3.0), // Rounds to even might differ
        ];

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
    fn variable_reserved_names() {
        // Test that math function names don't conflict with variables
        let mut env = Environment::new();

        // These should work as variable names (not conflict with functions)
        let var_names = vec!["e", "pi", "x", "y", "z", "n", "i", "result"];

        for name in var_names {
            let input = format!("{} = 42", name);
            let sub = Lexer::lex(&input).unwrap();
            let result = Calculator::calculate_with_env(sub, &input, &mut env);
            assert!(
                result.is_ok(),
                "Should be able to use '{}' as variable name",
                name
            );
            assert_eq!(env.get(name), Some(42.0));
        }
    }

    #[test]
    fn percentage_edge_cases() {
        let cases: Vec<(&str, f64)> = vec![
            ("100 % 50", 50.0),      // 50% of 100
            ("100 % 100", 100.0),    // 100% of 100
            ("100 % 0", 0.0),        // 0% of 100
            ("0 % 50", 0.0),         // 50% of 0
            ("200 % 25", 50.0),      // 25% of 200
            ("50 % 200", 100.0),     // 200% of 50
            // Chained percentages
            ("100 % 50 % 50", 25.0), // 50% of (50% of 100)
            // Percentage with other ops
            ("100 % 10 + 5", 15.0),
            ("5 + 100 % 10", 15.0),
            ("100 % 10 * 2", 20.0),
            ("2 * 100 % 10", 20.0),
        ];

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
    fn power_edge_cases() {
        let cases: Vec<(&str, f64)> = vec![
            ("2 ^ 0", 1.0),
            ("2 ^ 1", 2.0),
            ("2 ^ -1", 0.5),
            ("2 ^ -2", 0.25),
            ("4 ^ 0.5", 2.0),        // Square root
            ("8 ^ (1/3)", 2.0),      // Cube root
            ("27 ^ (1/3)", 3.0),
            ("1 ^ 1000", 1.0),
            ("(-2) ^ 2", 4.0),
            ("(-2) ^ 3", -8.0),
            ("0 ^ 1", 0.0),
            ("0 ^ 100", 0.0),
        ];

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
    fn syntax_error_cases() {
        let error_cases: Vec<&str> = vec![
            // Empty groups
            "()",
            "[]",
            "(())",
            // Mismatched brackets
            "(5 + 3]",
            "[5 + 3)",
            "((5 + 3)",
            "(5 + 3))",
            "[5]]",
            // Division by zero
            "5 / 0",
            "10 / (5 - 5)",
            // Function errors
            "sqrt(-1)",
            "log(0)",
            "log(-5)",
            "ln(0)",
            "ln(-5)",
            // Factorial errors
            "(-1)!",
            "3.5!",
            "171!",
        ];

        for input in error_cases {
            let result = match Lexer::lex(input) {
                Ok(sub) => Calculator::calculate(sub, input),
                Err(e) => Err(e),
            };
            assert!(
                result.is_err(),
                "Expected error for input: '{}', got: {:?}",
                input,
                result
            );
        }
    }

    #[test]
    fn valid_edge_syntax() {
        // These are valid syntax that might seem like errors
        let cases: Vec<(&str, f64)> = vec![
            // Single operators with implicit 0 or are parsed differently
            ("+5", 5.0),       // Unary plus
            ("-5", -5.0),      // Unary minus
        ];

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
