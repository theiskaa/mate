//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::token::Token;

// Main structure model for errors of lexer.
#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    // The error template used to generate cool error messages by input, invalid token, title of
    // error and explanation of error.
    // Generated error would be like:
    //
    // ```
    // <your {err} title here>
    //
    //      "<your input here>"
    //         |
    //         | > Your detailed error
    //         | > explanation here.
    // ```
    fn indexed_error(input: String, point: i32, err: String, expl: Vec<&str>) -> Self {
        let mut message = err;

        let tab = "     ";
        let mut space = String::new();
        for _ in 0..point - 1 {
            space.push(' ');
        }

        message.push_str(&format!("{tab}\"{}\" \n", input.trim_end()));
        for exp in expl.iter() {
            message.push_str(&format!(" {tab}{space}{exp}\n"));
        }

        Self { msg: message }
    }

    // A custom early made error for empty input cases.
    pub fn empty_input() -> Self {
        Self {
            msg: String::from("error: cannot parse an empty input"),
        }
    }

    // A custom early made error for empty tokens cases.
    pub fn empty_tokens() -> Self {
        Self {
            msg: String::from("error: cannot calculate result from an empty token list"),
        }
    }

    // A custom early made error for invalid tokens cases.
    // Looks like:
    //
    // ```
    // [!] error: missing some tokens to calculate result
    //
    //      "<your input here [X]>"
    //                         |
    //                         | > Cannot convert the character
    //                         | > that represented as number,
    //                         | > to the actual number representation.
    // ```
    //
    pub fn missing_some_tokens(input: String, point: i32) -> Self {
        let message = "error: missing some tokens to calculate result\n\n".to_string();

        let mut inpt: String = input.trim_end().to_string();
        let pointer = " {X} ";

        for i in 1..pointer.len() {
            let p: i32 = point + (i as i32);
            let pch: char = pointer.chars().nth(i - 1).unwrap();

            let backid: usize = if p < 1 { 0 } else { (p - 1) as usize };

            let back_ch = inpt.chars().nth(backid).unwrap_or('0');
            let next_ch = inpt.chars().nth((p + 1) as usize).unwrap_or('0');

            if (back_ch == ' ' || next_ch == ' ') && pch == ' ' {
                continue;
            }

            inpt.insert(p as usize, pch);
        }

        // A split list of error explanation.
        let explanation: Vec<&str> = Vec::from([
            "|",
            "| > Expected a token character.",
            "| > hint: `42`, `+`, `-`, `/`, `*`, `%`, `^`.",
        ]);

        Error::indexed_error(inpt, point + 4, message, explanation)
    }

    // A custom [indexed_error] implementation for rust string -> to -> number parsing error.
    // Looks like:
    //
    // ```
    // error: cannot parse token literal: `<token-literal>` to a number
    //
    //      "<your input here>"
    //         |
    //         | > Cannot convert the character (that represented
    //         | > as number) to the actual number representation.
    // ```
    pub fn cannot_parse_to_number(input: String, token: Token) -> Self {
        let message = format!(
            "error: cannot parse token literal: `{}` to a number\n\n",
            token.clone().literal.clone()
        );

        // A split list of error explanation.
        let explanation: Vec<&str> = Vec::from([
            "|",
            "| > Cannot convert the character (that represented",
            "| > as number) to the actual number representation.",
        ]);

        Error::indexed_error(input, token.index.1 + 1, message, explanation)
    }

    // A custom early made error for invalid order case of token characters.
    pub fn invalid_order() -> Self {
        let space = "      ";
        let mut msg = String::from("error: invalid order of token characters\n");

        msg.push_str(&format!("{space}A valid token/character order is:"));
        msg.push_str(&format!("{space}[Numerable], [Operation], [Numerable]"));

        Self { msg }
    }

    pub fn illegal_token(input: String, token: Token) -> Self {
        let message = format!(
            "error: found an illegal character: `{}` \n\n",
            token.literal
        );

        // A split list of error explanation.
        let explanation: Vec<&str> = Vec::from([
            "|",
            "| > We do not know how to parse this character",
            "| > If you think this is a bug or a practical feature",
            "| > that we do not have yet, please open an issue:",
            "| >   -> https://github.com/theiskaa/mate/issues/new",
        ]);

        Error::indexed_error(input, token.index.1 + 1, message, explanation)
    }

    // A custom error for division by zero cases.
    pub fn division_by_zero(input: String, point: i32) -> Self {
        let message = String::from("error: division by zero\n\n");

        let explanation: Vec<&str> = Vec::from([
            "|",
            "| > Cannot divide by zero.",
            "| > hint: ensure the divisor is not zero.",
        ]);

        Error::indexed_error(input, point, message, explanation)
    }

    pub fn mismatched_parentheses(input: String, point: i32) -> Self {
        let message = String::from("error: mismatched parentheses or brackets\n\n");

        let explanation: Vec<&str> = Vec::from([
            "|",
            "| > Found a closing bracket without a matching opening bracket,",
            "| > or brackets are mismatched (e.g., '(' closed with ']').",
            "| > hint: ensure all brackets are properly paired.",
        ]);

        Error::indexed_error(input, point, message, explanation)
    }

    pub fn to_string(&self) -> String {
        self.msg.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let result: Error = Error::new(String::from("test message"));
        assert_eq!(result.msg, String::from("test message"));
    }

    #[test]
    fn empty_input() {
        let result: Error = Error::empty_input();
        assert_eq!(
            result.msg,
            String::from("error: cannot parse an empty input")
        );
    }

    #[test]
    fn empty_tokens() {
        let result: Error = Error::empty_tokens();
        assert_eq!(
            result.msg,
            String::from("error: cannot calculate result from an empty token list")
        );
    }

    #[test]
    fn to_string() {
        let error: Error = Error::new(String::from("A new message"));
        assert_eq!(error.to_string(), error.msg)
    }
}
