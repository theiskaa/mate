use crate::errors::Error;
use crate::token::Token;
use crate::utils::ChUtils;
use substring::Substring;

/// A structure model of the lexer model of application.
pub struct Lexer<'a> {
    input: &'a str,         // Expression input.
    examination_char: char, // Current char under examination.
    position: usize,        // Current position in input (points to current char).
    read_position: usize,   // Current reading position in input (after current char).
}

impl<'a> Lexer<'a> {
    /// Creates a new full-fit lexer model.
    /// Takes [input] and fills all other required fields.
    pub fn new(input: &'static str) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::new("Cannot create a new lexer with empty input"));
        }

        // TODO: call lexer::readChar().

        let l: Lexer = Self {
            input,
            examination_char: input.chars().nth(0).unwrap(),
            position: 0,      // TODO: re-append right index
            read_position: 1, // TODO: re-append right index
        };

        Ok(l)
    }

    /// TODO: implement diagram
    fn lex(&mut self, input: &'static str) -> Result<Vec<Token>, Error> {
        // TODO: implement functionality.
        Ok(vec![])
    }

    /// Converts byte-character to token-structure.
    /// Mainly used to generate 1D(first-party) tokens in [`lex`] method.
    ///
    ///         ╭─────────────╮ In second part of token generation, white(empty) spaces are auto-skipped
    ///  ╭──────│───────────╮ │ by [skip_whitespace] method and generate_token checks: {if that character is sign or not},
    ///  │ 422  +  6  *  7  │ │ if it's, method, firstly reads that character by [read_char].
    ///  ╰──│───────────────╯ ╰───▶ And then creates new token by automatically filling token data.
    ///     │
    ///     │ In genesis, [`self.examination_char`] would be "4", and [generate_token] has to determine
    ///     │ "4" can be not single-digit, it needs to reed full number not only "4".
    ///     ╰───▶ So, [read_number] method will be used to read and return final number.
    ///
    ///   ... and so on ...
    ///
    fn generate_token(&mut self) -> Token {
        self.skip_whitespace();

        // TODO: implement generate_token functionality.

        Token::new("")
    }

    /// Returns the next character by current position.
    fn peek_char(&mut self) -> Option<char> {
        let position_char: Option<char> = self.input.chars().nth(self.read_position);
        match position_char {
            Some(ch) => return Some(ch),
            None => return None,
        }
    }

    /// Returns the previous character by current position.
    fn peek_char_back(&mut self) -> Option<char> {
        let position_char: Option<char> = self.input.chars().nth(self.position - 1);
        match position_char {
            Some(ch) => return Some(ch),
            None => return None,
        }
    }

    /// Reads char and fills lexer object with read and manipulated data
    fn read_char(&mut self) -> Option<char> {
        let position_char: Option<char> = self.input.chars().nth(self.read_position);
        match position_char {
            None => return None,
            Some(ch) => {
                self.examination_char = ch;
                self.position = self.read_position;
                self.read_position += 1;
                return Some(ch);
            }
        }
    }

    /// Eats all type of empty(white) spaces.
    fn skip_whitespace(&mut self) {
        let c: char = self.examination_char;
        while c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            _ = self.read_char()
        }
    }

    ///
    /// Goes and collects from start to end of
    /// the string number, and returns the full part of that number from input.
    ///
    ///  "-426.7" actually is a array of [char]s
    ///  ╭────────────────────────────────────────────╮
    ///  │ -426.7 ───▶ ['-', '4', '2', '6', '.', '7'] │
    ///  ╰────────────────────────────────────────────╯
    ///   To make program understood that full number,
    ///   We need to determine the start and end index
    ///   of that full-number in rune array (from digit to digit).
    ///
    fn read_number(&mut self) -> &'a str {
        let start: usize = self.position;

        // Include negative/positive representation signs.
        let char_at_start: Option<char> = self.input.chars().nth(start);
        let char_at_start: char = match char_at_start {
            Some(ch) => ch,
            None => '+', // as default numbers are positive
        };

        let is_plus_or_minus: bool = char_at_start.to_string().as_str().is_plus_or_minus();
        if is_plus_or_minus {
            _ = self.read_char()
        }

        let ch: char = self.examination_char;
        let is_number: bool = ch.to_string().as_str().is_number();
        let is_point: bool = ch.to_string().as_str().is_point();

        // Keep reading forward chars if l.Char is number or number-point.
        while is_number || is_point {
            _ = self.read_char()
        }

        self.input.substring(start, self.position)
    }
}
