class Validators {
  // Pattern to catch points [commas and points]
  static final points = RegExp(r"[,.]");

  static final numsSignsPoints = RegExp(r"[,.]*[+-]*[0-9]");

  // Patterns to catch nums and letters in operation.
  static final nums = RegExp(r"[0-9]"), letters = RegExp(r"[A-Za-z]");

  // Patterns to catch operation signs in operation.
  static final plusMinus = RegExp(r"[-+]"), multDiv = RegExp(r"[/*]");

  static bool isNum(String c) => nums.hasMatch(c);
  static bool isPlusOrMinus(String c) => plusMinus.hasMatch(c);
  static bool isMultOrDiv(String c) => multDiv.hasMatch(c);
  static bool isPoint(String c) => points.hasMatch(c);
}
