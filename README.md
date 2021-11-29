<p align="center">
 <img width="350" src="https://user-images.githubusercontent.com/59066341/138941465-a4354274-3976-4571-bdcd-df031d7d4761.png" alt="Package Logo">
 <br>
 <a href="https://pub.dev/packages/mate">
  <img src="https://img.shields.io/pub/v/mate?color=blue" alt="pub version" />
 </a>
 <a href="https://github.com/theiskaa/mate/blob/main/LICENSE">
  <img src="https://img.shields.io/badge/License-MIT-red.svg" alt="License: MIT"/>
 </a>
</p>

## Documentation

Create new mate instance.
```dart
final Mate mate = Mate();
```

Then you can calculate your "string" expression like:
```dart
final String expression = "-2 + 5 + 10 * 2";
final double? result = mate.calculate(expression); // --> 23
```

When we call calculate, it checks the validness of expression automatically.
So, we needn't to do some manual checking here. (If expression is invalid, then result would be null)

But in anyways, if you wanna check the validness of expression manually, you can do it, like:
```dart
final bool isInvalid = mate.isInvalidExp(exp);
```

---

> **Check [official example](https://github.com/theiskaa/mate/blob/main/example/main.dart) of Mate**

> **Check [official UI implementation example](https://github.com/theiskaa/mate/blob/main/example/app.dart) of Mate**

---

## Explanation
Mate's **parsing**/**lexing** algorithm looks like an **interpreter**. <br>
It has early created constant chars, and uses **lexer**/**parser** to convert string expression to tokens list.

**Token** is a special object model, that has **type** and **value**. It is a char's, library implementation variant.

So as you guess, `Mate`'s `calculate` function, takes string expression, parses it by using `Lexer`,
and by using lexer's parsing result, it calculates final result.
However, we pass lexer's parsing result to `Expression`'s parts, and then we call calculate function from `Expression` to get final result.
#### Let's see an example:
Our expression is `"2 + 2 * 5"`, that expression would be passed to `Mate`'s `calculate` function.
Then, it'd call `Lexer`'s `parse` function to convert expression to `List<Token>`.

**Our expression's, parsed variant would look like:**
```dart
[
  Token(type: Type.number, value: Token.number(2)),
  Token(type: Type.addition),
  Token(type: Type.subExpression, value: [
    Token(type: Type.number, value: Token.number(2)),
    Token(type: Type.multiplication),
    Token(type: Type.number, value: Token.number(5)),
  ])
]
```
Then, by using that result, `Expression` can calculate final result. --> `2 + (2*5) = 2 + 10 = 12`.

Also parentheses are sub expressions, and by adding parentheses parsing support, sub expression also can include another sub expression inside it. (We call it nested expression).
#### Let's see an example of parentheses:
When our expression is `((20 / 4) * (20 / 5)) + 1`, then parsed variant of it, would look like:
```dart
[
  Token(type: Type.subExpression, value: [
    Token(type: Type.subExpression, value: [
      Token(type: Type.number, value: Token.number(20)),
      Token(type: Type.division),
      Token(type: Type.number, value: Token.number(4)),
    ]),
    Token(type: Type.multiplication),
    Token(type: Type.subExpression, value: [
      Token(type: Type.number, value: Token.number(20)),
      Token(type: Type.division),
      Token(type: Type.number, value: Token.number(5)),
    ]),
  ]),
  Token(type: Type.addition),
  Token(type: Type.number, value: Token.number(1)),
]
```
