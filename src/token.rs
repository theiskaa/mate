use crate::utils::ChUtils;

pub enum TokenType {
    NUMBER,
    ILLEGAL,

    // Operations
    PLUS,
    MINUS,
    PRODUCT,
    DIVIDE,
}

/// A small-block representing structure of lexer's input.
pub struct Token<'a> {
    typ: TokenType,
    literal: &'a str,
}

impl<'a> Token<'a> {
    /// Create a new token model with only literal.
    /// The type is decided automatically by checking it.
    pub fn new(literal: &'a str) -> Self {
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

        return Self { typ, literal };
    }
}
