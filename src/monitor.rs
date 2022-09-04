use crate::token::{Token, TokenType};

pub trait Monitor {
    // Converts the [&self] object to the
    // right String representing value.
    fn to_string(&self) -> String;
}

/// A monitor debugger implementation for [Token].
impl Monitor for Token {
    fn to_string(&self) -> String {
        String::from(self.typ.to_string() + " | " + self.literal.as_str())
    }
}

/// A monitor debugger implementation for [TokenType].
impl Monitor for TokenType {
    fn to_string(&self) -> String {
        let data = match self {
            TokenType::NUMBER => "NUMBER",
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::PRODUCT => "PRODUCT",
            TokenType::DIVIDE => "DIVIDE",
        };

        String::from(data)
    }
}
