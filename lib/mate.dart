library mate;

import 'src/lexer.dart';
import 'src/expression.dart';
import 'src/tokens.dart';
import 'src/validators.dart';

class Mate {
  /// Keeps adding current result to last result.
  ///
  /// For example: If you calculate "2+8" first time then you'd get 10
  /// And then if you make new operation something like: "2+5". Then result would be:
  /// Last result + current result. So, 2+8+2+5 = 10+7 = 17.
  final bool keepAddingOn;
  Mate({this.keepAddingOn = false});

  /// Early created main lexer instance of mate.
  /// Used to convert(parse) string expression to tokens list.
  final Lexer lexer = Lexer();

  /// Early created main expression.
  /// Used to store parts(tokens) and then calculate final result.
  final Expression expression = Expression();

  /// Takes user-input string expression, parses it by using [Lexer],
  /// and then calculates final result with [Expression].
  double? calculate(String exp) {
    if (!keepAddingOn) expression.clear();

    final List<Token> parts = lexer.parse(exp);

    if (isInvalidExp(parts)) return null;

    expression.parts = parts;

    return expression.calculate();
  }

  bool isInvalidExp(List<Token> tokens) => (tokens.length % 2) != 0;

  /// Looks and returns if provided expression is invalid or not. (for our library)
  bool isInvalidExpAsString(String exp) {
    // A normal operation cannot be less than 3 length
    // The minimum size operation example: "2+2".
    if (exp.length < 3) return true;

    // Expression cannot include letters.
    if (Validators.letters.hasMatch(exp)) return true;

    // TODO: Should remove after resolving #3
    // Current version of parser haven't support for expressions with parentheses.
    // So, if expression contains any parentheses that means given expression is invalid.
    if (exp.contains('(') || exp.contains(')')) return true;

    // Looks if operation starts with any invalid starter sign.
    // Division, Product and percentage signs is invalid to start with.
    var startsWithSign = exp.startsWith(Validators.multDivPer);

    // Looks if operation ends with any invalid sign.
    // Each sign is invalid to end with.
    final last = exp[exp.length - 1];
    var endsWithSign = last == '+' || last == '-' || last == '/' || last == '*';

    return startsWithSign || endsWithSign;
  }
}
