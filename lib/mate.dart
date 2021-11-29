library mate;

import 'src/lexer.dart';
import 'src/expression.dart';
import 'src/tokens.dart';
import 'src/validators.dart';
import 'src/dev_utils.dart';

/// Mate is main class of library.
///
/// It has already created, [Lexer] and [Expression].
/// And two functions, [calculate] and [isInvalidExp].
///
/// [calculate] is the main function that takes string expression,
/// parses it (converts to tokens) by [Lexer], and then by using [Expression],
/// calculates final result.
///
/// [calculate] function automatically checks given string expression's validness.
/// if it's invalid, then function will return `null`, otherwise as normal a double number value.
class Mate {
  /// Decides running mode of mate.
  /// As default it's `true`, (adapted to release mode).
  ///
  /// To run mate on debug mode (development mode), make [debugMode] true.
  /// It will log parsed expression's token tree, in each call.
  final bool debugMode;

  Mate({this.debugMode = false});

  /// Early created main lexer instance of mate.
  /// Used to convert(parse) string expression to tokens list.
  final Lexer lexer = Lexer();

  /// Early created main expression.
  /// Used to store parts(tokens) and then calculate final result.
  final Expression expression = Expression();

  /// Looks and returns if provided expression is invalid or not. (for our library)
  bool isInvalidExp(String exp) => !Validators.isValidExpression(exp);

  /// Takes user-input string expression, parses it by using [Lexer],
  /// and then calculates final result with [Expression].
  double? calculate(String exp) {
    if (isInvalidExp(exp)) return null;

    final List<Token> parts = lexer.parse(exp);
    expression.parts = parts;

    // Log tokens/parts tree, if debug mode was enabled.
    if (debugMode) logTree(parts);

    return expression.calculate();
  }
}
