import 'package:mate/src/expression.dart';
import 'package:mate/src/validators.dart';

/// ### Lightweight parser library to parse and calculate string expressions.
///
/// Parsing and calculating:
/// ```dart
/// final String exp = '-2 + 5 + 10 * 2';
/// final result = ExpressionParser().calculate(exp); // --> 23
/// ```
///
/// You can also check localy if expression is invalid or not:
/// ```dart
/// final isInvalid = ExpressionParser().isInvalidExp(exp) // --> false.
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

  /// Looks and returns if provided expression is invalid or not. (for our library)
  bool isInvalidExp(String exp) {
    // A normal operation cannot be less than 3 length
    // The minimum size opeartion example: "2+2".
    if (exp.length < 3) return true;

    // Expression cannot include letters.
    if (Validators.letters.hasMatch(exp)) return true;

    // TODO: Should remove after resolving #3
    // Current version of parser haven't support for expressions with parentheses.
    // So, if expression contains any parentheses that means given expression is invalid.
    if (exp.contains('(') || exp.contains(')')) return true;

    // Looks if opeation starts with any invalid starter sign.
    // Division, Product and percentage signs is invalid to start with.
    var startsWithSign = exp.startsWith(Validators.multDivPer);

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
    // Trim white spaces, and replace point instead of commas.
    _trimedExp = op.replaceAll(' ', '').replaceAll(',', '.');

    String oneTimePart = '';
    for (var i = 0; i < _trimedExp!.length; i++) {
      final c = _trimedExp![i];

      if (oneTimePart.isEmpty && i != _trimedExp!.length - 1) {
        oneTimePart += c;
        continue;
      }

      // If "c" is number or point (dot/comma), add to current one time part.
      // For example if operation's a random part is "123"
      // It'll add "1" and then "2", then "3", So, by doing that
      // We'll understand that given char as "123".
      if (Validators.isNum(c) || Validators.isPoint(c)) oneTimePart += c;

      if (Validators.isPlusOrMinus(c) || Validators.isNotNummable(c)) {
        // If "c" is not convertable to number (isn't minus or plus), then should continue adding them to one time part.
        // Because, we cannot convert a string something like "*2" or "/2" to double.
        if (Validators.isNotNummable(c)) {
          oneTimePart += c;
          continue;
        }

        // If "c" is convertable to number (is minus or plus), that means we've to complete current one time part.
        // For example, in random case it'd be something like: "-2" or "+2".
        expression.parts.add(oneTimePart);
        oneTimePart = '';

        if (c != '+') oneTimePart += c;
      }

      // If we're at the end of the looping, add one time part to operation parts.
      if (i == _trimedExp!.length - 1) expression.parts.add(oneTimePart);
    }
  }
}
