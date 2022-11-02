# v0.1.4
The fifth version of mate.
1. Added high-level error handling and errors: [see in docs](https://github.com/theiskaa/mate#errors)
2. Implemented functionality to parse and calculate absolute-value expressions.
 
- **Pull Requests:** [#22](https://github.com/theiskaa/mate/pull/22) & [#23](https://github.com/theiskaa/mate/pull/23)

- **H-E that versions has ability to solve**: `[ 2 - 12 ] / 2` -> *`5`*

# v0.1.3
The fourth version of mate.
Implemented functionality to parse and calculate power expressions.

- **Pull Request:** [#20](https://github.com/theiskaa/mate/pull/20)

- **H-E that versions has ability to solve**: `5 ^ 2` -> *`25`*

# v0.1.2
The third version of mate.
Implemented functionality to understood and auto-append multiplication between two number(normal number & sub-expression) tokens.

- **Pull Request:** [#19](https://github.com/theiskaa/mate/pull/19)

> **H-E that version has ability to solve**: `4(9 + 5)(9 - 3)` -> *`336`*

# v0.1.1
The second version of mate.
Implemented the new percentage token. Which means besides of addition, subtraction, division, and multiplication. Now, it can parse and calculate percentage expressions.

- **Pull Request:** [#18](https://github.com/theiskaa/mate/pull/18)

> **H-E that version has ability to solve**: `50 % 5 + (100 % 2.5)` -> *`5`*

# v0.1.0

The very first version of mate.
Has ability to parse simple arithmetic expressions(addition, subtraction, division, and multiplication) + nested expressions with parentheses.

> **H-E that version has ability to solve**: `5 + (2 + (10 - (2.5 + 2.5))) * (5 - 9 / (8 - (2.5 * 2)))` -> *`19`*
