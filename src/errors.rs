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
        let mut message = err.clone();

        let tab: String = String::from("     ");
        let mut space: String = String::from("");
        for _ in 0..point - 1 {
            space.push_str(" ");
        }

        message.push_str(format!("{}\"{}\" \n", tab.clone(), input.trim_end()).as_str());
        for exp in expl.iter() {
            message.push_str(format!(" {}{}{}\n", tab.clone(), space.clone(), exp).as_str());
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
        let message = format!("error: missing some tokens to calculate result\n\n");

        let mut inpt: String = input.clone().trim_end().to_string();
        let pointer: String = String::from(" {X} ");

        for i in 1..pointer.len() {
            let p: usize = (point as usize) + i;
            let pch: char = pointer.chars().nth(i - 1).unwrap();

            let back_ch: char = match inpt.chars().nth(p - 1) {
                Some(v) => v,
                None => '0',
            };
            let next_ch: char = match inpt.chars().nth(p + 1) {
                Some(v) => v,
                None => '0',
            };

            if back_ch == ' ' && pch == ' ' || next_ch == ' ' && pch == ' ' {
                continue;
            }

            inpt.insert(p, pch);
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

        msg.push_str(format!("{}A valid token/character order is:", space).as_str());
        msg.push_str(format!("{}[Numerable], [Operation], [Numerable]", space).as_str());

        Self { msg }
    }

    // A custom [indexed_error] implementation for illegal token error.
    // Looks like:
    //
    // ```
    // error: found an illegal character `<token-literal>`
    //
    //      "<your input here>"
    //         |
    //         | > We do not know how to parse this character
    //         | > If you think this is a bug or a practical feature
    //         | > that we do not have yet, please open an issue:
    //         | >   -> https://github.com/theiskaa/mate/issues/new
    // ```
    pub fn illeagal_token(input: String, token: Token) -> Self {
        let message = format!(
            "error: found an illegal character: `{}` \n\n",
            token.clone().literal
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
