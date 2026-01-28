# mate

[![Crates.io](https://img.shields.io/crates/v/mate-rs)](https://crates.io/crates/mate-rs)
[![License](https://img.shields.io/crates/l/mate-rs)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/mate-rs)](https://crates.io/crates/mate-rs)
[![GitHub Stars](https://img.shields.io/github/stars/theiskaa/mate)](https://github.com/theiskaa/mate/stargazers)

A simple and lightweight arithmetic expression interpreter written in Rust. Mate parses string expressions using a lexer that generates a token tree, then calculates results using an X/O/Y algorithm. Supports basic arithmetic, parentheses, absolute values, powers, percentages, and mathematical functions.

Both binary and library are provided. The binary offers an interactive REPL and direct expression evaluation. The library enables programmatic calculation with full access to the lexer and calculator components.

## Install binary
Install the binary globally:

```bash
cargo install mate-rs
```

For the latest git version:

```bash
cargo install --git https://github.com/theiskaa/mate
```

## Install as library

Add to your project:

```bash
cargo add mate-rs
```

Or add to your Cargo.toml:

```toml
mate-rs = "0.2.0"
```

## Usage

### Command Line

Evaluate expressions directly:

```bash
mate 2 + 2
mate "(5 + 3) * 2"
mate "sqrt(16) + sin(3.14159 / 2)"
```

Start interactive REPL:

```bash
mate
```

Show parsed tokens with `-t` flag:

```bash
mate -t "10 / 2"
```

### Library Usage

Using the `Mate` wrapper for simple calculations:

```rust
use mate_rs::mate::Mate;

let result = Mate::calculate("6 * 7");
match result {
    Ok(v) => assert_eq!(v, 42.0),
    Err(e) => println!("Error: {}", e),
};
```

Using `Lexer` and `Calculator` separately:

```rust
use mate_rs::{calculator::Calculator, lexer::Lexer};

let input = "[ (2 + 5) * (5 - 9 + (8 - 5)) ] + 35";
let tokens = Lexer::lex(input).unwrap();
let result = Calculator::calculate(tokens, input);
```

## Supported Operations

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `2 + 3` |
| `-` | Subtraction | `5 - 2` |
| `*` | Multiplication | `4 * 3` |
| `/` | Division | `10 / 2` |
| `%` | Percentage | `50 % 10` (10% of 50 = 5) |
| `^` | Power | `2 ^ 3` (= 8) |
| `()` | Parentheses | `(2 + 3) * 4` |
| `[]` | Absolute value | `[-5]` (= 5) |

## Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sqrt(x)` | Square root | `sqrt(16)` (= 4) |
| `sin(x)` | Sine (radians) | `sin(3.14159 / 2)` |
| `cos(x)` | Cosine (radians) | `cos(0)` (= 1) |
| `tan(x)` | Tangent (radians) | `tan(0.785)` |
| `log(x)` | Base-10 logarithm | `log(100)` (= 2) |
| `ln(x)` | Natural logarithm | `ln(2.718)` |
| `exp(x)` | Exponential (e^x) | `exp(1)` |
| `floor(x)` | Round down | `floor(3.7)` (= 3) |
| `ceil(x)` | Round up | `ceil(3.2)` (= 4) |
| `round(x)` | Round to nearest | `round(3.5)` (= 4) |

Functions can be nested: `sqrt(floor(17))`, `sin(cos(0))`, `2 * sqrt(16) + 1`

## How it Works

### Lexer

The Lexer iterates through the input string, converting each character into tokens. It groups tokens related to multiplication or division operations into a single sub-expression to maintain correct operation priority. It also nests parentheses, absolute values, and powers using a level-to-expression algorithm.

The level-to-expression algorithm maps each expression to its nesting level. For example, given the token list `(2 + 5) * (5 - 9 / (8 - 5))`:

<img width="500" alt="mate" src="https://user-images.githubusercontent.com/59066341/192025304-220c58eb-8bbe-4820-bd6a-5f18b5b5758b.png">

This approach ensures correct operation priority is maintained.

### Calculator

The Calculator uses an X/O/Y algorithm where X and Y are numbers and O is an operation:

```
╭────────╮ ╭───────────╮ ╭────────╮
│ NUMBER │ │ OPERATION │ │ NUMBER │
╰────────╯ ╰───────────╯ ╰────────╯
     ╰───╮       │        ╭───╯
         ▼       ▼        ▼
         X  [+, -, *, /]  Y
```

Sub-expressions are recursively evaluated before the main calculation.

## Contributing

For information regarding contributions, please refer to [CONTRIBUTING.md](CONTRIBUTING.md) file.
