use crate::utils::ChUtils;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    NUMBER,
    ILLEGAL,
    SUBEXP,

    // Operations
    PLUS,
    MINUS,
    PRODUCT,
    DIVIDE,
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

    // Checks if pointed token's type is illegal or not.
    pub fn is_illegal(&self) -> bool {
        match self.typ {
            TokenType::ILLEGAL => true,
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
        ]);

        for (literal, expected) in test_data {
            let res = Token::from(literal);
            assert_eq!(res, expected);
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
}
