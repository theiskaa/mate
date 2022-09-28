<p align="center">
 <img width="350" src="https://user-images.githubusercontent.com/59066341/138941465-a4354274-3976-4571-bdcd-df031d7d4761.png" alt="Package Logo">
 <br>
 <a href="https://github.com/theiskaa/mate/blob/main/LICENSE">
  <img src="https://img.shields.io/badge/License-MIT-red.svg" alt="License: MIT"/>
 </a>
</p>

Mate is a library for parsing and calculating arithmetic expressions inputted as &str(string). Uses Lexer(similar to interpreted-programming languages' lexer) structure to parse string input in to token list, and Calculator structure to calculate final result from token list. Implements also a general wrapper structure that implements Lexer and Calculator inside of it. And makes it easy to calculate arithmetic expression's result directly without dealing with parsing and calculating manually.

# Usage

This crate is on [crates.io](http://crates.io/crates/mate-rs) and can be used by adding mate-rs to your dependencies in your project's `Cargo.toml`.
```toml
[dependencies]
mate-rs = "0.1.3"
```

## Example: Simple usage | `Mate`

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

## Example: Complicated usage | `Lexer` and `Calculator`

`Lexer` is the main structure that parses string-input to token-list.
`Calculator` is the structure that used to calculate final result via `Lexer`'s result.

```rust
use mate_rs::{calculator::Calculator, lexer::Lexer};

// Generated tokens gonna be something like:
//  | Token(type: NUMBER  literal: "-2"),
//  | Token(type: PLUS    literal: "+"),
//  | Token(type: NUMBER  literal: "2"),
//  | Token(type: PLUS    literal: "+"),
//  | Token(
//  |   type: SUBEXP,
//  |   tokens: [
//  |        Token(type: NUMBER,  literal: "6")
//  |        Token(type: PRODUCT, literal: "*")
//  |        Token(type: NUMBER,  literal: "7")
//  |   ],
//  | ),
let tokens = Lexer::lex(" - 2 + 2 + 6 * 7").unwrap(); // should handle error case also

// Result will be calculated from tokens, by X/O/Y algorithm.
let result = Calculator::calculate(tokens);

match result {
    Ok(v) => assert_eq!(v, 42.0),
    Err(_) => {
        // Do something ...
    }
};
```

---

# How it works
Mate is all about two main structures, [Lexer] and [Calculator].
[Lexer] is the structure that takes care of parsing given string expression, 
and [Calculator] is the structure that takes care of calculating final result via parsed tokens 

## Lexer
Loops through the given input string, reads and converts each character to an [Token] structure.
We've several types of main tokens and they are:
- `ILLEGAL` - illegal character.
- `NUMBER` - number type.
- `MINUS`, `PLUS`, `PRODUCT`, `DIVIDE`, `PERCENTAGE`, `ROOT` - operations.
- `LPAREN`, `RPAREN` - parentheses.
- `SUBEXP` - sub expression, expressions inside of parentheses or combinations of division and multiplication.

Lexer's `lex` functionality converts each character to one of these tokens.
It combines multiplication or division operation related tokens into one sub-expression to keep the operation-priority right.
And nests the parentheses with a custom level-to-expression algorithm.

level-to-expression algorithm is mapping algorithm that maps concrete expression to it's nesting level.

*For example if the given token list is* -> `(2 + 5) : (5 - 9 / (8 - 5))`.
*Generated result will be:*

<img width="500" alt="mate" src="https://user-images.githubusercontent.com/59066341/192025304-220c58eb-8bbe-4820-bd6a-5f18b5b5758b.png">

**By doing that we make it easy to keep the operation-priority safe.**

## Calculator

Calculator takes the parsed token-list and calculates final result of it.
Uses custom `X/O/Y` algorithm a.k.a `X/OPERATION/Y` where `X` and `Y` are numbers, and `O` is operation.
If cannot get the `X` or `Y` takes them as zero.
```
╭────────╮ ╭───────────╮ ╭────────╮
│ NUMBER │ │ OPERATION │ │ NUMBER │
╰────────╯ ╰───────────╯ ╰────────╯
     ╰───╮       │        ╭───╯
         ▼       ▼        ▼
         X  [+, -, *, /]  Y
```
