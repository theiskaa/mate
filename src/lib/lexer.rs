//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use crate::{
    errors::Error,
    token::{Sub, SubMethod, Token},
    utils::ChUtils,
};
use std::{cell::Cell, collections::HashMap};

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer<'a> {
    input: &'a str,               // Expression input.
    examination_char: Cell<char>, // Current char under examination.
    position: Cell<usize>,        // Current position in input (points to current char).
    read_position: Cell<usize>,   // Current reading position in input (after current char).
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Result<Lexer<'a>, Error> {
        if input.is_empty() {
            return Err(Error::empty_input());
        }

        let first_char = input.chars().next().ok_or_else(Error::empty_input)?;

        Ok(Self {
            input,
            examination_char: Cell::new(first_char),
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
    pub fn lex(input: &'a str) -> Result<Sub, Error> {
        let lexer: Lexer = Lexer::new(input)?;

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

        match Lexer::nest_parentheses(tokens, input) {
            Err(e) => Err(e),
            Ok(v) => match Lexer::break_nesting(0, v, input) {
                Err(e) => Err(e),
                Ok(v) => Ok(Lexer::combine_tokens(v)),
            },
        }
    }

    // The nesting-to-tokens algorithm implementation.
    //
    // Nesting-to-tokens algorithm is a hashing algorithm that lexer uses to
    // parse parentheses expressions and put them into their nest level.
    //
    // For example if the given token list is -> "5 + (2 + 4) : (4 + 5 * (3 + 5))"
    // Generated result will be:  --> `Note: {<integer>} represents the pointer token`
    //  | 0: 5 + {1} : {2}
    //  | 1: 2 + 4
    //  | 2: 4 + 5 * {3}
    //  | 3: 3 + 5
    //
    // By storing tokens by their nesting levels, makes it easy to understand and implement
    // any kind of parentheses expressions as sub-expressions.
    fn nest_parentheses(
        tokens: Vec<Token>,
        input: &str,
    ) -> Result<HashMap<usize, (Vec<Token>, bool)>, Error> {
        let mut nested: HashMap<usize, (Vec<Token>, bool)> = HashMap::new();

        let mut level: usize = 0;

        let mut i: usize = 0;
        let mut startert: Token = Token::empty();
        while i < tokens.clone().len() {
            let t: Token = tokens[i].clone();

            if t.is_lparen() || t.is_labs() {
                startert = t.clone();

                let mut base: (Vec<Token>, bool) = match nested.get(&0) {
                    None => (vec![], false),
                    Some(v) => v.clone(),
                };

                level += 1;

                base.0
                    .push(Token::new_pointer(level, startert.to_submethod()));
                nested.insert(0, base.clone());

                match Lexer::take_till_end(tokens.clone(), i) {
                    None => {
                        return Err(Error::mismatched_parentheses(
                            input.to_string(),
                            t.index.1 + 1,
                        ))
                    }
                    Some(v) => {
                        nested.insert(level, (v.0, v.2));
                        i = v.1;
                    }
                };

                continue;
            } else if t.is_rparen() || t.is_rabs() {
                if startert.matchto(&t) {
                    startert = Token::empty();
                    i += 1;
                    continue;
                }
                return Err(Error::mismatched_parentheses(
                    input.to_string(),
                    t.index.1 + 1,
                ));
            }

            let mut base: (Vec<Token>, bool) = match nested.get(&0) {
                None => (vec![], false),
                Some(v) => v.clone(),
            };

            base.0.push(t);
            nested.insert(0, base.clone());
            i += 1;
        }

        Ok(nested)
    }

    // Collects all tokens from exact one parentheses-expression-clip.
    //
    // If [start] doesn't equals to any kind of opening(left) parentheses, result gonna be [None].
    fn take_till_end(tokens: Vec<Token>, start: usize) -> Option<(Vec<Token>, usize, bool)> {
        let mut iteration_count = start;
        let mut has_to_recall: bool = false;

        let mut level: i32 = 1;

        if start >= tokens.len() {
            return None;
        }

        let start_token = tokens[start].clone();
        if !start_token.is_lparen() && !start_token.is_labs() {
            return None;
        }

        // Initialize the matcho_collection with start_token.
        // In case of different kinds of parentheses([normal] and [abs]) we have to track the
        // nesting level by right matching token-types.
        // So, if the opening is normal parentheses token and closing is abs parentheses
        // token we shouldn't decrement the level.
        let mut matcho_collection: Vec<Token> = vec![start_token.clone()];

        let mut collected: Vec<Token> = Vec::new();
        for t in tokens.iter().skip(start + 1) {
            let t = t.clone();

            iteration_count += 1;

            if t.is_lparen() || t.is_labs() {
                level += 1;
                has_to_recall = true;
                matcho_collection.push(t.clone())
            }

            if t.is_rparen() || t.is_rabs() {
                match matcho_collection.last() {
                    None => return None,
                    Some(last) => {
                        if last.matchto(&t) {
                            level -= 1;
                            matcho_collection.pop();
                        } else {
                            return None;
                        }
                    }
                }

                if level == 0 {
                    return Some((collected, iteration_count, has_to_recall));
                }
            }

            collected.push(t);
        }

        if level != 0 {
            return None;
        }

        Some((collected, iteration_count, has_to_recall))
    }

    // Breaks the result of [nest_parentheses] into one line token list.
    // Runs into each nest-level indexed hash-map value and collects them into one line token
    // list.
    // If it's required to re-nest current nest-level indexed hash-map value, it calls
    // [nest_parentheses] inside of itself.
    fn break_nesting(
        point: usize,
        nested: HashMap<usize, (Vec<Token>, bool)>,
        input: &str,
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
                                let combined: Sub = Lexer::combine_tokens(v.0.clone());
                                result.push(Token::new_sub(combined.tokens, t.clone().sub.method));
                                continue;
                            }

                            match Lexer::nest_parentheses(v.0.clone(), input) {
                                Err(e) => return Err(e),
                                Ok(v) => match Lexer::break_nesting(0, v, input) {
                                    Err(e) => return Err(e),
                                    Ok(v) => {
                                        let combined: Sub = Lexer::combine_tokens(v);
                                        result.push(Token::new_sub(
                                            combined.tokens,
                                            t.clone().sub.method,
                                        ));
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
    // then continue with the other ones.
    // So, we have to convert the multiplication and division
    // parts of main expression into the sub expressions.
    // Combines function tokens with their arguments and factorial operators into single sub-expression tokens.
    // This ensures that function calls like sqrt(16) and factorials like 5! are treated as atomic units
    // and won't be broken up by operator precedence logic.
    fn combine_function_calls(tokens: Vec<Token>) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let current = &tokens[i];

            // Handle prefix functions: sqrt(16), sin(x), etc.
            if current.is_function() && i + 1 < tokens.len() {
                let arg = &tokens[i + 1];
                // Combine function and its argument into a sub-expression
                let func_call = Token::new_sub(
                    vec![current.clone(), arg.clone()],
                    SubMethod::PAREN,
                );
                result.push(func_call);
                i += 2; // Skip both function and argument
            }
            // Handle postfix factorial: 5!, (2+3)!
            else if (current.is_number() || current.is_sub_exp())
                && i + 1 < tokens.len()
                && tokens[i + 1].is_factorial()
            {
                // Combine number/subexp and factorial into a sub-expression
                let factorial_expr = Token::new_sub(
                    vec![current.clone(), tokens[i + 1].clone()],
                    SubMethod::PAREN,
                );
                result.push(factorial_expr);
                i += 2; // Skip both operand and factorial
            } else {
                result.push(current.clone());
                i += 1;
            }
        }

        result
    }

    fn combine_tokens(tokens: Vec<Token>) -> Sub {
        // First pass: combine function tokens with their arguments
        let tokens = Lexer::combine_function_calls(tokens);

        let mut combined_tokens: Vec<Token> = Vec::new();
        let mut sub_tokens: Vec<Token> = Vec::new();
        let mut power_subs: Vec<Token> = Vec::new();

        for i in 0..tokens.len() {
            let current = tokens[i].clone();
            let next = if i < tokens.len() - 1 {
                tokens[i + 1].clone()
            } else {
                Token::from(String::new(), Token::unknown_index())
            };

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
            let is_power_sub = !power_subs.is_empty()
                && (current.is_number() || current.is_sub_exp() || current.is_power() || current.is_function());
            if is_power_sub || next.is_power() && (current.is_number() || current.is_sub_exp() || current.is_function()) {
                power_subs.push(current.clone());
                continue;
            }

            if !power_subs.is_empty() {
                sub_tokens.push(Token::new_sub(
                    Lexer::combine_powers(power_subs.clone(), power_subs.clone().len() - 1),
                    SubMethod::PAREN,
                ));

                power_subs.clear();
            }

            let current_is_combinable = current.is_div_or_prod() || current.is_percentage();
            let next_is_combinable = next.is_div_or_prod() || next.is_percentage();
            let is_sub = !sub_tokens.is_empty()
                && (current.is_number() || current.is_sub_exp() || current_is_combinable || current.is_function());

            // Checks matching of new or exiting sub-token.
            if is_sub || next_is_combinable && (current.is_number() || current.is_sub_exp() || current.is_function()) {
                if !power_subs.is_empty() {
                    sub_tokens.push(Token::new_sub(
                        Lexer::combine_powers(power_subs.clone(), power_subs.len() - 1),
                        SubMethod::PAREN,
                    ));
                    power_subs.clear();
                }

                sub_tokens.push(current);
                continue;
            }

            if !sub_tokens.is_empty() {
                if sub_tokens.len() == 1 && sub_tokens.clone()[0].is_sub_exp() {
                    combined_tokens.append(&mut sub_tokens.clone());
                } else {
                    combined_tokens.push(Token::new_sub(sub_tokens.clone(), SubMethod::PAREN));
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
                sub_tokens.push(Token::new_sub(
                    Lexer::combine_powers(power_subs.clone(), power_subs.len() - 1),
                    SubMethod::PAREN,
                ))
            }
        }

        if combined_tokens.is_empty() {
            return Sub::new(sub_tokens, SubMethod::PAREN);
        }

        // Avoid appending sub-expression-token to empty tokens list.
        if !sub_tokens.is_empty() {
            if sub_tokens.len() == 1 && sub_tokens.clone()[0].is_sub_exp() {
                combined_tokens.append(&mut sub_tokens.clone()[0].sub.tokens);
            } else {
                combined_tokens.push(Token::new_sub(sub_tokens.clone(), SubMethod::PAREN));
            }
        }

        Sub::new(combined_tokens, SubMethod::PAREN)
    }

    // Combines 1D sub expression power tokens to actual nested-power sub-expression vector.
    // > For example: if given data is:
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

        let end = start as i32 - 2;
        if end < 0 {
            return combined_tokens;
        }

        let cpart: Vec<Token> = tokens.clone()[end as usize..=start].to_vec();
        combined_tokens.append(&mut tokens.clone()[..end as usize].to_vec());
        combined_tokens.push(Token::new_sub(cpart, SubMethod::PAREN));

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

            self.read_char()?;

            return Some(Ok(Token::from(ch, (position, position))));
        }

        // Check for a positive number.
        if ch.is_number() || ch.is_point() {
            match self.read_number() {
                None => return None,
                Some(v) => return Some(Ok(Token::from(v.0, v.1))),
            }
        }

        // Check for identifier (function name).
        let c = self.examination_char.get();
        if c.is_alphabetic() {
            match self.read_identifier() {
                None => return None,
                Some(v) => return Some(Ok(Token::from(v.0, v.1))),
            }
        }

        let lit: String = self.examination_char.get().to_string();
        self.read_char()?;

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
                Some(ch)
            }
            None => {
                if self.read_position.get() == self.input.len() {
                    let ch: char = self.input.chars().nth(self.position.get()).unwrap();

                    self.examination_char.set(ch);
                    self.position.set(self.read_position.get());
                    self.read_position.set(self.read_position.get() + 1);
                    return Some(ch);
                }

                None
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
        let char_at_start: char = self.input.chars().nth(start).unwrap_or('+');

        if char_at_start.to_string().is_plus_or_minus() {
            self.read_char()?;
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

        let num: String = input
            .chars()
            .skip(start)
            .take(self.position.get() - start)
            .collect();
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

    // Reads an identifier (function name) from the input.
    // Returns the identifier string and its position range.
    fn read_identifier(&self) -> Option<(String, (i32, i32))> {
        let start: usize = self.position.get();

        let mut ch: char = self.examination_char.get();
        while ch.is_alphabetic() {
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

        let ident: String = self
            .input
            .chars()
            .skip(start)
            .take(self.position.get() - start)
            .collect();

        let end = if ident.is_empty() {
            start
        } else {
            self.position.get() - 1
        };

        Some((ident, (start as i32, end as i32)))
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

        self.input.chars().nth(index)
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

        self.input.chars().nth(bindex as usize)
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
                    let is_abs: (bool, bool) = v.to_string().is_abs();
                    let is_factorial = v == '!';

                    return !is_paren.1 && !is_abs.1 && !is_factorial && !v.to_string().is_number();
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
        let test_data: HashMap<String, Result<Sub, Error>> = HashMap::from([
            (String::new(), Err(Error::empty_input())),
            (
                String::from("25"),
                Ok(Sub::new(
                    vec![Token::from(String::from("25"), (0, 1))],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("-25"),
                Ok(Sub::new(
                    vec![Token::from(String::from("-25"), (0, 2))],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(25)"),
                Ok(Sub::new(
                    vec![Token::new_sub(
                        vec![Token::from(String::from("25"), (1, 2))],
                        SubMethod::PAREN,
                    )],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(-25)"),
                Ok(Sub::new(
                    vec![Token::new_sub(
                        vec![Token::from(String::from("-25"), (1, 3))],
                        SubMethod::PAREN,
                    )],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("-25 + 5"),
                Ok(Sub::new(
                    vec![
                        Token::from(String::from("-25"), (0, 2)),
                        Token::from(String::from("+"), (4, 4)),
                        Token::from(String::from("5"), (6, 6)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("- - 2 + - 5"),
                Ok(Sub::new(
                    vec![
                        Token::from(String::from("-"), (0, 0)),
                        Token::from(String::from("-2"), (2, 4)),
                        Token::from(String::from("+"), (6, 6)),
                        Token::from(String::from("-5"), (8, 10)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("42 * 5"),
                Ok(Sub::new(
                    vec![
                        Token::from(String::from("42"), (0, 1)),
                        Token::from(String::from("*"), (3, 3)),
                        Token::from(String::from("5"), (5, 5)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("- 2 * 7 / 5 + - 20 / - 5"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("-2"), (0, 2)),
                                Token::from(String::from("*"), (4, 4)),
                                Token::from(String::from("7"), (6, 6)),
                                Token::from(String::from("/"), (8, 8)),
                                Token::from(String::from("5"), (10, 10)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("+"), (12, 12)),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("-20"), (14, 17)),
                                Token::from(String::from("/"), (19, 19)),
                                Token::from(String::from("-5"), (21, 23)),
                            ],
                            SubMethod::PAREN,
                        ),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(5 - 9) - 10"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("5"), (1, 1)),
                                Token::from(String::from("-"), (3, 3)),
                                Token::from(String::from("9"), (5, 5)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("-"), (8, 8)),
                        Token::from(String::from("10"), (10, 11)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(10 - 5) - (10 / 2)"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("10"), (1, 2)),
                                Token::from(String::from("-"), (4, 4)),
                                Token::from(String::from("5"), (6, 6)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("-"), (9, 9)),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("10"), (12, 13)),
                                Token::from(String::from("/"), (15, 15)),
                                Token::from(String::from("2"), (17, 17)),
                            ],
                            SubMethod::PAREN,
                        ),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("((10 - 5) - (10 / 2)) / 2"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("10"), (2, 3)),
                                        Token::from(String::from("-"), (5, 5)),
                                        Token::from(String::from("5"), (7, 7)),
                                    ],
                                    SubMethod::PAREN,
                                ),
                                Token::from(String::from("-"), (10, 10)),
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("10"), (13, 14)),
                                        Token::from(String::from("/"), (16, 16)),
                                        Token::from(String::from("2"), (18, 18)),
                                    ],
                                    SubMethod::PAREN,
                                ),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("/"), (22, 22)),
                        Token::from(String::from("2"), (24, 24)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(2 + 5) * (5 - 9 / (8 - 5))"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("2"), (1, 1)),
                                Token::from(String::from("+"), (3, 3)),
                                Token::from(String::from("5"), (5, 5)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("*"), (8, 8)),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("5"), (11, 11)),
                                Token::from(String::from("-"), (13, 13)),
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("9"), (15, 15)),
                                        Token::from(String::from("/"), (17, 17)),
                                        Token::new_sub(
                                            vec![
                                                Token::from(String::from("8"), (20, 20)),
                                                Token::from(String::from("-"), (22, 22)),
                                                Token::from(String::from("5"), (24, 24)),
                                            ],
                                            SubMethod::PAREN,
                                        ),
                                    ],
                                    SubMethod::PAREN,
                                ),
                            ],
                            SubMethod::PAREN,
                        ),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("5(5 / 2)(9 * 3)11"),
                Ok(Sub::new(
                    vec![
                        Token::from(String::from("5"), (0, 0)),
                        Token::from(String::from("*"), Token::unknown_index()),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("5"), (2, 2)),
                                Token::from(String::from("/"), (4, 4)),
                                Token::from(String::from("2"), (6, 6)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("*"), Token::unknown_index()),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("9"), (9, 9)),
                                Token::from(String::from("*"), (11, 11)),
                                Token::from(String::from("3"), (13, 13)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("*"), Token::unknown_index()),
                        Token::from(String::from("11"), (15, 16)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("5 ^ 3 ^ 2 ^ 5 * 19 - 50"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("5"), (0, 0)),
                                        Token::from(String::from("^"), (2, 2)),
                                        Token::new_sub(
                                            vec![
                                                Token::from(String::from("3"), (4, 4)),
                                                Token::from(String::from("^"), (6, 6)),
                                                Token::new_sub(
                                                    vec![
                                                        Token::from(String::from("2"), (8, 8)),
                                                        Token::from(String::from("^"), (10, 10)),
                                                        Token::from(String::from("5"), (12, 12)),
                                                    ],
                                                    SubMethod::PAREN,
                                                ),
                                            ],
                                            SubMethod::PAREN,
                                        ),
                                    ],
                                    SubMethod::PAREN,
                                ),
                                Token::from(String::from("*"), (14, 14)),
                                Token::from(String::from("19"), (16, 17)),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("-"), (19, 19)),
                        Token::from(String::from("50"), (21, 22)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("5 ^ 3 ^ 19"),
                Ok(Sub::new(
                    vec![
                        Token::from(String::from("5"), (0, 0)),
                        Token::from(String::from("^"), (2, 2)),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("3"), (4, 4)),
                                Token::from(String::from("^"), (6, 6)),
                                Token::from(String::from("19"), (8, 9)),
                            ],
                            SubMethod::PAREN,
                        ),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("(2 + 3 ^ 5) ^ 9"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("2"), (1, 1)),
                                Token::from(String::from("+"), (3, 3)),
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("3"), (5, 5)),
                                        Token::from(String::from("^"), (7, 7)),
                                        Token::from(String::from("5"), (9, 9)),
                                    ],
                                    SubMethod::PAREN,
                                ),
                            ],
                            SubMethod::PAREN,
                        ),
                        Token::from(String::from("^"), (12, 12)),
                        Token::from(String::from("9"), (14, 14)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("[2 - 12] - 10"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("2"), (1, 1)),
                                Token::from(String::from("-"), (3, 3)),
                                Token::from(String::from("12"), (5, 6)),
                            ],
                            SubMethod::ABS,
                        ),
                        Token::from(String::from("-"), (9, 9)),
                        Token::from(String::from("10"), (11, 12)),
                    ],
                    SubMethod::PAREN,
                )),
            ),
            (
                String::from("[7 - 14] * [5 - 9 / [5 - 3]]"),
                Ok(Sub::new(
                    vec![
                        Token::new_sub(
                            vec![
                                Token::from(String::from("7"), (1, 1)),
                                Token::from(String::from("-"), (3, 3)),
                                Token::from(String::from("14"), (5, 6)),
                            ],
                            SubMethod::ABS,
                        ),
                        Token::from(String::from("*"), (9, 9)),
                        Token::new_sub(
                            vec![
                                Token::from(String::from("5"), (12, 12)),
                                Token::from(String::from("-"), (14, 14)),
                                Token::new_sub(
                                    vec![
                                        Token::from(String::from("9"), (16, 16)),
                                        Token::from(String::from("/"), (18, 18)),
                                        Token::new_sub(
                                            vec![
                                                Token::from(String::from("5"), (21, 21)),
                                                Token::from(String::from("-"), (23, 23)),
                                                Token::from(String::from("3"), (25, 25)),
                                            ],
                                            SubMethod::ABS,
                                        ),
                                    ],
                                    SubMethod::PAREN,
                                ),
                            ],
                            SubMethod::ABS,
                        ),
                    ],
                    SubMethod::PAREN,
                )),
            ),
        ]);

        for (input, expected) in test_data {
            let result: Result<Sub, Error> = Lexer::lex(input.as_str());
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn mismatched_parentheses() {
        let test_cases: Vec<&str> = vec![
            "( ]",
            "[ )",
            "(5 + 3]",
            "[5 + 3)",
            "((5 + 3)",
            "(5 + 3))",
            "[[5 + 3]",
            "[5 + 3]]",
            "(5 + [3)",
            "[5 + (3]",
            "5 + (3 * [2)]",
        ];

        for input in test_cases {
            let result = Lexer::lex(input);
            assert!(
                result.is_err(),
                "Expected error for mismatched parentheses in: {}",
                input
            );
        }
    }

    #[test]
    fn valid_parentheses() {
        let test_cases: Vec<&str> = vec![
            "(5 + 3)",
            "[5 + 3]",
            "((5 + 3))",
            "[[5 + 3]]",
            "(5 + [3])",
            "[5 + (3)]",
            "((5) + (3))",
            "5 + (3 * [2 + 1])",
        ];

        for input in test_cases {
            let result = Lexer::lex(input);
            assert!(
                result.is_ok(),
                "Expected valid result for: {}, got error: {:?}",
                input,
                result
            );
        }
    }
}
