//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::token::{Token, TokenType};

pub trait Monitor {
    // Converts the [&self] object to the
    // right String representing value.
    fn to_string(&self, n: usize) -> String;
}

// A monitor debugger implementation for [Token].
impl Monitor for Token {
    fn to_string(&self, n: usize) -> String {
        let mut lit: String;
        let mut space: String = String::new();

        let nest: usize = n;
        if self.typ != TokenType::SUBEXP {
            lit = format!("({})", self.literal.to_string());
        } else {
            lit = String::new();
            for t in self.sub_tokens.iter().map(|t| t.to_string(nest + 1)) {
                lit.push_str(format!("\n{}", t).as_str())
            }
        }

        for _ in 0..nest {
            space.push_str("  ");
        }

        format!("{}{}{}", space.as_str(), self.typ.to_string(0), lit)
    }
}

// A monitor debugger implementation for [TokenType].
impl Monitor for TokenType {
    fn to_string(&self, _n: usize) -> String {
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
            TokenType::PERCENTAGE => "PERCENTAGE",
            TokenType::ROOT => "ROOT",
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
                Token::from(String::from("-25")).to_string(0),
                String::from("NUMBER(-25)"),
            ),
            (
                Token::from(String::from("/")).to_string(0),
                String::from("DIVIDE(/)"),
            ),
        ]);

        for (t, expected) in test_data {
            assert_eq!(t, expected);
        }
    }

    #[test]
    fn token_type_to_string() {
        let test_data: HashMap<String, &str> = HashMap::from([
            (TokenType::NUMBER.to_string(0), "NUMBER"),
            (TokenType::ILLEGAL.to_string(0), "ILLEGAL"),
            (TokenType::SUBEXP.to_string(0), "SUB-EXPRESSION"),
            (TokenType::LPAREN.to_string(0), "LEFT-PARENTHESES"),
            (TokenType::RPAREN.to_string(0), "RIGHT-PARENTHESES"),
            (TokenType::POINTER.to_string(0), "POINTER"),
            (TokenType::PLUS.to_string(0), "PLUS"),
            (TokenType::MINUS.to_string(0), "MINUS"),
            (TokenType::PRODUCT.to_string(0), "PRODUCT"),
            (TokenType::DIVIDE.to_string(0), "DIVIDE"),
            (TokenType::PERCENTAGE.to_string(0), "PERCENTAGE"),
            (TokenType::ROOT.to_string(0), "ROOT"),
        ]);

        for (tt, expected) in test_data {
            assert_eq!(tt, expected);
        }
    }
}
