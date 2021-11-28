class Validators {
  // Pattern to catch points (commas and dots).
  static final points = RegExp(r"[,.]");

  // Patterns to catch nums and letters in operation.
  static final nums = RegExp(r"[0-9]"), letters = RegExp(r"[A-Za-z]");

  // Patterns to catch operation signs in expression.
  static final plusMinus = RegExp(r"[-+]"), multDivPer = RegExp(r"[/*%]");

  // Pattern to catch parentheses.
  static final parentheses = RegExp(r"[()]");

  // Checks if given char is num or not.
  static bool isNum(String c) => nums.hasMatch(c);

  // Checks if given char is plus or minus.
  static bool isPlusOrMinus(String c) => plusMinus.hasMatch(c);

  // Checks if given char is multiplication, division or percentage sign.
  static bool isNummable(String c) => !multDivPer.hasMatch(c);

  // Checks if given char is dot or comma.
  static bool isPoint(String c) => points.hasMatch(c);

  // Checks if given part is completed or not.
  static bool isNotCompletedPart(String p) {
    return multDivPer.hasMatch(p) || p.contains('+') || parentheses.hasMatch(p);
  }

  // Checks if given char is num or not.
  static bool isNumOrPoint(String c) => nums.hasMatch(c) || points.hasMatch(c);

  // Checks if given char is parentheses or not.
  static bool isPr(String c) => parentheses.hasMatch(c);

  // Checks if given char is opening parentheses sign or not.
  static bool isOpeningPr(String c) => c == '(';

  // Checks if given char is closing parentheses sign or not.
  static bool isClosingPr(String c) => c == ')';

  // Checks if given expression was nested correctly.
  // `((20/4) * (20/5))` is nested correctly, but ((20/4) * (20/5) isn't.
  static bool nestedCorrectly(String exp) {
    if (!exp.contains(parentheses)) return true;

    String listed = '';
    int openingPr = 0, closingPr = 0;

    for (var i = 0; i < exp.length; i++) {
      final c = exp[i];

      if (isPr(c)) listed += c;
      if (isOpeningPr(c)) openingPr++;
      if (isClosingPr(c)) closingPr++;
    }

    final listedCorrectly =
        isOpeningPr(listed[0]) && isClosingPr(listed[listed.length - 1]);

    return listedCorrectly && openingPr == closingPr;
  }
}
