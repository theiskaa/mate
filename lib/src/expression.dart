import 'package:mate/src/tokens.dart';

/// Expression is a parsed variant of "string" expression.
///
/// Where, parsed (divided as parts) string expression stored in [parts] list.
///
/// [calculate] uses [parts] and [takeRes] to get final result of [Expression].
class Expression {
  // The parsed (divided as parts/tokens) expression list.
  List<Token> parts = [];

  /// Clears expression, by removing all parts of it.
  void clear() => parts.clear();

  // Loops through expression parts and takes sum of them.
  // By doing that we get final result of expression.
  double? calculate([List<Token>? tokens]) {
    double result = 0;

    final _parts = tokens ?? parts;

    // If length of parts is one, that means it has only sub expression.
    // So, we could directly calculate sub expression without looping.
    if (_parts.length == 1) return calculate(_parts[0].value);

    for (var i = 0; i <= _parts.length; i += 2) {
      final part = _parts[i];

      final y = part.type.isNumber ? part.value : calculate(part.value);
      final sign = i == 0 ? Token.addition : _parts[i - 1].value;

      result = takeRes(sign, result, y);
    }

    return result;
  }

  // Takes sign and two double values.
  // Makes appropriate operation by given sign and then returns result.
  double takeRes(String sign, double x, double y) {
    final operations = {
      "+": x + y,
      "-": x - y,
      "*": x * y,
      "/": x / y,
      "%": (x / 100) * y
    };

    return operations[sign] ?? 0;
  }
}
