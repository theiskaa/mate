import 'validators.dart';

/// Expression is a parsed variant of "string" expression.
///
/// Where, parsed (divided as parts) string expression stored in [parts] list.
///
/// [calculate] uses [parts] and [takeSum] to get final result of [Expression].
class Expression {
  // The parsed (divided as parts) expression.
  List<String> parts = [];

  // Loops through expression parts and takes sum of them.
  // By doing that we get final result of expression.
  double? calculate() {
    double result = 0;

    for (var i = 0; i < parts.length; i++) {
      final part = parts[i];
      double? c;

      if (Validators.isNotCompletedPart(part)) {
        c = _calcPart(part);
      } else {
        c = double.tryParse(part);
      }

      // If "c" is null, that means something went wrong on calculating.
      // Then, we return null, which means operation is invalid.
      if (c == null) return null;

      if (i == 0) {
        result = c;
        continue;
      }

      result = c + result;
    }

    return result;
  }

  // Takes not completed part expression from current expression parts,
  // Then re-parses and re-calculates it, and then returns result.
  double _calcPart(String miniExp) {
    double res = 0;

    final _nums = miniExp.split(Validators.multDivPer);
    final _operations = miniExp.split(Validators.numsSignsPoints);

    // Remove all blank strings from _operations.
    _operations.removeWhere((i) => i.isEmpty);

    for (var i = 0; i < _nums.length; i++) {
      double? c = double.tryParse(_nums[i])!;

      if (i == 0) {
        res = c;
        continue;
      }

      res = takeSum(_operations[i - 1], res, c);
    }

    return res;
  }

  // Takes sign and two double values.
  // Makes appropriate operation by given sign and then returns result.
  double takeSum(String sign, double x, double y) {
    final operationSums = {
      "+": x + y,
      "-": x - y,
      "*": x * y,
      "/": x / y,
      "%": (x / 100) * y
    };

    return operationSums[sign] ?? 0;
  }

  /// Clears expression, by removing all parts of it.
  void clear() => parts.clear();
}
