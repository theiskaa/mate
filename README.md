<p align="center">
 <img width="350" src="https://user-images.githubusercontent.com/59066341/138941465-a4354274-3976-4571-bdcd-df031d7d4761.png" alt="Package Logo">
 <br>
 <a href="https://github.com/theiskaa/mate/blob/main/LICENSE">
  <img src="https://img.shields.io/badge/License-MIT-red.svg" alt="License: MIT"/>
 </a>
</p>

Mate is a library designed for parsing and calculating arithmetic expressions inputted as strings. It utilizes a Lexer structure, similar to those found in interpreted programming languages, to parse string input into a token list. The Calculator structure is then used to compute the final result from this token list. A general wrapper structure is also implemented that encapsulates both the Lexer and Calculator, simplifying the process of calculating arithmetic expression results without the need for manual parsing and calculation.

**Note**: This project serves as a demonstration and is not intended for production use. It represents my initial steps in learning to create a programming language.

# Usage

This crate is available on [crates.io](http://crates.io/crates/mate-rs) and can be added to your project's dependencies in your `Cargo.toml` file.
```toml
[dependencies]
mate-rs = "0.1.4"
```

## Example: Using `Mate`

`Mate` is a general wrapper structure for `Lexer` and `Calculator`. It provides a single method for calculating results from string input.

```rust
use mate_rs::mate::Mate;

let result = Mate::calculate("6 * 7");
match result {
    Ok(v) => assert_eq!(v, 42.0),
    Err(_) => {
        // Handle error ...
    }
};
```

## Example: Using `Lexer` and `Calculator`

`Lexer` is the primary structure that parses string input into a token list. `Calculator` is used to compute the final result from the `Lexer`'s output.

```rust
use mate_rs::{calculator::Calculator, lexer::Lexer};

let input = "[ (2 + 5) * (5 - 9 + (8 - 5)) ] + 35";
let tokens = Lexer::lex(input.clone()).unwrap(); // Error handling should also be implemented
```

<details close>
<summary>View the generated token tree</summary>
<br>
<pre>
<code>
// The generated tokens will resemble the following:
//
//   Token(
//     type: SUBEXP,
//     tokens: [
//          Token(
//            type: SUBEXP,
//            tokens: [
//                 Token(type: NUMBER,  literal: "2")
//                 Token(type: PLUS,    literal: "+")
//                 Token(type: NUMBER,  literal: "5")
//            ],
//          ),
//          Token(type: PRODUCT, literal: "*"),
//          Token(
//            type: SUBEXP,
//            tokens: [
//                 Token(type: NUMBER,  literal: "5")
//                 Token(type: MINUS,   literal: "-")
//                 Token(type: NUMBER,  literal: "9")
//                 Token(type: PLUS,    literal: "+")
//                 Token(
//                   type: SUBEXP,
//                   tokens: [
//                        Token(type: NUMBER,  literal: "8")
//                        Token(type: PLUS,    literal: "-")
//                        Token(type: NUMBER,  literal: "5")
//                   ],
//                 ),
//            ],
//          ),
//     ],
//   ),
//   Token(type: PLUS,    literal: "+")
//   Token(type: NUMBER,  literal: "35")
</code>
</pre>
</details>

```rust
// The result is calculated from the tokens using the X/O/Y algorithm.
//
//  ╭────────╮ ╭───────────╮ ╭────────╮
//  │ NUMBER │ │ OPERATION │ │ NUMBER │
//  ╰────────╯ ╰───────────╯ ╰────────╯
//       ╰───╮       │        ╭───╯
//           ▼       ▼        ▼
//           X  [+, -, *, /]  Y
//
let result = Calculator::calculate(tokens, input.clone());
```

---

# How it Works
Mate primarily consists of two main structures: [Lexer] and [Calculator]. The [Lexer] structure is responsible for parsing and interpreting the given string expression, while the [Calculator] structure calculates the final result from the parsed tokens.

## Lexer
The Lexer iterates through the given input string, reading and converting each character into a [Token] structure. The main types of tokens include:
- `ILLEGAL` - Represents an illegal character.
- `NUMBER` - Represents a number.
- `MINUS`, `PLUS`, `PRODUCT`, `DIVIDE`, `PERCENTAGE`, `POWER` - Represents operations.
- `LPAREN`, `RPAREN` - Represents parentheses.
- `LABS`, `RABS` - Represents absolute values.
- `SUBEXP` - Represents sub-expressions, which are expressions inside parentheses, absolute values, or combinations of division and multiplication.

The Lexer's `lex` function converts each character into one of these tokens. It groups tokens related to multiplication or division operations into a single sub-expression to maintain the correct operation priority. It also nests parentheses, absolute values, and powers using a custom level-to-expression algorithm.

The level-to-expression algorithm is a mapping algorithm that maps a specific expression to its nesting level.

For example, given the token list `(2 + 5) : (5 - 9 / (8 - 5))`, the generated result will be:

<img width="500" alt="mate" src="https://user-images.githubusercontent.com/59066341/192025304-220c58eb-8bbe-4820-bd6a-5f18b5b5758b.png">

This approach ensures the correct operation priority is maintained.

## Calculator

The Calculator takes the parsed token list and calculates the final result. It uses a custom `X/O/Y` algorithm, also known as `X/OPERATION/Y`, where `X` and `Y` are numbers, and `O` is an operation. If `X` or `Y` cannot be obtained, they are taken as zero.
```
╭────────╮ ╭───────────╮ ╭────────╮
│ NUMBER │ │ OPERATION │ │ NUMBER │
╰────────╯ ╰───────────╯ ╰────────╯
     ╰───╮       │        ╭───╯
         ▼       ▼        ▼
         X  [+, -, *, /]  Y
```

---

# Syntax

The syntax of `Mate` is designed to be as simple and basic as possible. It follows the plain-text mathematics syntax with a few customizations. Here are some examples:

### Addition and Subtraction
`2 + 5` and `2 - 5` are valid syntaxes. (in `x <operation> y` where x and y could be `NUMBER` or `SUBEXP`).

### Multiplication, Division, and Percentage
`4 * 2`, `4 / 2`, and `4 % 2` are valid syntaxes. (in `x <operation> y` where x and y could be `NUMBER` or `SUBEXP`). Also, `4(10 / 2)` or `4(2)` are valid syntaxes for multiplication.

### Power
`4 ^ 2` is a valid syntax. (in `x <operation> y` where x and y could be `NUMBER` or `SUBEXP`). Continuous powers are also accepted: `4 ^ 2 ^ 3`, which will be automatically converted to `4 ^ (2 ^ 3)`.

### Parentheses
`(2 * 5)` is a valid syntax. (in `x <operation> y` where x and y could be `NUMBER` or `SUBEXP`). Nested parentheses expressions are also accepted: `(2 * ((5 * 2) / (9 - 5)))`.

### Absolute Values
`[2 - 12]` is a valid syntax. (in `x <operation> y` where x and y could be `NUMBER` or `SUBEXP`). Nested absolute-value expressions are also accepted: `[7 - 14] * [5 - 9 / [5 - 3]]`.

