use crate::token::{Token, TokenType};
use regex::Regex;

//
// A interface for custom char-type-checking utility methods.
// Has a various methods (checkers) based on [&str].
//
pub trait ChUtils {
    // Checks if the given [&self] object is number or not.
    fn is_number(&self) -> bool;

    // Checks if the given [&self] object is point(comma, dot) or not.
    //
    // Like <.> in 3.14 or <,> in 3,14
    fn is_point(&self) -> bool;

    // Checks if the given [&self] object is plus sign or minus sign.
    //
    // Plus signs   --> <+>
    // Minus signs  --> <->
    fn is_plus_or_minus(&self) -> bool;

    // Checks if the given [&self] object is division sign or multiplication sign.
    //
    // Division signs        --> <:> and </>
    // Multiplication signs  --> <*> and <•>
    fn is_div_or_prod(&self) -> bool;

    // A function that combines [is_plus_or_minus] and [is_div_or_prod].
    // So, it checks if [&self] object is operation sign or not.
    //
    // Plus signs            --> <+>
    // Minus signs           --> <->
    // Division signs        --> <:> and </>
    // Multiplication signs  --> <*> and <•>
    fn is_operation_sign(&self) -> bool;

    // Checks if the given [&self] object is left parentheses or right parentheses sign.
    //
    // Left Parentheses  --> (
    // Right Parentheses --> )
    fn is_parentheses(&self) -> bool;
}

impl ChUtils for String {
    fn is_number(&self) -> bool {
        Regex::new("[[:digit:]]").unwrap().is_match(self)
    }

    fn is_point(&self) -> bool {
        self.trim().eq(".") || self.trim().eq(",")
    }

    fn is_plus_or_minus(&self) -> bool {
        self.trim().eq("+") || self.trim().eq("-")
    }

    fn is_div_or_prod(&self) -> bool {
        let is_div: bool = self.trim().eq(":") || self.trim().eq("/");
        let is_prod: bool = self.trim().eq("*") || self.trim().eq("•");

        is_div || is_prod
    }

    fn is_operation_sign(&self) -> bool {
        self.is_plus_or_minus() || self.is_div_or_prod()
    }

    fn is_parentheses(&self) -> bool {
        self.trim().eq("(") || self.trim().eq(")")
    }
}

impl ChUtils for Token {
    fn is_number(&self) -> bool {
        match self.typ {
            TokenType::NUMBER => true,
            _ => false,
        }
    }

    fn is_point(&self) -> bool {
        false // token has not point type at all.
    }

    fn is_plus_or_minus(&self) -> bool {
        match self.typ {
            TokenType::PLUS => true,
            TokenType::MINUS => true,
            _ => false,
        }
    }

    fn is_div_or_prod(&self) -> bool {
        match self.typ {
            TokenType::PRODUCT => true,
            TokenType::DIVIDE => true,
            _ => false,
        }
    }

    fn is_operation_sign(&self) -> bool {
        self.is_plus_or_minus() || self.is_div_or_prod()
    }

    fn is_parentheses(&self) -> bool {
        match self.typ {
            TokenType::LPAREN => true,
            TokenType::RPAREN => true,
            _ => false,
        }
    }
}

// Includes tests for only String implementation of [ChUtils].
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn is_number() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("42"), true),
            (String::from("-25"), true),
            (String::from("+50"), true),
            (String::from("-"), false),
            (String::from("+"), false),
            (String::from("/"), false),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_number(), expected);
            assert_eq!(Token::from(target).is_number(), expected);
        }
    }

    #[test]
    fn is_point() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("."), true),
            (String::from(","), true),
            (String::from("-"), false),
            (String::from("+"), false),
            (String::from("/"), false),
            (String::from("5"), false),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_point(), expected);
        }
    }

    #[test]
    fn is_plus_or_minus() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("-"), true),
            (String::from("+"), true),
            (String::from("/"), false),
            (String::from(".5"), false),
            (String::from("/"), false),
            (String::from("*"), false),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_plus_or_minus(), expected);
            assert_eq!(Token::from(target).is_plus_or_minus(), expected);
        }
    }

    #[test]
    fn is_div_or_prod() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("/"), true),
            (String::from("*"), true),
            (String::from(":"), true),
            (String::from("•"), true),
            (String::from("-"), false),
            (String::from("+"), false),
            (String::from(".5"), false),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_div_or_prod(), expected);
            assert_eq!(Token::from(target).is_div_or_prod(), expected);
        }
    }

    #[test]
    fn is_operation_sign() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("/"), true),
            (String::from("*"), true),
            (String::from(":"), true),
            (String::from("•"), true),
            (String::from("-"), true),
            (String::from("+"), true),
            (String::from("5"), false),
            (String::from("."), false),
            (String::from(","), false),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_operation_sign(), expected);
            assert_eq!(Token::from(target).is_operation_sign(), expected);
        }
    }

    #[test]
    fn is_parentheses() {
        let test_data: HashMap<String, bool> = HashMap::from([
            (String::from("/"), false),
            (String::from("*"), false),
            (String::from(":"), false),
            (String::from("‚Ä¢"), false),
            (String::from("-"), false),
            (String::from("+"), false),
            (String::from("5"), false),
            (String::from(")"), true),
            (String::from("("), true),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_parentheses(), expected);
            assert_eq!(Token::from(target).is_parentheses(), expected);
        }
    }
}
