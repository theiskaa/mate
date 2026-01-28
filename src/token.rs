//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::utils::ChUtils;

// The structure model for high level sub expression implementations.
// That could hold the actual source: tokens, and transformation method.
//
// For example: in case of parentheses and combinable
// operations(*, /, %) method have to be [PAREN].
// Or, in case of absolute values the method have to be [ABS],
// to let calculator know the approach it has to take to
// return final result of tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct Sub {
    pub tokens: Vec<Token>,
    pub method: SubMethod,
}

impl Sub {
    // Generate a new sub structure data.
    pub fn new(tokens: Vec<Token>, method: SubMethod) -> Self {
        Self { tokens, method }
    }

    // Generates a empty representation of token's sub element.
    pub fn empty() -> Self {
        Self {
            tokens: Vec::new(),
            method: SubMethod::PAREN,
        }
    }
}

// The method type of sub expression -> [Sub].
// Used to decide the final calculation method of sub expression tokens.
// For example, in case of [PAREN] the result will be default result of calculated [tokens].
// Or, in case of [ABS] the result always gonna be positive value.
#[derive(Clone, Debug, PartialEq)]
pub enum SubMethod {
    PAREN,
    ABS,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,

    // Internal
    NUMBER,

    // Sub related tokens
    SUBEXP,
    POINTER,
    LPAREN,
    RPAREN,
    LABS,
    RABS,

    // Operations
    PLUS,
    MINUS,
    PRODUCT,
    DIVIDE,
    PERCENTAGE,
    POWER,
}

// The main structure of input's each parsed character.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
    pub sub: Sub,
    // the index range of concrete token.
    // [-1] represents the unknown index.
    // left side is the starting point and right side is ending point.
    pub index: (i32, i32),
}

impl TokenType {
    // A function to get valid sub-method from token type.
    //
    // - [TokenType::ABS] is [SubMethod::ABS].
    // - [TokenType::LPAREN] & [TokenType::RPAREN] is [SubMethod::PAREN].
    pub fn to_submethod(&self) -> SubMethod {
        match self {
            TokenType::LABS => SubMethod::ABS,
            TokenType::RABS => SubMethod::ABS,
            _ => SubMethod::PAREN, // + LPAREN, RPAREN
        }
    }
}

impl Token {
    // Define a new Token value by providing all fields.
    pub fn new(typ: TokenType, literal: String, sub: Sub, index: (i32, i32)) -> Self {
        Self {
            typ,
            literal,
            sub,
            index,
        }
    }

    // Create a new sub token model with just sub tokens.
    pub fn new_sub(tokens: Vec<Token>, method: SubMethod) -> Self {
        Self {
            typ: TokenType::SUBEXP,
            literal: String::new(),
            sub: Sub { tokens, method },
            index: Token::unknown_index(),
        }
    }

    // Creates a pointer token, that newer will be used
    // at normal token result.
    pub fn new_pointer(i: usize, method: SubMethod) -> Self {
        Self {
            typ: TokenType::POINTER,
            literal: format!("{i}"),
            sub: Sub::new(Vec::new(), method),
            index: Token::unknown_index(),
        }
    }

    // Create a new token model from a literal.
    // The type is decided automatically by checking it.
    pub fn from(mut literal: String, index: (i32, i32)) -> Self {
        let typ = if literal.is_number() {
            TokenType::NUMBER
        } else {
            match literal.trim() {
                "+" => TokenType::PLUS,
                "-" => TokenType::MINUS,
                "*" | "•" => TokenType::PRODUCT,
                "/" | ":" => TokenType::DIVIDE,
                "(" => TokenType::LPAREN,
                ")" => TokenType::RPAREN,
                "%" => TokenType::PERCENTAGE,
                "^" => TokenType::POWER,
                "[" => TokenType::LABS,
                "]" => TokenType::RABS,
                _ => TokenType::ILLEGAL,
            }
        };

        // Clear the white-spaces from literal.
        literal.retain(|c| !c.is_whitespace());

        Self {
            typ,
            literal,
            sub: Sub::empty(),
            index,
        }
    }

    // Creates an empty Token model.
    pub fn empty() -> Self {
        Self {
            typ: TokenType::ILLEGAL,
            literal: String::new(),
            sub: Sub::empty(),
            index: (0, 0),
        }
    }

    // A function to get valid sub-method from token.
    //
    // - If [self.typ] is [ABS] is [SubMethod::ABS].
    // - If [self.typ] is [LPAREN] or [RPAREN] is [SubMethod::PAREN].
    pub fn to_submethod(&self) -> SubMethod {
        match &self.typ {
            TokenType::LABS => SubMethod::ABS,
            TokenType::RABS => SubMethod::ABS,
            _ => SubMethod::PAREN, // + LPAREN, RPAREN
        }
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

        self.literal.as_str().parse::<usize>().ok()
    }

    pub fn is_illegal(&self) -> bool {
        matches!(self.typ, TokenType::ILLEGAL)
    }

    pub fn is_lparen(&self) -> bool {
        matches!(self.typ, TokenType::LPAREN)
    }

    pub fn is_rparen(&self) -> bool {
        matches!(self.typ, TokenType::RPAREN)
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self.typ, TokenType::POINTER)
    }

    pub fn is_sub_exp(&self) -> bool {
        matches!(self.typ, TokenType::SUBEXP)
    }

    pub fn is_power(&self) -> bool {
        matches!(self.typ, TokenType::POWER)
    }

    pub fn is_labs(&self) -> bool {
        matches!(self.typ, TokenType::LABS)
    }

    pub fn is_rabs(&self) -> bool {
        matches!(self.typ, TokenType::RABS)
    }

    // Checks the "parentheses" family tokens' matching to each other.
    // So, if pointed(self) token is left-parentheses
    // given token(t) should be right-parentheses, if not returns false.
    pub fn matchto(&self, t: Token) -> bool {
        let m = match self.typ {
            TokenType::LPAREN => TokenType::RPAREN,
            TokenType::LABS => TokenType::RABS,
            _ => TokenType::ILLEGAL,
        };

        m == t.typ
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new_sub_struct() {
        let test_data: Vec<Sub> = vec![
            Sub {
                tokens: Vec::new(),
                method: SubMethod::PAREN,
            },
            Sub {
                tokens: Vec::new(),
                method: SubMethod::ABS,
            },
        ];

        for sub in test_data {
            let res = Sub::new(sub.clone().tokens, sub.clone().method);
            assert_eq!(res, sub)
        }
    }

    #[test]
    fn empty() {
        let test_data: Vec<Sub> = vec![Sub {
            tokens: Vec::new(),
            method: SubMethod::PAREN,
        }];

        for sub in test_data {
            let res = Sub::empty();
            assert_eq!(res, sub)
        }
    }

    #[test]
    fn to_submethod() {
        assert_eq!(TokenType::LABS.to_submethod(), SubMethod::ABS);
        assert_eq!(TokenType::RABS.to_submethod(), SubMethod::ABS);
        assert_eq!(TokenType::LPAREN.to_submethod(), SubMethod::PAREN);
        assert_eq!(TokenType::RPAREN.to_submethod(), SubMethod::PAREN);
    }

    #[test]
    fn new() {
        let test_data: Vec<Token> = vec![
            Token {
                typ: TokenType::PLUS,
                literal: String::from("+"),
                sub: Sub::empty(),
                index: (0, 0),
            },
            Token {
                typ: TokenType::MINUS,
                literal: String::from("-"),
                sub: Sub::empty(),
                index: (1, 1),
            },
            Token {
                typ: TokenType::DIVIDE,
                literal: String::from("/"),
                sub: Sub::empty(),
                index: (2, 2),
            },
            Token {
                typ: TokenType::SUBEXP,
                literal: String::from(""),
                sub: Sub::new(
                    Vec::from([
                        Token::from(String::from("2"), (0, 0)),
                        Token::from(String::from("+"), (1, 1)),
                        Token::from(String::from("5"), (2, 2)),
                    ]),
                    SubMethod::PAREN,
                ),
                index: (0, 2),
            },
        ];

        for t in test_data {
            let res = Token::new(
                t.clone().typ,
                t.clone().literal,
                t.clone().sub,
                t.clone().index,
            );

            assert_eq!(res.typ, t.clone().typ);
            assert_eq!(res.literal, t.clone().literal);
            assert_eq!(res.sub, t.clone().sub);
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
                    sub: Sub::new(
                        Vec::from([
                            Token::from(String::from("4"), (0, 0)),
                            Token::from(String::from("+"), (0, 0)),
                            Token::from(String::from("2"), (0, 0)),
                        ]),
                        SubMethod::PAREN,
                    ),
                    index: Token::unknown_index(),
                },
            ),
            (
                vec![String::from("2"), String::from("+"), String::from("+")],
                Token {
                    typ: TokenType::SUBEXP,
                    literal: String::new(),
                    sub: Sub::new(
                        Vec::from([
                            Token::from(String::from("2"), (0, 0)),
                            Token::from(String::from("+"), (0, 0)),
                            Token::from(String::from("+"), (0, 0)),
                        ]),
                        SubMethod::PAREN,
                    ),
                    index: Token::unknown_index(),
                },
            ),
        ]);

        for (t, expected) in test_data {
            let tokens = t.into_iter().map(|tt| Token::from(tt, (0, 0))).collect();
            let res = Token::new_sub(tokens, SubMethod::PAREN);

            assert_eq!(res.typ, expected.clone().typ);
            assert_eq!(res.literal, expected.clone().literal);
            assert_eq!(res.sub, expected.clone().sub);
            assert_eq!(res.index, expected.clone().index);
        }
    }

    #[test]
    fn new_pointer() {
        let test_data: HashMap<usize, Token> = HashMap::from([
            (
                0,
                Token::new(
                    TokenType::POINTER,
                    String::from("0"),
                    Sub::new(Vec::new(), SubMethod::PAREN),
                    (-1, -1),
                ),
            ),
            (
                99,
                Token::new(
                    TokenType::POINTER,
                    String::from("99"),
                    Sub::new(Vec::new(), SubMethod::ABS),
                    (-1, -1),
                ),
            ),
        ]);

        for (i, expected) in test_data {
            let token: Token = Token::new_pointer(i, expected.clone().sub.method);
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn from() {
        let test_data: HashMap<(String, (i32, i32)), Token> = HashMap::from([
            (
                (String::from("42"), (0, 1)),
                Token::new(TokenType::NUMBER, String::from("42"), Sub::empty(), (0, 1)),
            ),
            (
                (String::from("}"), (0, 0)),
                Token::new(TokenType::ILLEGAL, String::from("}"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("+"), (0, 0)),
                Token::new(TokenType::PLUS, String::from("+"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("-"), (0, 0)),
                Token::new(TokenType::MINUS, String::from("-"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("*"), (0, 0)),
                Token::new(TokenType::PRODUCT, String::from("*"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("•"), (0, 0)),
                Token::new(TokenType::PRODUCT, String::from("•"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("/"), (0, 0)),
                Token::new(TokenType::DIVIDE, String::from("/"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from(":"), (0, 0)),
                Token::new(TokenType::DIVIDE, String::from(":"), Sub::empty(), (0, 0)),
            ),
            (
                (String::from("%"), (0, 0)),
                Token::new(
                    TokenType::PERCENTAGE,
                    String::from("%"),
                    Sub::empty(),
                    (0, 0),
                ),
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
            (Some(0), Token::new_pointer(0, SubMethod::PAREN)),
            (Some(9), Token::new_pointer(9, SubMethod::PAREN)),
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
            (true, Token::from(String::from("|"), (0, 0))),
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
            (true, Token::new_pointer(0, SubMethod::PAREN)),
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
            (true, Token::new_sub(vec![], SubMethod::PAREN)),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_sub_exp());
        }
    }

    #[test]
    fn is_power() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("("), (0, 0))),
            (true, Token::from(String::from("^"), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_power());
        }
    }

    #[test]
    fn is_labs() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("]"), (0, 0))),
            (true, Token::from(String::from("["), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_labs());
        }
    }

    #[test]
    fn is_rabs() {
        let test_data: HashMap<bool, Token> = HashMap::from([
            (false, Token::from(String::from("-25"), (0, 1))),
            (false, Token::from(String::from("-"), (0, 0))),
            (false, Token::from(String::from("["), (0, 0))),
            (true, Token::from(String::from("]"), (0, 0))),
        ]);

        for (expected, token) in test_data {
            assert_eq!(expected, token.is_rabs());
        }
    }

    #[test]
    fn matchto() {
        let test_data: HashMap<bool, (Token, Token)> = HashMap::from([
            (
                true,
                (
                    Token::from(String::from("("), (0, 0)),
                    Token::from(String::from(")"), (0, 0)),
                ),
            ),
            (
                true,
                (
                    Token::from(String::from("["), (0, 0)),
                    Token::from(String::from("]"), (0, 0)),
                ),
            ),
            (
                false,
                (
                    Token::from(String::from("0"), (0, 0)),
                    Token::from(String::from("1"), (0, 0)),
                ),
            ),
        ]);

        for (expected, tokens) in test_data {
            assert_eq!(expected, tokens.0.matchto(tokens.1));
        }
    }
}
