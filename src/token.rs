//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::utils::ChUtils;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    NUMBER,
    SUBEXP,
    LPAREN,
    RPAREN,
    ILLEGAL,
    POINTER,

    // Operations
    PLUS,
    MINUS,
    PRODUCT,
    DIVIDE,
    PERCENTAGE,
    ROOT,
}

// A small-block representing structure of lexer's input.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
    pub sub_tokens: Vec<Token>,
}

impl Token {
    // Define a new Token value by providing all fields.
    pub fn new(typ: TokenType, literal: String, sub_tokens: Vec<Token>) -> Self {
        Self {
            typ,
            literal,
            sub_tokens,
        }
    }

    // Create a new sub token model with just sub tokens.
    pub fn new_sub(sub_tokens: Vec<Token>) -> Self {
        Self {
            typ: TokenType::SUBEXP,
            literal: String::new(),
            sub_tokens,
        }
    }

    // Creates a pointer token, that newer will be used
    // at normal token result.
    pub fn new_pointer(i: usize) -> Self {
        Self {
            typ: TokenType::POINTER,
            literal: format!("{}", i),
            sub_tokens: Vec::new(),
        }
    }

    // Create a new token model from a literal.
    // The type is decided automatically by checking it.
    pub fn from(mut literal: String) -> Self {
        let typ: TokenType;

        if literal.is_number() {
            typ = TokenType::NUMBER;
        } else {
            typ = match literal.trim() {
                "+" => TokenType::PLUS,
                "-" => TokenType::MINUS,
                "*" => TokenType::PRODUCT,
                "•" => TokenType::PRODUCT,
                "/" => TokenType::DIVIDE,
                ":" => TokenType::DIVIDE,
                "(" => TokenType::LPAREN,
                ")" => TokenType::RPAREN,
                "%" => TokenType::PERCENTAGE,
                "^" => TokenType::ROOT,
                _ => TokenType::ILLEGAL,
            }
        }

        // Clear the white-spaces from literal.
        literal.retain(|c| !c.is_whitespace());

        return Self {
            typ,
            literal,
            sub_tokens: Vec::new(),
        };
    }

    // Takes the pointer's index as [usize].
    // If current token is not an pointer token, returned option will be [None].
    pub fn take_pointer_index(&self) -> Option<usize> {
        if self.typ != TokenType::POINTER {
            return None;
        }

        match self.literal.as_str().parse::<usize>() {
            Err(_) => return None,
            Ok(v) => return Some(v),
        };
    }

    // Checks if pointed token's type is illegal or not.
    pub fn is_illegal(&self) -> bool {
        match self.typ {
            TokenType::ILLEGAL => true,
            _ => false,
        }
    }

    // Checks if pointed token's type is left-parentheses or not.
    pub fn is_lparen(&self) -> bool {
        match self.typ {
            TokenType::LPAREN => true,
            _ => false,
        }
    }

    // Checks if pointed token's type is right-parentheses or not.
    pub fn is_rparen(&self) -> bool {
        match self.typ {
            TokenType::RPAREN => true,
            _ => false,
        }
    }

    // Checks if pointed token's type is pointer or not.
    pub fn is_pointer(&self) -> bool {
        match self.typ {
            TokenType::POINTER => true,
            _ => false,
        }
    }

    // Checks if pointed token's type is sub-expression or not.
    pub fn is_sub_exp(&self) -> bool {
        match self.typ {
            TokenType::SUBEXP => true,
            _ => false,
        }
    }

    // Checks if pointed token's type is root or not.
    pub fn is_root(&self) -> bool {
        match self.typ {
            TokenType::ROOT => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new() {
        let test_data: Vec<Token> = vec![
            Token {
                typ: TokenType::PLUS,
                literal: String::from("+"),
                sub_tokens: Vec::new(),
            },
            Token {
                typ: TokenType::MINUS,
                literal: String::from("-"),
                sub_tokens: Vec::new(),
            },
            Token {
                typ: TokenType::DIVIDE,
                literal: String::from("/"),
                sub_tokens: Vec::new(),
            },
            Token {
                typ: TokenType::SUBEXP,
                literal: String::from(""),
                sub_tokens: Vec::from([
                    Token::from(String::from("2")),
                    Token::from(String::from("+")),
                    Token::from(String::from("5")),
                ]),
            },
        ];

        for t in test_data {
            let res = Token::new(t.clone().typ, t.clone().literal, t.clone().sub_tokens);

            assert_eq!(res.typ, t.clone().typ);
            assert_eq!(res.literal, t.clone().literal);
            assert_eq!(res.sub_tokens, t.clone().sub_tokens);
        }
    }

    #[test]
    fn new_sub() {
        let test_data: HashMap<Vec<String>, Token> = HashMap::from([
            (
                vec![String::from("4"), String::from("+"), String::from("2")],
                Token {
                    typ: TokenType::SUBEXP,
                    literal: String::new(),
                    sub_tokens: vec![
                        Token::from(String::from("4")),
                        Token::from(String::from("+")),
                        Token::from(String::from("2")),
                    ],
                },
            ),
            (
                vec![String::from("2"), String::from("+"), String::from("+")],
                Token {
                    typ: TokenType::SUBEXP,
                    literal: String::new(),
                    sub_tokens: vec![
                        Token::from(String::from("2")),
                        Token::from(String::from("+")),
                        Token::from(String::from("+")),
                    ],
                },
            ),
        ]);

        for (t, expected) in test_data {
            let tokens = t.into_iter().map(|tt| Token::from(tt)).collect();
            let res = Token::new_sub(tokens);

            assert_eq!(res.typ, expected.clone().typ);
            assert_eq!(res.literal, expected.clone().literal);
            assert_eq!(res.sub_tokens, expected.clone().sub_tokens);
        }
    }

    #[test]
    fn new_pointer() {
        let test_data: HashMap<usize, Token> = HashMap::from([
            (
                0,
                Token::new(TokenType::POINTER, String::from("0"), Vec::new()),
            ),
            (
                99,
                Token::new(TokenType::POINTER, String::from("99"), Vec::new()),
            ),
        ]);

        for (i, expected) in test_data {
            let token: Token = Token::new_pointer(i);
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn from() {
        let test_data: HashMap<String, Token> = HashMap::from([
            (
                String::from("42"),
                Token::new(TokenType::NUMBER, String::from("42"), Vec::new()),
            ),
            (
                String::from("}"),
                Token::new(TokenType::ILLEGAL, String::from("}"), Vec::new()),
            ),
            (
                String::from("+"),
                Token::new(TokenType::PLUS, String::from("+"), Vec::new()),
            ),
            (
                String::from("-"),
                Token::new(TokenType::MINUS, String::from("-"), Vec::new()),
            ),
            (
                String::from("*"),
                Token::new(TokenType::PRODUCT, String::from("*"), Vec::new()),
            ),
            (
                String::from("•"),
                Token::new(TokenType::PRODUCT, String::from("•"), Vec::new()),
            ),
            (
                String::from("/"),
                Token::new(TokenType::DIVIDE, String::from("/"), Vec::new()),
            ),
            (
                String::from(":"),
                Token::new(TokenType::DIVIDE, String::from(":"), Vec::new()),
            ),
            (
                String::from("%"),
                Token::new(TokenType::PERCENTAGE, String::from("%"), Vec::new()),
            ),
        ]);

        for (literal, expected) in test_data {
            let res = Token::from(literal);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn take_pointer_index() {
        let test_data: HashMap<Option<usize>, Token> = HashMap::from([
            (None, Token::from(String::from("25"))),
            (None, Token::from(String::from("-"))),
            (Some(0), Token::new_pointer(0)),
            (Some(9), Token::new_pointer(9)),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.take_pointer_index());
        }
    }

    #[test]
    fn is_illegal() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (true, Token::from(String::from("}"))),
            (true, Token::from(String::from("["))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_illegal());
        }
    }

    #[test]
    fn is_lparen() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (false, Token::from(String::from(")"))),
            (true, Token::from(String::from("("))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_lparen());
        }
    }

    #[test]
    fn is_rparen() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (false, Token::from(String::from("("))),
            (true, Token::from(String::from(")"))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_rparen());
        }
    }

    #[test]
    fn is_pointer() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (false, Token::from(String::from("("))),
            (true, Token::new_pointer(0)),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_pointer());
        }
    }

    #[test]
    fn is_sub_exp() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (false, Token::from(String::from("("))),
            (true, Token::new_sub(vec![])),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_sub_exp());
        }
    }

    #[test]
    fn is_root() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"))),
            (false, Token::from(String::from("-"))),
            (false, Token::from(String::from("("))),
            (true, Token::from(String::from("^"))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_root());
        }
    }
}
