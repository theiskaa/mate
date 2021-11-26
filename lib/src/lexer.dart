import 'package:mate/src/tokens.dart';

import 'validators.dart';

/// #### Lexer is a simple expression parser.
///
/// It takes string expression, scans it and then generates tokens from expression.
/// For example:
/// if expression is `"2+2"`, then [Lexer]'s output will look something like:
/// ```dart
/// [
///   Token(Type.number, number(2)),
///   Token(Type.addition, "+"),
///   Token(Type.number, number(2)),
/// ];
/// ```
///
/// Here is an another example:
///
/// if expression is `"2+2*5"` then [Lexer]'s output will look something like:
/// ```dart
/// [
///   Token(type: Type.number, value: Token.number(2)),
///   Token(type: Type.addition),
///   Token(
///     type: Type.subExpression,
///     value: [
///       Token(type: Type.number, value: Token.number(2)),
///       Token(type: Type.multiplication),
///       Token(type: Type.number, value: Token.number(5)),
///     ],
///   ),
/// ]
/// ```
/// We have `subExpression` typed token in output, because we should re-calculate it and then
/// take sum of result and number. So when we calculate subExpression, we'd get `10` right?
/// So, final equation would be: `2 + 10` = `12`.
class Lexer {
  // Parse, is the main parsing function of lexer, it usually used to parse
  // not-modified user input expression.
  //
  // To parse sub expression, we use [parseSubExpression]
  List<Token> parse(String expression) {
    var tokens = <Token>[];

    // Trim white spaces, and replace point instead of commas.
    expression = expression.replaceAll(' ', '').replaceAll(',', '.');

    String oneTime = '';

    // Adds stored and not empty oneTime (usually numbers and sub expressions) to tokens list.
    finish() {
      if (oneTime.isEmpty) return;

      tokens.add(Token(
        type: oneTime.toType!,
        value: oneTime.toType?.value(oneTime),
      ));
      oneTime = '';
    }

    for (var i = 0; i < expression.length; i++) {
      final c = expression[i];

      if (!Validators.isNum(c) && !Validators.isPoint(c)) {
        // If c is not convert able sign (+ or -), we should keep adding on `oneTime`
        // 2+2*5 --> 2, (2*5) is full oneTime that it's contains not convert able number.
        if (!Validators.isNummable(c) || i == 0) {
          oneTime += c;
          continue;
        }

        // Finish current one time before adding new token.
        if (oneTime.isNotEmpty) finish();

        final token = Token(type: c.toType!, value: c.toType?.value());
        tokens.add(token);
        continue;
      }

      // Should keep store/cache numbers in onTime, to get full number.
      // So, if expression is "22+8", to get full 22 we should store 2 and then add 2 again to the oneTime.
      // Then we can convert oneTime("22") to num value.
      if (oneTime.isEmpty || Validators.isNum(c) || Validators.isPoint(c)) {
        oneTime += c;
      }

      if (i == expression.length - 1) finish();
    }

    return tokens;
  }

  // Basically used to parse(convert string expression to tokens list) sub expressions.
  // It has it's own algorithm to parse sub string expression, So, it doesn't uses algorithm of default parsing function.
  List<Token> parseSubExpression(String subExp) {
    var tokens = <Token>[];

    String oneTime = '';

    // Adds stored and not empty oneTime (usually numbers) to tokens list.
    addOneTime() {
      if (oneTime.isEmpty) return;

      tokens.add(Token(
        type: oneTime.toType!,
        value: oneTime.toType?.value(oneTime),
      ));
      oneTime = '';
    }

    for (var i = 0; i < subExp.length; i++) {
      final c = subExp[i];

      // Adds any sign as independent token.
      if (!Validators.isNum(c) && !Validators.isPoint(c)) {
        addOneTime();
        tokens.add(Token(type: c.toType!));
      }

      // Should keep store/cache numbers in onTime, to get full number.
      // So, if sub expression is "22*5", to get full 22 we should store 2 and then add 2 again to the oneTime.
      if (Validators.isNum(c) || Validators.isPoint(c)) oneTime += c;

      if (i == subExp.length - 1) addOneTime();
    }

    return tokens;
  }
}
