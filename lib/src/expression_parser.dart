import 'package:mate/src/expression.dart';

/// ### Lightweight parser library to parse and calculate string expressions.
///
/// Parsing and calculating:
/// ```dart
/// final String exp = '-2 + 5 + 10 * 2';
/// final result = ExpressionParser().calculate(exp); // result is 23
/// ```
///
/// You can also check localy if expression is invalid or not:
/// ```dart
/// final isInvalid = ExpressionParser().isInvalidExp(exp) // result is [false].
/// ```
class ExpressionParser {
  /// Keeps adding current result to last result.
  ///
  /// For example: If you calculate "2+8" first time then you'd get 10
  /// And then if you make new operation something like: "2+5". Then result would be:
  /// Last result + current result. So, 2+8+2+5 = 10+7 = 17.
  final bool keepAddingOn;
  ExpressionParser({this.keepAddingOn = false});

  // Takes trimed (cleared by empty spaces) expression.
  String? _trimedExp;

  // Early created main expression.
  // Used to store parts and then calculate final result.
  Expression expression = Expression();

  // Patterns to catch nums and letters in operation.
  final _numsRegEx = RegExp(r"[0-9]"), _lettersRegEx = RegExp(r"[A-Za-z]");

  // Patterns to catch operation signs in operation.
  final _plusMinusRegEx = RegExp(r"[-+]"), _multDivRegEx = RegExp(r"[/*]");

  /// Looks and returns if provided expression is invalid or not. (for our library)
  bool isInvalidExp(String exp) {
    // A normal operation cannot be less than 3 length
    // The minimum size opeartion example: "2+2".
    if (exp.length < 3) return true;

    // Expression cannot include letters.
    if (_lettersRegEx.hasMatch(exp)) return true;

    // TODO: Should remove after resolving #3
    // Current version of parser haven't support for expressions with parentheses.
    // So, if expression contains any parentheses that means given expression is invalid.
    if (exp.contains('(') || exp.contains(')')) return true;

    // Looks if opeation starts with any invalid starter sign.
    // Divider and multiplicater sign is invalid to start with.
    var startsWithSign = exp.startsWith(_multDivRegEx);

    // Looks if operation ends with any invalid sign.
    // Each sign is invalid to end with.
    final last = exp[exp.length - 1];
    var endsWithSign = last == '+' || last == '-' || last == '/' || last == '*';

    return startsWithSign || endsWithSign;
  }

  /// Takes "string" operation, parses it and then calls "calculate" from parsed expression.
  /// So, as a result it returns the result of given "string" operation.
  double? calculate(String exp) {
    if (isInvalidExp(exp)) return null;

    if (!keepAddingOn) expression.clear();

    _parse(exp);
    return expression.calculate();
  }

  /// Takes operation directly from input, parses it by trimming empty spaces.
  /// Then divides operation as parts to make calculation easy and understanable.
  void _parse(String op) {
    _trimedExp = op.replaceAll(' ', '');

    String oneTimePart = '';

    // Divide operation as parts.
    for (var i = 0; i < _trimedExp!.length; i++) {
      final c = _trimedExp![i];

      final isNum = _numsRegEx.hasMatch(c);
      final isPlusOrMinus = _plusMinusRegEx.hasMatch(c);
      final isMultOrDiv = _multDivRegEx.hasMatch(c);

      // If current one time part is empty,
      // should add directly - without checking type of "c".
      if (oneTimePart.isEmpty && i != _trimedExp!.length - 1) {
        oneTimePart += c;
        continue;
      }

      // If "c" is number, add to current one time part.
      // For example if operation's a random part is "123"
      // It'll add "1" and then "2", then "3", So, by doing that
      // We'll understand that given char as "123".
      if (isNum) oneTimePart += c;

      if (isPlusOrMinus || isMultOrDiv) {
        // If "c" is multiplication or division sign, then should continue adding themto one time part.
        // Because, we cannot convert a string something like "*2" or "/2" to double.
        if (isMultOrDiv) {
          oneTimePart += c;
          continue;
        }

        // If "c" isn't multiplication or division sign, that means we've to complete current one time part.
        // For example, in random case it'd be something like: "-2" or "+2".
        expression.parts.add(oneTimePart);
        oneTimePart = '';

        oneTimePart += c;
      }

      // If we're at the end of the looping, add one time part to operation parts.
      if (i == _trimedExp!.length - 1) expression.parts.add(oneTimePart);
    }
  }
}
