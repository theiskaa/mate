//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::token::{Token, TokenType};

pub trait Monitor {
    // Converts the [&self] object to the
    // right String representing value.
    fn to_string(&self) -> String;
}

// A monitor debugger implementation for [Token].
impl Monitor for Token {
    fn to_string(&self) -> String {
        let mut lit: String;
        if self.typ != TokenType::SUBEXP {
            lit = self.literal.to_string();
        } else {
            lit = String::new();
            for t in self.sub_tokens.iter().map(|t| t.to_string()) {
                // TODO: improve sub expression monitoring.
                lit.push_str(format!("{} ||| ", t).as_str())
            }
        }

        String::from(self.typ.to_string() + " | " + lit.as_str())
    }
}

// A monitor debugger implementation for [TokenType].
impl Monitor for TokenType {
    fn to_string(&self) -> String {
        let data = match self {
            TokenType::NUMBER => "NUMBER",
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::SUBEXP => "SUB-EXPRESSION",
            TokenType::LPAREN => "LEFT-PARENTHESES",
            TokenType::RPAREN => "RIGHT-PARENTHESES",
            TokenType::POINTER => "POINTER",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::PRODUCT => "PRODUCT",
            TokenType::DIVIDE => "DIVIDE",
        };

        String::from(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn token_to_string() {
        let test_data: HashMap<String, String> = HashMap::from([
            (
                Token::from(String::from("-25")).to_string(),
                String::from("NUMBER | -25"),
            ),
            (
                Token::from(String::from("/")).to_string(),
                String::from("DIVIDE | /"),
            ),
        ]);

        for (t, expected) in test_data {
            assert_eq!(t, expected);
        }
    }

    #[test]
    fn token_type_to_string() {
        let test_data: HashMap<String, &str> = HashMap::from([
            (TokenType::NUMBER.to_string(), "NUMBER"),
            (TokenType::ILLEGAL.to_string(), "ILLEGAL"),
            (TokenType::SUBEXP.to_string(), "SUB-EXPRESSION"),
            (TokenType::LPAREN.to_string(), "LEFT-PARENTHESES"),
            (TokenType::RPAREN.to_string(), "RIGHT-PARENTHESES"),
            (TokenType::POINTER.to_string(), "POINTER"),
            (TokenType::PLUS.to_string(), "PLUS"),
            (TokenType::MINUS.to_string(), "MINUS"),
            (TokenType::PRODUCT.to_string(), "PRODUCT"),
            (TokenType::DIVIDE.to_string(), "DIVIDE"),
        ]);

        for (tt, expected) in test_data {
            assert_eq!(tt, expected);
        }
    }
}
