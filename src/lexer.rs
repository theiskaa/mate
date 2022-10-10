//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::{errors::Error, token::Token, utils::ChUtils};
use std::{cell::Cell, collections::HashMap};
use substring::Substring;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer<'a> {
    input: &'a str,               // Expression input.
    examination_char: Cell<char>, // Current char under examination.
    position: Cell<usize>,        // Current position in input (points to current char).
    read_position: Cell<usize>,   // Current reading position in input (after current char).
}

impl<'a> Lexer<'a> {
    // Creates a new Lexer object with given input.
    // Basically used at [Lexer::lex] function.
    fn new(input: &'a str) -> Result<Lexer, Error> {
        if input.len() < 1 {
            return Err(Error::empty_input());
        }

        Ok(Self {
            input,
            examination_char: Cell::new(input.chars().nth(0).unwrap()),
            position: Cell::from(0),
            read_position: Cell::from(1),
        })
    }

    // [Lex] is the main function that converts
    // each [char] to an understandable token variable.
    //
    //   USER INPUT
    //  ╭──────────────────────────╮
    //  │ (4 * 5 - 5) * 2 + 24 / 2 │
    //  ╰──────────────────────────╯
    //
    //   OUTPUT OF THE LEXER
    //  ╭───────────────────────────────────╮
    //  │                                   │    ╭─▶ First Sub Expression
    //  │   ╭───────────────────────────╮   │    │
    //  │   │                           │────────╯
    //  │   │   ╭───────────────────╮   │   │
    //  │   │   │                   │─╮ │   │
    //  │   │   │   ╭───────────╮   │ │ │   │
    //  │   │   │   │ NUMBER(4) │   │ ╰────────────▶ Second Sub Expression
    //  │   │   │   │ PRODUCT   │─╮ │   │   │        Which belongs to first sub expression.
    //  │   │   │   │ NUMBER(5) │ │ │   │   │
    //  │   │   │   ╰───────────╯ ╰──────────────╮
    //  │   │   │    MINUS          │   │   │    │
    //  │   │   │    NUMBER(5)      │   │   │    ╰─▶ Third Sub Expression
    //  │   │   │                   │   │   │        Which belongs to second sub expression.
    //  │   │   ╰───────────────────╯   │   │
    //  │   │                           │   │
    //  │   │    PRODUCT                │   │
    //  │   │    NUMBER(2)              │   │
    //  │   │                           │   │
    //  │   ╰───────────────────────────╯   │
    //  │                                   │
    //  │    PLUS                           │
    //  │                                   │
    //  │   ╭──────────────────────────╮    │    ╭─▶ Fourth Sub Expression
    //  │   │                          │    │    │
    //  │   │  NUMBER(24)              │    │    │
    //  │   │  DIVIDE                  │─────────╯
    //  │   │  NUMBER(2)               │    │
    //  │   │                          │    │
    //  │   ╰──────────────────────────╯    │
    //  │                                   │
    //  ╰───────────────────────────────────╯
    //
    pub fn lex(input: &'a str) -> Result<Vec<Token>, Error> {
        let lexer: Lexer = match Lexer::new(input) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let mut tokens: Vec<Token> = Vec::new();
        loop {
            match lexer.generate_token() {
                None => break,
                Some(r) => match r {
                    Err(e) => return Err(e),
                    Ok(r) => tokens.push(r),
                },
            }
        }

        match Lexer::nest_parentheses(tokens) {
            Err(e) => return Err(e),
            Ok(v) => match Lexer::break_nesting(0, v) {
                Err(e) => return Err(e),
                Ok(v) => return Ok(Lexer::combine_tokens(v)),
            },
        }
    }

    // The nesting-to-tokens algorithm implementation.
    // Nesting-to-tokens algorithm is a hashing algorithm that lexer uses to
    // parse parentheses expressions and put them into their nest level.
    //
    // For example if the given token list is -> "5 + (2 + 4) : (4 + 5 * (3 + 5))"
    // Generated result will be:  --> Note: {<integer>} represents the pointer token.
    //  | 0: 5 + {1} : {2}
    //  | 1: 2 + 4
    //  | 2: 4 + 5 * {3}
    //  | 3: 3 + 5
    //
    // By storing tokens by their nesting levels, makes it easy to understand and implement
    // parentheses expressions as sub-expressions.
    fn nest_parentheses(tokens: Vec<Token>) -> Result<HashMap<usize, (Vec<Token>, bool)>, Error> {
        let mut nested: HashMap<usize, (Vec<Token>, bool)> = HashMap::new();

        let mut level: usize = 0;

        let mut i: usize = 0;
        while i < tokens.clone().len() {
            if tokens[i].clone().is_lparen() {
                let mut base: (Vec<Token>, bool) = match nested.get(&0) {
                    None => (vec![], false),
                    Some(v) => v.clone(),
                };

                level += 1;

                base.0.push(Token::new_pointer(level));
                nested.insert(0, base.clone());

                match Lexer::take_till_end(tokens.clone(), i) {
                    None => return Err(Error::new(String::from("TODO: find a appropriate error"))),
                    Some(v) => {
                        let mut new: (Vec<Token>, bool) = (vec![], v.2);
                        for t in v.0.iter() {
                            new.0.push(t.clone());
                        }

                        nested.insert(level, new);
                        i = v.1;
                    }
                };

                continue;
            } else if tokens[i].clone().is_rparen() {
                i += 1;
                continue;
            }

            let mut base: (Vec<Token>, bool) = match nested.get(&0) {
                None => (vec![], false),
                Some(v) => v.clone(),
            };

            base.0.push(tokens[i].clone());
            nested.insert(0, base.clone());
            i += 1;
        }

        return Ok(nested);
    }

    // Collects all the tokens from exact one parentheses expression.
    // To make all stuff work well, pass the right starting point of your parentheses.
    // If [start] doesn't equals to opening parentheses, result gonna be [None].
    fn take_till_end(tokens: Vec<Token>, start: usize) -> Option<(Vec<Token>, usize, bool)> {
        let mut iteration_count = start;
        let mut has_to_recall: bool = false;

        let mut level: i32 = 1; // [start] indexed value should always equal to a opening parentheses
        if !tokens.clone()[start].is_lparen() || start > tokens.clone().len() {
            return None;
        }

        let mut collected: Vec<Token> = Vec::new();
        for i in (start + 1)..tokens.len() {
            iteration_count += 1;

            if tokens[i].clone().is_lparen() {
                level += 1;
                has_to_recall = true;
            }

            if tokens[i].clone().is_rparen() {
                level -= 1;
                if level == 0 {
                    return Some((collected, iteration_count, has_to_recall));
                }
            }

            collected.push(tokens[i].clone());
        }

        Some((collected, iteration_count, has_to_recall))
    }

    // Breaks the result of [nest_parentheses] into one line token list.
    // Runs into each nest-level indexed hash-map value and collects them into one line token
    // list.
    // If it's required to re-nest current nest-level indexed hash-map value, it calls
    // [nest_parentheses] and then itself inside of it.
    fn break_nesting(
        point: usize,
        nested: HashMap<usize, (Vec<Token>, bool)>,
    ) -> Result<Vec<Token>, Error> {
        let mut result: Vec<Token> = Vec::new();

        match nested.get(&point) {
            None => return Ok(result),
            Some(v) => {
                for t in v.0.iter() {
                    if !t.is_pointer() {
                        result.push(t.clone());
                        continue;
                    }

                    match nested.get(&t.clone().take_pointer_index().unwrap()) {
                        None => continue,
                        Some(v) => {
                            if !v.1 {
                                let combined: Vec<Token> = Lexer::combine_tokens(v.0.clone());
                                result.push(Token::new_sub(combined));
                                continue;
                            }

                            // If the tokens at current point in nested, contains parentheses
                            // that means we have to re-nest and re break them as tokens recursively..
                            match Lexer::nest_parentheses(v.0.clone()) {
                                Err(e) => return Err(e),
                                Ok(v) => match Lexer::break_nesting(0, v) {
                                    Err(e) => return Err(e),
                                    Ok(v) => {
                                        let combined: Vec<Token> = Lexer::combine_tokens(v);
                                        result.push(Token::new_sub(combined));
                                    }
                                },
                            }
                        }
                    }
                }
            }
        };

        Ok(result)
    }

    // Takes first-party tokens, combines them and returns
    // 1D nested tokens.
    //
    // In first inner result of token generation of [lex],
    // multiplication and division aren't collected together.
    // To take care of arithmetic's "process priority", we have
    // first calculate the multiplication or division action, and
    // then continue to the other ones.
    // So, that, we have to convert the multiplication and division
    // parts of main expression into the sub expressions.
    fn combine_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut combined_tokens: Vec<Token> = Vec::new();
        let mut sub_tokens: Vec<Token> = Vec::new();
        let mut power_subs: Vec<Token> = Vec::new();

        // Combine products/divisions/parentheses as sub-expression.
        for i in 0..tokens.len() {
            let next: Token;
            let current: Token = tokens[i].clone();
            if i < tokens.len() - 1 {
                next = tokens[i + 1].clone();
            } else {
                next = Token::from(String::new(), Token::unknown_index());
            }

            let is_auto_solids = current.is_number() && next.is_number()
                || current.is_sub_exp() && next.is_sub_exp();
            let is_auto_mixings = current.is_number() && next.is_sub_exp()
                || current.is_sub_exp() && next.is_number();

            // Auto append multiplication ◀╮
            // if there is no sign between │ two "number"(normal number and sub-exp) token.
            //    ╭──────────────────╭─────╯
            // ╭─ ▼ ───────╮     ╭── ▼ ─────────╮
            // │ 4(2 + 10) │ ──▶ │ 4 • (2 + 10) │
            // ╰───────────╯     ╰──────────────╯
            if is_auto_solids || is_auto_mixings {
                sub_tokens.append(&mut Vec::from([
                    current.clone(),
                    Token::from(String::from("*"), Token::unknown_index()),
                ]));
                continue;
            }

            // Collect power subs in different array to create a different sub expression with them.
            // By doing that we gonna easily keep operation priority safe.
            let is_power_sub = power_subs.len() > 0
                && (current.is_number() || current.is_sub_exp() || current.is_power());
            if is_power_sub || next.is_power() && (current.is_number() || current.is_sub_exp()) {
                power_subs.push(current.clone());
                continue;
            }

            if !power_subs.is_empty() {
                sub_tokens.push(Token::new_sub(Lexer::combine_powers(
                    power_subs.clone(),
                    power_subs.clone().len() - 1,
                )));

                power_subs.clear();
            }

            let current_is_combinable = current.is_div_or_prod() || current.is_percentage();
            let next_is_combinable = next.is_div_or_prod() || current.is_percentage();
            let is_sub = sub_tokens.len() > 0
                && (current.is_number() || current.is_sub_exp() || current_is_combinable);

            // Checks matching of new or exiting sub-token.
            if is_sub || next_is_combinable && (current.is_number() || current.is_sub_exp()) {
                if !power_subs.is_empty() {
                    sub_tokens.push(Token::new_sub(Lexer::combine_powers(
                        power_subs.clone(),
                        power_subs.len() - 1,
                    )));
                    power_subs.clear();
                }

                sub_tokens.push(current);
                continue;
            }

            if !sub_tokens.is_empty() {
                if sub_tokens.len() == 1 && sub_tokens.clone()[0].is_sub_exp() {
                    combined_tokens.append(&mut sub_tokens.clone());
                } else {
                    combined_tokens.push(Token::new_sub(sub_tokens.clone()));
                }

                sub_tokens.clear()
            }

            combined_tokens.push(current);
        }

        if !power_subs.is_empty() {
            if sub_tokens.is_empty() {
                sub_tokens.append(&mut Lexer::combine_powers(
                    power_subs.clone(),
                    power_subs.len() - 1,
                ));
            } else {
                sub_tokens.push(Token::new_sub(Lexer::combine_powers(
                    power_subs.clone(),
                    power_subs.len() - 1,
                )))
            }
        }

        if combined_tokens.is_empty() {
            return sub_tokens;
        }

        // Avoid appending sub-expression-token to empty tokens list.
        if !sub_tokens.is_empty() {
            if sub_tokens.len() == 1 && sub_tokens.clone()[0].is_sub_exp() {
                combined_tokens.append(&mut sub_tokens.clone()[0].sub_tokens);
            } else {
                combined_tokens.push(Token::new_sub(sub_tokens.clone()));
            }
        }

        return combined_tokens;
    }

    // Combines 1D sub expression power tokens to actual nested-power sub-expression vector.
    //  For example: if given data is:
    //   ╭────────────────╮                      ╭───────────────────╮
    //   │ 5 ^ 2 ^ 3 ^ 2  │ it'd be converted to │ 5 ^ (2 ^ (3 ^ 2)) │
    //   ╰────────────────╯                      ╰───────────────────╯
    //  We have to start reading from the ending, that's why we nest powers to individual
    //  sub-expression.
    //  By doing that we make it easy to understood by calculator.
    //  So, as a result it'd be resolved like:
    //  ╭───────────────────╮     ╭─────────────╮     ╭─────────╮     ╭───╮
    //  │ 5 ^ (2 ^ (3 ^ 2)) │ ──▶ │ 5 ^ (2 ^ 9) │ ──▶ │ 5 ^ 512 │ ──▶ │ ? │
    //  ╰───────────────────╯     ╰─────────────╯     ╰─────────╯     ╰───╯
    fn combine_powers(tokens: Vec<Token>, start: usize) -> Vec<Token> {
        if tokens.len() == 3 {
            return tokens;
        }

        let mut combined_tokens: Vec<Token> = Vec::new();

        let end = start.clone() as i32 - 2;
        if end < 0 {
            return combined_tokens;
        }

        let cpart: Vec<Token> = tokens.clone()[end as usize..=start.clone()].to_vec();
        combined_tokens.append(&mut tokens.clone()[..end as usize].to_vec());
        combined_tokens.push(Token::new_sub(cpart));

        if end <= 0 {
            return combined_tokens;
        }

        Lexer::combine_powers(combined_tokens, end as usize)
    }

    // Converts byte-character to token-structure.
    // Mainly used to generate 1D(first-party) tokens in [`lex`] method.
    //
    //         ╭─────────────╮ In second part of token generation, white(empty) spaces are auto-skipped
    //  ╭──────│───────────╮ │ by [skip_whitespace] method. And generate_token checks: {if that character is sign or not},
    //  │ 422  +  6  *  7  │ │ if it's, it firstly reads that character by [read_char].
    //  ╰──│───────────────╯ ╰───▶ And then creates new token by automatically filling token data.
    //     │
    //     │ In genesis, [`self.examination_char`] would be "4", and [generate_token] has to determine
    //     │ "4" can be not single-digit, it needs to reed full number not only "4".
    //     ╰───▶ So, [read_number] method will be used to read and return final number.
    //
    //   ... and so on ...
    //
    fn generate_token(&self) -> Option<Result<Token, Error>> {
        self.skip_whitespace();

        let ch: String = self.examination_char.get().to_string();
        let position: i32 = self.position.get() as i32;
        if ch.is_operation_sign() {
            if ch.is_plus_or_minus() && self.is_free_from_number(1) && self.next_is_number(1) {
                match self.read_number() {
                    None => return None,
                    Some(v) => return Some(Ok(Token::from(v.0, v.1))),
                }
            }

            if let None = self.read_char() {
                return None;
            };

            return Some(Ok(Token::from(ch, (position, position))));
        }

        // Check for a positive number.
        if ch.is_number() || ch.is_point() {
            match self.read_number() {
                None => return None,
                Some(v) => return Some(Ok(Token::from(v.0, v.1))),
            }
        }

        let lit: String = self.examination_char.get().to_string();
        if let None = self.read_char() {
            return None;
        }

        Some(Ok(Token::from(lit, (position, position))))
    }

    // A [char] reading functionality, that also updates state of lexer.
    // Reads char and fills lexer object with read and manipulated data.
    fn read_char(&self) -> Option<char> {
        match self.input.chars().nth(self.read_position.get()) {
            Some(ch) => {
                self.examination_char.set(ch);
                self.position.set(self.read_position.get());
                self.read_position.set(self.read_position.get() + 1);
                return Some(ch);
            }
            None => {
                if self.read_position.get() == self.input.len() {
                    let ch: char = self.input.chars().nth(self.position.get()).unwrap();

                    self.examination_char.set(ch);
                    self.position.set(self.read_position.get());
                    self.read_position.set(self.read_position.get() + 1);
                    return Some(ch);
                }

                return None;
            }
        }
    }

    // Collects from start to end of the string number,
    // and returns the full part of that number from input.
    //
    //  "-426.7" actually is a array of [char]s
    //  ╭────────────────────────────────────────────╮
    //  │ -426.7 ───▶ ['-', '4', '2', '6', '.', '7'] │
    //  ╰────────────────────────────────────────────╯
    //   To make computer understood that full number,
    //   We need to determine the start and end index
    //   of that full-number in rune array (from digit to digit).
    //
    fn read_number(&self) -> Option<(String, (i32, i32))> {
        let input: String = self.input.to_string();
        let start: usize = self.position.get();

        // Include negative/positive representation signs.
        let char_at_start: char = match self.input.chars().nth(start) {
            Some(ch) => ch,
            None => '+', // as default numbers are positive
        };

        if char_at_start.to_string().is_plus_or_minus() {
            if let None = self.read_char() {
                return None;
            }
        }

        // Keep reading forward chars if l.Char is number or number-point.
        let mut ch: char = self.examination_char.get();
        while ch.to_string().is_number() || ch.to_string().is_point() || ch == ' ' {
            match self.read_char() {
                Some(v) => ch = v,
                None => {
                    if self.read_position.get() >= self.input.len() {
                        break;
                    }

                    return None;
                }
            }
        }

        let num = input.substring(start, self.position.get()).to_string();
        let end = match num.chars().last() {
            None => self.position.get(),
            Some(v) => {
                if v != ' ' {
                    self.position.get() - 1
                } else {
                    self.position.get() - 2
                }
            }
        };

        Some((num, (start as i32, end as i32)))
    }

    // Eats all type of empty(white) spaces.
    fn skip_whitespace(&self) {
        let mut c: char = self.examination_char.get();
        while c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            match self.read_char() {
                Some(v) => c = v,
                None => break,
            }
        }
    }

    // Returns the next character by current position.
    //
    // [step] will be used to determine, how many steps we wanna go further.
    // As default (when you wanna go for one step next) you should make [step] <1>.
    fn peek_char(&self, step: usize) -> Option<char> {
        let index: usize = self.position.get() + step;
        if index >= self.input.len() {
            return None;
        }

        match self.input.chars().nth(index) {
            Some(ch) => return Some(ch),
            None => return None,
        }
    }

    // Returns the previous character by current position.
    //
    // [step] will be used to determine, how many steps we wanna go back.
    // As default (when you wanna go for one step back) you should make [step] <1>.
    fn peek_char_back(&self, step: usize) -> Option<char> {
        let bindex: i32 = self.position.get() as i32 - step as i32;
        if bindex < 0 {
            return None;
        }

        match self.input.chars().nth(bindex as usize) {
            Some(ch) => return Some(ch),
            None => return None,
        }
    }

    // Checks if the current positioned character is free from any number.
    //
    // If previous character of current position is white space, we should check for the next
    // previous one.
    fn is_free_from_number(&self, step: usize) -> bool {
        match self.peek_char_back(step) {
            None => true, // if there is nothing in back, then it's free from number.
            Some(v) => {
                if v != ' ' {
                    let is_paren: (bool, bool) = v.to_string().is_parentheses();
                    return !is_paren.1 && !v.to_string().is_number();
                }

                self.is_free_from_number(step + 1)
            }
        }
    }

    // Checks for a negative or sign provided number in the next of our current position.
    //
    // If next character of current position is white space, we should check for the next
    // of current next.
    fn next_is_number(&self, step: usize) -> bool {
        match self.peek_char(step) {
            None => false, // nothing != number
            Some(v) => {
                if v != ' ' {
                    return v.to_string().is_number();
                }

                self.next_is_number(step + 1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new() {
        let test_data: HashMap<&str, Result<Lexer, Error>> = HashMap::from([
            ("", Err(Error::empty_input())),
            (
                "4 + 2",
                Ok(Lexer {
                    input: "4 + 2",
                    examination_char: Cell::new('4'),
                    position: Cell::from(0),
                    read_position: Cell::from(1),
                }),
            ),
        ]);

        for (input, expected) in test_data {
            let result: Result<Lexer, Error> = Lexer::new(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn lex() {
        let test_data: HashMap<&str, Result<Vec<Token>, Error>> = HashMap::from([
            ("", Err(Error::empty_input())),
            ("25", Ok(vec![Token::from(String::from("25"), (0, 1))])),
            ("-25", Ok(vec![Token::from(String::from("-25"), (0, 2))])),
            (
                "(25)",
                Ok(vec![Token::new_sub(vec![Token::from(
                    String::from("25"),
                    (1, 2),
                )])]),
            ),
            (
                "(-25)",
                Ok(vec![Token::new_sub(vec![Token::from(
                    String::from("-25"),
                    (1, 3),
                )])]),
            ),
            (
                "-25 + 5",
                Ok(vec![
                    Token::from(String::from("-25"), (0, 2)),
                    Token::from(String::from("+"), (4, 4)),
                    Token::from(String::from("5"), (6, 6)),
                ]),
            ),
            (
                "- - 2 + - 5",
                Ok(vec![
                    Token::from(String::from("-"), (0, 0)),
                    Token::from(String::from("-2"), (2, 4)),
                    Token::from(String::from("+"), (6, 6)),
                    Token::from(String::from("-5"), (8, 10)),
                ]),
            ),
            (
                "42 * 5",
                Ok(vec![
                    Token::from(String::from("42"), (0, 1)),
                    Token::from(String::from("*"), (3, 3)),
                    Token::from(String::from("5"), (5, 5)),
                ]),
            ),
            (
                "- 2 * 7 / 5 + - 20 / - 5",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::from(String::from("-2"), (0, 2)),
                        Token::from(String::from("*"), (4, 4)),
                        Token::from(String::from("7"), (6, 6)),
                        Token::from(String::from("/"), (8, 8)),
                        Token::from(String::from("5"), (10, 10)),
                    ]),
                    Token::from(String::from("+"), (12, 12)),
                    Token::new_sub(vec![
                        Token::from(String::from("-20"), (14, 17)),
                        Token::from(String::from("/"), (19, 19)),
                        Token::from(String::from("-5"), (21, 23)),
                    ]),
                ]),
            ),
            (
                "(5 - 9) - 10",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::from(String::from("5"), (1, 1)),
                        Token::from(String::from("-"), (3, 3)),
                        Token::from(String::from("9"), (5, 5)),
                    ]),
                    Token::from(String::from("-"), (8, 8)),
                    Token::from(String::from("10"), (10, 11)),
                ]),
            ),
            (
                "(10 - 5) - (10 / 2)",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::from(String::from("10"), (1, 2)),
                        Token::from(String::from("-"), (4, 4)),
                        Token::from(String::from("5"), (6, 6)),
                    ]),
                    Token::from(String::from("-"), (9, 9)),
                    Token::new_sub(vec![
                        Token::from(String::from("10"), (12, 13)),
                        Token::from(String::from("/"), (15, 15)),
                        Token::from(String::from("2"), (17, 17)),
                    ]),
                ]),
            ),
            (
                "((10 - 5) - (10 / 2)) / 2",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::new_sub(vec![
                            Token::from(String::from("10"), (2, 3)),
                            Token::from(String::from("-"), (5, 5)),
                            Token::from(String::from("5"), (7, 7)),
                        ]),
                        Token::from(String::from("-"), (10, 10)),
                        Token::new_sub(vec![
                            Token::from(String::from("10"), (13, 14)),
                            Token::from(String::from("/"), (16, 16)),
                            Token::from(String::from("2"), (18, 18)),
                        ]),
                    ]),
                    Token::from(String::from("/"), (22, 22)),
                    Token::from(String::from("2"), (24, 24)),
                ]),
            ),
            (
                "(2 + 5) * (5 - 9 / (8 - 5))",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::from(String::from("2"), (1, 1)),
                        Token::from(String::from("+"), (3, 3)),
                        Token::from(String::from("5"), (5, 5)),
                    ]),
                    Token::from(String::from("*"), (8, 8)),
                    Token::new_sub(vec![
                        Token::from(String::from("5"), (11, 11)),
                        Token::from(String::from("-"), (13, 13)),
                        Token::new_sub(vec![
                            Token::from(String::from("9"), (15, 15)),
                            Token::from(String::from("/"), (17, 17)),
                            Token::new_sub(vec![
                                Token::from(String::from("8"), (20, 20)),
                                Token::from(String::from("-"), (22, 22)),
                                Token::from(String::from("5"), (24, 24)),
                            ]),
                        ]),
                    ]),
                ]),
            ),
            (
                "5(5 / 2)(9 * 3)11",
                Ok(vec![
                    Token::from(String::from("5"), (0, 0)),
                    Token::from(String::from("*"), Token::unknown_index()),
                    Token::new_sub(vec![
                        Token::from(String::from("5"), (2, 2)),
                        Token::from(String::from("/"), (4, 4)),
                        Token::from(String::from("2"), (6, 6)),
                    ]),
                    Token::from(String::from("*"), Token::unknown_index()),
                    Token::new_sub(vec![
                        Token::from(String::from("9"), (9, 9)),
                        Token::from(String::from("*"), (11, 11)),
                        Token::from(String::from("3"), (13, 13)),
                    ]),
                    Token::from(String::from("*"), Token::unknown_index()),
                    Token::from(String::from("11"), (15, 16)),
                ]),
            ),
            (
                "5 ^ 3 ^ 2 ^ 5 * 19 - 50",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::new_sub(vec![
                            Token::from(String::from("5"), (0, 0)),
                            Token::from(String::from("^"), (2, 2)),
                            Token::new_sub(vec![
                                Token::from(String::from("3"), (4, 4)),
                                Token::from(String::from("^"), (6, 6)),
                                Token::new_sub(vec![
                                    Token::from(String::from("2"), (8, 8)),
                                    Token::from(String::from("^"), (10, 10)),
                                    Token::from(String::from("5"), (12, 12)),
                                ]),
                            ]),
                        ]),
                        Token::from(String::from("*"), (14, 14)),
                        Token::from(String::from("19"), (16, 17)),
                    ]),
                    Token::from(String::from("-"), (19, 19)),
                    Token::from(String::from("50"), (21, 22)),
                ]),
            ),
            (
                "5 ^ 3 ^ 19",
                Ok(vec![
                    Token::from(String::from("5"), (0, 0)),
                    Token::from(String::from("^"), (2, 2)),
                    Token::new_sub(vec![
                        Token::from(String::from("3"), (4, 4)),
                        Token::from(String::from("^"), (6, 6)),
                        Token::from(String::from("19"), (8, 9)),
                    ]),
                ]),
            ),
            (
                "(2 + 3 ^ 5) ^ 9",
                Ok(vec![
                    Token::new_sub(vec![
                        Token::from(String::from("2"), (1, 1)),
                        Token::from(String::from("+"), (3, 3)),
                        Token::new_sub(vec![
                            Token::from(String::from("3"), (5, 5)),
                            Token::from(String::from("^"), (7, 7)),
                            Token::from(String::from("5"), (9, 9)),
                        ]),
                    ]),
                    Token::from(String::from("^"), (12, 12)),
                    Token::from(String::from("9"), (14, 14)),
                ]),
            ),
        ]);

        for (input, expected) in test_data {
            let result: Result<Vec<Token>, Error> = Lexer::lex(input);
            assert_eq!(result, expected);
        }
    }

    // TODO: should add tests for private functions also.
}
