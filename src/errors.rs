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
    pub fn missing_some_tokens() -> Self {
        // TODO: improve
        Self {
            msg: String::from("error: missing some tokens to calculate result"),
        }
    }

    // A custom early made error for rust string -> to -> number parsing error.
    pub fn cannot_parse_to_number() -> Self {
        // TODO: improve
        Self {
            msg: String::from("error: cannot parse token literal to a number"),
        }
    }

    // A custom early made error for invalid order case of token characters.
    pub fn invalid_order() -> Self {
        // TODO: improve
        Self {
            msg: String::from("error: invalid order of token characters"),
        }
    }

    // A custom early made error for illegal token warning.
    pub fn illeagal_token(input: String, token: Token) -> Self {
        let mut message = format!(
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

        let tab: String = String::from("     ");
        let mut space: String = String::from("");
        for _ in 0..token.index.1 - 1 {
            space.push_str(" ");
        }

        message
            .push_str(format!("{}\"{}\" \n", tab.clone(), input.trim_start().trim_end()).as_str());
        for exp in explanation.iter() {
            message.push_str(format!(" {}{}{}\n", tab.clone(), space.clone(), exp).as_str());
        }

        Self { msg: message }
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
    fn missing_some_tokens() {
        let result: Error = Error::missing_some_tokens();
        assert_eq!(
            result.msg,
            String::from("error: missing some tokens to calculate result")
        );
    }

    #[test]
    fn cannot_parse_to_number() {
        let result: Error = Error::cannot_parse_to_number();
        assert_eq!(
            result.msg,
            String::from("error: cannot parse token literal to a number")
        );
    }

    #[test]
    fn invalid_order() {
        let result: Error = Error::invalid_order();
        assert_eq!(
            result.msg,
            String::from("error: invalid order of token characters")
        );
    }

    #[test]
    fn illeagal_token() {
        let result: Error =
            Error::illeagal_token(String::from("$"), Token::from(String::from("$"), (0, 0)));

        // TODO: write test for [illeagal_token()].
        assert_eq!(result.msg, result.to_string());
    }

    #[test]
    fn to_string() {
        let error: Error = Error::new(String::from("A new message"));
        assert_eq!(error.to_string(), error.msg)
    }
}
