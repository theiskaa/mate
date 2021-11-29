import 'package:mate/src/lexer.dart';
import 'package:mate/src/validators.dart';

// All token types.
enum Type {
  dot, // point/comma

  addition,
  subtraction,
  multiplication,
  division,
  percentage,

  leftPR, // Left parentheses
  rightPR, // Left parentheses

  number,
  subExpression,

  undefined // alternative to null.
}

/// Token is a main object class.
///
/// That is alternative form of expression's each charachter.
class Token {
  Type type;
  dynamic value;

  Token({required this.type, this.value}) {
    // Set value if its type is sign.
    if (type.isSign && value == null) value = type.value();
  }

  // Point signs
  static const dot = '.';

  // Operation signs.
  static const addition = '+', subtraction = '-';
  static const multiplication = '*', division = '/', percentage = '%';

  // Parentheses signs.
  static const leftPR = '(', rightPR = ')';

  // Variable tokens.
  static double number(double n) => n;
  static List<Token> subExpression(String exp) {
    return Lexer().parseSubExpression(exp);
  }
}

// Shortcut utilities for token's type.
extension TypeUtils on Type {
  // Looks if token type is sign or not.
  bool get isSign {
    final _signs = [
      Type.addition,
      Type.subtraction,
      Type.multiplication,
      Type.division,
      Type.percentage,
      Type.dot,
      Type.leftPR,
      Type.rightPR,
    ];

    return _signs.contains(this);
  }

  // Looks if type is sub expression token type.
  bool get isSubExpression => this == Type.subExpression;

  // Looks if type is number token type.
  bool get isNumber => this == Type.number;

  // Looks if type is parentheses sign token type.
  bool get isParenthesesSign => this == Type.leftPR || this == Type.rightPR;

  // Generates token value from token type.
  dynamic value([dynamic value]) {
    switch (this) {
      case Type.addition:
        return Token.addition;
      case Type.subtraction:
        return Token.subtraction;
      case Type.multiplication:
        return Token.multiplication;
      case Type.division:
        return Token.division;
      case Type.percentage:
        return Token.percentage;
      case Type.dot:
        return Token.dot;
      case Type.leftPR:
        return Token.leftPR;
      case Type.rightPR:
        return Token.rightPR;
      case Type.number:
        return Token.number(double.tryParse(value)!);
      case Type.subExpression:
        return Token.subExpression(value);
      default:
    }
  }
}

// Shortcut utilities for string expression.
extension StringUtils on String {
  // Converts string to Type, if type exists.
  Type? get toType {
    final values = {
      Token.addition: Type.addition,
      Token.subtraction: Type.subtraction,
      Token.multiplication: Type.multiplication,
      Token.division: Type.division,
      Token.percentage: Type.percentage,
      Token.dot: Type.dot,
      Token.leftPR: Type.leftPR,
      Token.rightPR: Type.rightPR,
    };

    if (length == 1 && !Validators.isNum(this)) {
      return values[this] ?? Type.undefined;
    }
    if (Validators.isNotCompletedPart(this)) return Type.subExpression;
    if (Validators.isNum(this)) return Type.number;
  }
}
