//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::token::{Token, TokenType};

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
    // Left  Parentheses --> (
    // Right Parentheses --> )
    fn is_parentheses(&self) -> (bool, bool);

    // Checks if the given [&self] object is left abs or right abs sign.
    //
    // Left  ABS --> [
    // Right ABS --> ]
    fn is_abs(&self) -> (bool, bool);

    // Checks if the given [%self] object is percentage sign or not.
    fn is_percentage(&self) -> bool;
}

impl ChUtils for String {
    fn is_number(&self) -> bool {
        self.chars().any(|c| c.is_ascii_digit())
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

    fn is_parentheses(&self) -> (bool, bool) {
        (self.trim().eq("("), self.trim().eq(")"))
    }

    fn is_abs(&self) -> (bool, bool) {
        (self.trim().eq("["), self.trim().eq("]"))
    }

    fn is_percentage(&self) -> bool {
        self.trim().eq("%")
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

    fn is_parentheses(&self) -> (bool, bool) {
        match self.typ {
            TokenType::LPAREN => (true, false),
            TokenType::RPAREN => (false, true),
            _ => (false, false),
        }
    }

    fn is_abs(&self) -> (bool, bool) {
        match self.typ {
            TokenType::LABS => (true, false),
            TokenType::RABS => (false, true),
            _ => (false, false),
        }
    }

    fn is_percentage(&self) -> bool {
        match self.typ {
            TokenType::PERCENTAGE => true,
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
            assert_eq!(Token::from(target, (0, 0)).is_number(), expected);
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
            assert_eq!(Token::from(target, (0, 0)).is_plus_or_minus(), expected);
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
            assert_eq!(Token::from(target, (0, 0)).is_div_or_prod(), expected);
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
            assert_eq!(Token::from(target, (0, 0)).is_operation_sign(), expected);
        }
    }

    #[test]
    fn is_parentheses() {
        let test_data: HashMap<String, (bool, bool)> = HashMap::from([
            (String::from("/"), (false, false)),
            (String::from("*"), (false, false)),
            (String::from(":"), (false, false)),
            (String::from("‚Ä¢"), (false, false)),
            (String::from("-"), (false, false)),
            (String::from("+"), (false, false)),
            (String::from("5"), (false, false)),
            (String::from(")"), (false, true)),
            (String::from("("), (true, false)),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_parentheses(), expected);
            assert_eq!(Token::from(target, (0, 0)).is_parentheses(), expected);
        }
    }

    #[test]
    fn is_abs() {
        let test_data: HashMap<String, (bool, bool)> = HashMap::from([
            (String::from("/"), (false, false)),
            (String::from("*"), (false, false)),
            (String::from(":"), (false, false)),
            (String::from("‚Äö√Ñ¬¢"), (false, false)),
            (String::from("-"), (false, false)),
            (String::from("+"), (false, false)),
            (String::from("5"), (false, false)),
            (String::from("]"), (false, true)),
            (String::from("["), (true, false)),
        ]);

        for (target, expected) in test_data {
            assert_eq!(target.is_abs(), expected);
            assert_eq!(Token::from(target, (0, 0)).is_abs(), expected);
        }
    }

    #[test]
    fn is_percentage() {
        let test_data: HashMap<bool, String> = HashMap::from([
            (false, String::from("-25")),
            (false, String::from("-")),
            (false, String::from("(")),
            (true, String::from("%")),
        ]);

        for (expected, data) in test_data {
            assert_eq!(expected, data.is_percentage());
            assert_eq!(expected, Token::from(data, (0, 0)).is_percentage());
        }
    }
}
