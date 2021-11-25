library mate;

import 'src/lexer.dart';
import 'src/expression.dart';
import 'src/tokens.dart';
import 'src/validators.dart';

class Mate {
  /// Early created main lexer instance of mate.
  /// Used to convert(parse) string expression to tokens list.
  final Lexer lexer = Lexer();

  /// Early created main expression.
  /// Used to store parts(tokens) and then calculate final result.
  final Expression expression = Expression();

  /// Takes user-input string expression, parses it by using [Lexer],
  /// and then calculates final result with [Expression].
  double? calculate(String exp) {
    final List<Token> parts = lexer.parse(exp);

    if (isInvalidExp(parts)) return null;

    expression.parts = parts;
    return expression.calculate();
  }

  /// Looks and returns if provided expression is invalid or not. (for our library)
  bool isInvalidExp(List<Token> tokens) {
    if (tokens.length % 2 == 0) return true;

    // Looks if operation starts or ends with any invalid starter/ender sign.
    // Division, Product and percentage signs is invalid to start or end with.
    final startsWithSign = !Validators.isNummable(tokens[0].value.toString());

    final endsWithSign = !Validators.isNumOrPoint(
      tokens[tokens.length - 1].value.toString(),
    );

    if (tokens[0].type.isSign && (startsWithSign || endsWithSign)) {
      return true;
    }

    // Catch invalid tokens.
    var invalidValues = tokens.where(
      (t) => t.type == Type.undefined || t.value == null,
    );

    return invalidValues.isNotEmpty;
  }
}
