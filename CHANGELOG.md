# Changelog

## v0.2.1

Added factorial operator support.

- Implemented factorial operator: `5!` returns `120`, `(2+3)!` returns `120`
- Factorial works in expressions: `5! + 10`, `2 * 5!`, `sqrt(5!)`
- Supports nested factorials: `(3!)!` returns `720`
- Proper error handling for negative numbers, non-integers, and overflow

- **Issue:** [#24](https://github.com/theiskaa/mate/issues/24)

---

## v0.2.0

Major update with math functions, improved error handling, and project restructure.

### New Features
- Added 10 math functions: `sqrt`, `sin`, `cos`, `tan`, `log`, `ln`, `exp`, `floor`, `ceil`, `round`
- Functions support nesting: `sqrt(floor(17))`, `sin(cos(0))`
- Functions work in expressions: `2 * sqrt(16) + 1`
- Improved REPL with help, quit, clear, and tokens commands
- Added CLI argument support (`-h`, `-v`, `-t`, direct expressions)

### Bug Fixes
- Fixed division by zero (now returns proper error)
- Fixed panic on mismatched parentheses
- Fixed operator precedence bug with percentage operations
- Replaced all unsafe `.unwrap()` calls with proper error handling
- Fixed UTF-8 character handling in error messages

### Code Quality
- Restructured project to single crate with lib and bin
- Removed unnecessary dependencies (`regex`, `substring`)
- Fixed all clippy warnings
- Expanded test coverage to 57 tests

---

## v0.1.4

Added error handling and absolute value expressions.

- Implemented high-level error handling with descriptive messages
- Added absolute value expression parsing: `[-5]` returns `5`
- **Pull Requests:** [#22](https://github.com/theiskaa/mate/pull/22), [#23](https://github.com/theiskaa/mate/pull/23)

---

## v0.1.3

Added power expressions.

- Implemented power operator: `5 ^ 2` returns `25`
- Supports chained powers: `2 ^ 3 ^ 2` evaluates as `2 ^ (3 ^ 2)`
- **Pull Request:** [#20](https://github.com/theiskaa/mate/pull/20)

---

## v0.1.2

Added implicit multiplication.

- Auto-append multiplication between adjacent numbers and sub-expressions
- Example: `4(9 + 5)(9 - 3)` returns `336`
- **Pull Request:** [#19](https://github.com/theiskaa/mate/pull/19)

---

## v0.1.1

Added percentage operator.

- Implemented percentage token: `50 % 5` returns `2.5` (5% of 50)
- Works with sub-expressions: `50 % 5 + (100 % 2.5)` returns `5`
- **Pull Request:** [#18](https://github.com/theiskaa/mate/pull/18)

---

## v0.1.0

Initial release.

- Basic arithmetic: addition, subtraction, multiplication, division
- Parentheses grouping with nesting support
- Example: `5 + (2 + (10 - 5)) * (5 - 9 / (8 - 5))` returns `19`
