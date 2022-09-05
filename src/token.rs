use crate::utils::ChUtils;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    NUMBER,
    ILLEGAL,

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
}

impl Token {
    // Define a new Token value by providing all fields.
    pub fn new(typ: TokenType, literal: String) -> Self {
        Self { typ, literal }
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

        return Self { typ, literal };
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
        let test_data: HashMap<String, TokenType> = HashMap::from([
            (String::from("+"), TokenType::PLUS),
            (String::from("-"), TokenType::MINUS),
            (String::from("/"), TokenType::DIVIDE),
        ]);

        for (literal, typ) in test_data {
            let res = Token::new(typ.clone(), literal.clone());

            assert_eq!(res.typ, typ.clone());
            assert_eq!(res.literal, literal.clone());
        }
    }

    #[test]
    fn from() {
        let test_data: HashMap<String, Token> = HashMap::from([
            (
                String::from("42"),
                Token::new(TokenType::NUMBER, String::from("42")),
            ),
            (
                String::from("}"),
                Token::new(TokenType::ILLEGAL, String::from("}")),
            ),
            (
                String::from("+"),
                Token::new(TokenType::PLUS, String::from("+")),
            ),
            (
                String::from("-"),
                Token::new(TokenType::MINUS, String::from("-")),
            ),
            (
                String::from("*"),
                Token::new(TokenType::PRODUCT, String::from("*")),
            ),
            (
                String::from("•"),
                Token::new(TokenType::PRODUCT, String::from("•")),
            ),
            (
                String::from("/"),
                Token::new(TokenType::DIVIDE, String::from("/")),
            ),
            (
                String::from(":"),
                Token::new(TokenType::DIVIDE, String::from(":")),
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
