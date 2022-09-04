use crate::utils::ChUtils;

#[derive(Clone)]
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
#[derive(Clone)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    /// Create a new token model from a literal.
    /// The type is decided automatically by checking it.
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

    /// Checks if pointed token's type is illegal or not.
    pub fn is_illegal(&self) -> bool {
        match self.typ {
            TokenType::ILLEGAL => true,
            _ => false,
        }
    }
}
