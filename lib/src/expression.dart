/// Expression is a parsed variant of "string" expression.
///
/// That has: [parts], [calculate], and [takeSum] operations.
///
/// [calculate] uses [parts] and [takeSum] to get final result of [Expression].
class Expression {
  // The parsed nums of (pure) expression.
  List<String> parts = [];

  // Calculate, loops through expression parts and adds them to themself.
  // By doing that we get final result of expression parts.
  double? calculate() {
    double result = 0;

    for (var i = 0; i < parts.length; i++) {
      final part = parts[i];
      double? c;

      // If part contains division or product signs,
      // that means we have, not-completed part expression inside parts.
      if (part.contains('*') || part.contains('/')) {
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

  // Takes mini part expression from actual expression parts, and returns result of it.
  double _calcPart(String miniExp) {
    double res = 0;

    final _nums = miniExp.split(RegExp(r"[/*]"));
    final _operations = miniExp.split(RegExp(r"[+-]*[0-9]"));

    // TODO: Find better way to remove empty spaces.
    // Remove all blank strings from _operations.
    _operations.removeWhere((i) => i.isEmpty);

    for (var i = 0; i < _nums.length; i++) {
      double c = double.tryParse(_nums[i])!;

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
  double takeSum(String sign, double f, double s) {
    final operationSums = {"+": f + s, "-": f - s, "*": f * s, "/": f / s};
    return operationSums[sign] ?? 0;
  }
}
