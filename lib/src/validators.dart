class Validators {
  // Pattern to catch points [commas and points]
  static final points = RegExp(r"[,.]");

  static final numsSignsPoints = RegExp(r"[,.]*[+-]*[0-9]");

  // Patterns to catch nums and letters in operation.
  static final nums = RegExp(r"[0-9]"), letters = RegExp(r"[A-Za-z]");

  // Patterns to catch operation signs in operation.
  static final plusMinus = RegExp(r"[-+]"), multDiv = RegExp(r"[/*]");

  // Checks if given char is num or not. (by nums regular expression)
  static bool isNum(String c) => nums.hasMatch(c);

  // Checks if given char is plus or minus. (by plusMinus regular expression)
  static bool isPlusOrMinus(String c) => plusMinus.hasMatch(c);

  // Checks if given char is multiplication sign or division sign. (by multDiv regular expression)
  static bool isMultOrDiv(String c) => multDiv.hasMatch(c);

  // Check if given char is dot or comma. (by points regular expression)
  static bool isPoint(String c) => points.hasMatch(c);
}
