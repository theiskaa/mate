use crate::{errors::Error, token::Token, utils::ChUtils};
use std::cell::Cell;
use substring::Substring;

// Lexer is the main lexical converter of the mate.
// It converts given string Input(expression) to an array of tokens.
//
// > VISUAL EXAMPLE OF LEXER
//
//      USER INPUT
//     ╭──────────────────────────╮
//     │ (4 * 5 - 5) * 2 + 24 / 2 │
//     ╰──────────────────────────╯
//
//      OUTPUT OF THE LEXER
//     ╭───────────────────────────────────╮
//     │                                   │    ╭─▶ First Sub Expression
//     │   ╭───────────────────────────╮   │    │
//     │   │                           │────────╯
//     │   │   ╭───────────────────╮   │   │
//     │   │   │                   │─╮ │   │
//     │   │   │   ╭───────────╮   │ │ │   │
//     │   │   │   │ NUMBER(4) │   │ ╰────────────▶ Second Sub Expression
//     │   │   │   │ PRODUCT   │─╮ │   │   │        Which belongs to first sub expression.
//     │   │   │   │ NUMBER(5) │ │ │   │   │
//     │   │   │   ╰───────────╯ ╰──────────────╮
//     │   │   │    MINUS          │   │   │    │
//     │   │   │    NUMBER(5)      │   │   │    ╰─▶ Third Sub Expression
//     │   │   │                   │   │   │        Which belongs to second sub expression.
//     │   │   ╰───────────────────╯   │   │
//     │   │                           │   │
//     │   │    PRODUCT                │   │
//     │   │    NUMBER(2)              │   │
//     │   │                           │   │
//     │   ╰───────────────────────────╯   │
//     │                                   │
//     │    PLUS                           │
//     │                                   │
//     │   ╭──────────────────────────╮    │    ╭─▶ Fourth Sub Expression
//     │   │                          │    │    │
//     │   │  NUMBER(24)              │    │    │
//     │   │  DIVIDE                  │─────────╯
//     │   │  NUMBER(2)               │    │
//     │   │                          │    │
//     │   ╰──────────────────────────╯    │
//     │                                   │
//     ╰───────────────────────────────────╯
//
#[derive(Clone)]
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
            return Err(Error::new("Cannot lex an empty input"));
        }

        Ok(Self {
            input,
            examination_char: Cell::new(input.chars().nth(0).unwrap()),
            position: Cell::from(0),
            read_position: Cell::from(1),
        })
    }

    // Loops through the [self.input], converts each [char] to an understandable token
    // variable.
    //
    // As a result we'd got a list of tokens, which will be used to calculate.
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

        Ok(tokens)
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
    fn generate_token(&self) -> Option<Result<Token, Error<'a>>> {
        self.skip_whitespace();

        let ch: String = self.examination_char.get().to_string();
        if ch.is_operation_sign() {
            if ch.is_plus_or_minus() && self.is_free_from_number(1) && self.next_is_number(1) {
                match self.read_number() {
                    None => return None,
                    Some(v) => return Some(Ok(Token::from(v))),
                }
            }

            if let None = self.read_char() {
                return None;
            };

            return Some(Ok(Token::from(ch)));
        }

        // Check for a positive number.
        if ch.is_number() || ch.is_point() {
            match self.read_number() {
                None => return None,
                Some(v) => return Some(Ok(Token::from(v))),
            }
        }

        let lit: String = self.examination_char.get().to_string();
        if let None = self.read_char() {
            return None;
        }

        Some(Ok(Token::from(lit)))
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
    fn read_number(&self) -> Option<String> {
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

        Some(input.substring(start, self.position.get()).to_string())
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
                    return !v.to_string().is_number();
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
