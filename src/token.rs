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
    // the index range of concrete token.
    // [-1] represents the unknown index.
    // left side is the starting point and right side is ending point.
    pub index: (i32, i32),
}

impl Token {
    // Define a new Token value by providing all fields.
    pub fn new(typ: TokenType, literal: String, sub_tokens: Vec<Token>, index: (i32, i32)) -> Self {
        Self {
            typ,
            literal,
            sub_tokens,
            index,
        }
    }

    // Create a new sub token model with just sub tokens.
    pub fn new_sub(sub_tokens: Vec<Token>) -> Self {
        Self {
            typ: TokenType::SUBEXP,
            literal: String::new(),
            sub_tokens,
            index: Token::unknown_index(),
        }
    }

    // Creates a pointer token, that newer will be used
    // at normal token result.
    pub fn new_pointer(i: usize) -> Self {
        Self {
            typ: TokenType::POINTER,
            literal: format!("{}", i),
            sub_tokens: Vec::new(),
            index: Token::unknown_index(),
        }
    }

    // Create a new token model from a literal.
    // The type is decided automatically by checking it.
    pub fn from(mut literal: String, index: (i32, i32)) -> Self {
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
            index,
        };
    }

    // Returns the default unknown index representation.
    pub fn unknown_index() -> (i32, i32) {
        (-1, -1)
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
                index: (0, 0),
            },
            Token {
                typ: TokenType::MINUS,
                literal: String::from("-"),
                sub_tokens: Vec::new(),
                index: (1, 1),
            },
            Token {
                typ: TokenType::DIVIDE,
                literal: String::from("/"),
                sub_tokens: Vec::new(),
                index: (2, 2),
            },
            Token {
                typ: TokenType::SUBEXP,
                literal: String::from(""),
                sub_tokens: Vec::from([
                    Token::from(String::from("2"), (0, 0)),
                    Token::from(String::from("+"), (1, 1)),
                    Token::from(String::from("5"), (2, 2)),
                ]),
                index: (0, 2),
            },
        ];

        for t in test_data {
            let res = Token::new(
                t.clone().typ,
                t.clone().literal,
                t.clone().sub_tokens,
                t.clone().index,
            );

            assert_eq!(res.typ, t.clone().typ);
            assert_eq!(res.literal, t.clone().literal);
            assert_eq!(res.sub_tokens, t.clone().sub_tokens);
            assert_eq!(res.index, t.clone().index);
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
                        Token::from(String::from("4"), (0, 0)),
                        Token::from(String::from("+"), (0, 0)),
                        Token::from(String::from("2"), (0, 0)),
                    ],
                    index: Token::unknown_index(),
                },
            ),
            (
                vec![String::from("2"), String::from("+"), String::from("+")],
                Token {
                    typ: TokenType::SUBEXP,
                    literal: String::new(),
                    sub_tokens: vec![
                        Token::from(String::from("2"), (0, 0)),
                        Token::from(String::from("+"), (0, 0)),
                        Token::from(String::from("+"), (0, 0)),
                    ],
                    index: Token::unknown_index(),
                },
            ),
        ]);

        for (t, expected) in test_data {
            let tokens = t.into_iter().map(|tt| Token::from(tt, (0, 0))).collect();
            let res = Token::new_sub(tokens);

            assert_eq!(res.typ, expected.clone().typ);
            assert_eq!(res.literal, expected.clone().literal);
            assert_eq!(res.sub_tokens, expected.clone().sub_tokens);
            assert_eq!(res.index, expected.clone().index);
        }
    }

    #[test]
    fn new_pointer() {
        let test_data: HashMap<usize, Token> = HashMap::from([
            (
                0,
                Token::new(TokenType::POINTER, String::from("0"), Vec::new(), (-1, -1)),
            ),
            (
                99,
                Token::new(TokenType::POINTER, String::from("99"), Vec::new(), (-1, -1)),
            ),
        ]);

        for (i, expected) in test_data {
            let token: Token = Token::new_pointer(i);
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn from() {
        let test_data: HashMap<(String, (i32, i32)), Token> = HashMap::from([
            (
                (String::from("42"), (0, 1)),
                Token::new(TokenType::NUMBER, String::from("42"), Vec::new(), (0, 1)),
            ),
            (
                (String::from("}"), (0, 0)),
                Token::new(TokenType::ILLEGAL, String::from("}"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("+"), (0, 0)),
                Token::new(TokenType::PLUS, String::from("+"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("-"), (0, 0)),
                Token::new(TokenType::MINUS, String::from("-"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("*"), (0, 0)),
                Token::new(TokenType::PRODUCT, String::from("*"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("•"), (0, 0)),
                Token::new(TokenType::PRODUCT, String::from("•"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("/"), (0, 0)),
                Token::new(TokenType::DIVIDE, String::from("/"), Vec::new(), (0, 0)),
            ),
            (
                (String::from(":"), (0, 0)),
                Token::new(TokenType::DIVIDE, String::from(":"), Vec::new(), (0, 0)),
            ),
            (
                (String::from("%"), (0, 0)),
                Token::new(TokenType::PERCENTAGE, String::from("%"), Vec::new(), (0, 0)),
            ),
        ]);

        for (v, expected) in test_data {
            let res = Token::from(v.0, v.1);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn unknown_index() {
        assert_eq!(Token::unknown_index(), (-1, -1));
    }

    #[test]
    fn take_pointer_index() {
        let test_data: HashMap<Option<usize>, Token> = HashMap::from([
            (None, Token::from(String::from("25"), (0, 1))),
            (None, Token::from(String::from("-"), (0, 0))),
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
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (true, Token::from(String::from("}"), (0, 0))),
            (true, Token::from(String::from("["), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_illegal());
        }
    }

    #[test]
    fn is_lparen() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from(")"), (0, 0))),
            (true, Token::from(String::from("("), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_lparen());
        }
    }

    #[test]
    fn is_rparen() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("("), (0, 0))),
            (true, Token::from(String::from(")"), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_rparen());
        }
    }

    #[test]
    fn is_pointer() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("("), (0, 0))),
            (true, Token::new_pointer(0)),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_pointer());
        }
    }

    #[test]
    fn is_sub_exp() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("("), (0, 0))),
            (true, Token::new_sub(vec![])),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_sub_exp());
        }
    }

    #[test]
    fn is_root() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("("), (0, 0))),
            (true, Token::from(String::from("^"), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_root());
        }
    }
}
