class Validators {
  // Pattern to catch points (commas and dots).
  static final points = RegExp(r"[,.]");

  // Patterns to catch nums and letters in operation.
  static final nums = RegExp(r"[0-9]"), letters = RegExp(r"[A-Za-z]");

  // Patterns to catch operation signs in expression.
  static final plusMinus = RegExp(r"[-+]"), multDivPer = RegExp(r"[/*%]");

  // Checks if given char is num or not.
  static bool isNum(String c) => nums.hasMatch(c);

  // Checks if given char is plus or minus.
  static bool isPlusOrMinus(String c) => plusMinus.hasMatch(c);

  // Checks if given char is multiplication, division or percentage sign.
  static bool isNotNummable(String c) => multDivPer.hasMatch(c);

  // Checks if given char is dot or comma.
  static bool isPoint(String c) => points.hasMatch(c);

  // Checks if given part is completed or not.
  static bool isNotCompletedPart(String p) {
    return p.contains('*') || p.contains('/') || p.contains('%');
  }
}
