/*!
This crate provides a library for parsing and calculating arithmetic expressions inputted as &str(string).
Uses `Lexer` structure to parse string input in to token list, and `Calculator` structure to calculate final result from token list.
Has also a general wrapper structure that implements `Lexer` and `Calculator` inside of it. And makes it easy to calculate arithmetic
expression's result directly without dealing with parsing and calculating manually.

# Usage

This crate is on [crates.io](http://crates.io/crates/mate-rs) and can be used by adding mate-rs to your dependencies in your project's `Cargo.toml`.
```toml
[dependencies]
mate-rs = "0.1.3"
```

# Example: with `Mate`

`Mate` is general wrapper structure for `Lexer` and `Calculator`.
has only one method that used to `calculate` result via string(&str) input.

```rust
use mate_rs::mate::Mate;

let result = Mate::calculate("6 * 7");
match result {
    Ok(v) => assert_eq!(v, 42.0),
    Err(_) => {
        // Do something ...
    }
};
```

# Example: with `Lexer` and `Calculator`

`Lexer` is the main structure that parses string-input to token-list.
`Calculator` is the structure that used to calculate final result via `Lexer`'s result.

```rust
use mate_rs::{calculator::Calculator, lexer::Lexer};

// Generated tokens gonna be something like:
//  |
//  | Token(
//  |   type: SUBEXP,
//  |   tokens: [
//  |        Token(
//  |          type: SUBEXP,
//  |          tokens: [
//  |               Token(type: NUMBER,  literal: "2")
//  |               Token(type: PLUS,    literal: "+")
//  |               Token(type: NUMBER,  literal: "5")
//  |          ],
//  |        ),
//  |        Token(type: PRODUCT, literal: "*"),
//  |        Token(
//  |          type: SUBEXP,
//  |          tokens: [
//  |               Token(type: NUMBER,  literal: "5")
//  |               Token(type: MINUS,   literal: "-")
//  |               Token(type: NUMBER,  literal: "9")
//  |               Token(type: PLUS,    literal: "+")
//  |               Token(
//  |                 type: SUBEXP,
//  |                 tokens: [
//  |                      Token(type: NUMBER,  literal: "8")
//  |                      Token(type: PLUS,    literal: "-")
//  |                      Token(type: NUMBER,  literal: "5")
//  |                 ],
//  |               ),
//  |          ],
//  |        ),
//  |   ],
//  | ),
//  | Token(type: PLUS,    literal: "+")
//  | Token(type: NUMBER,  literal: "35")
//  |
let input = "[ (2 + 5) * (5 - 9 + (8 - 5)) ] + 35";
let tokens = Lexer::lex(input.clone()).unwrap(); // should handle error case also

// Result will be calculated from tokens, by X/O/Y algorithm.
//
//  ╭────────╮ ╭───────────╮ ╭────────╮
//  │ NUMBER │ │ OPERATION │ │ NUMBER │
//  ╰────────╯ ╰───────────╯ ╰────────╯
//       ╰───╮       │        ╭───╯
//           ▼       ▼        ▼
//           X  [+, -, *, /]  Y
//
let result = Calculator::calculate(tokens, input.clone());

match result {
    Ok(v) => assert_eq!(v, 42.0),
    Err(_) => {
        // Do something ...
    }
};
```

> For details refer to [repository](https://github.com/theiskaa/mate).
*/

pub mod calculator;
pub mod errors;
pub mod lexer;
pub mod mate;
pub mod monitor;
pub mod token;
pub mod utils;
