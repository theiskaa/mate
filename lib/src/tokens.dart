import 'package:mate/src/lexer.dart';
import 'package:mate/src/validators.dart';

// All token types.
enum Type {
  addition,
  subtraction,
  multiplication,
  division,
  percentage,

  dot,

  number,
  subExpression,

  undefined
}

// Token is a main object class that'd used as lexer's tokens.
class Token {
  Type type;
  dynamic value;

  Token({required this.type, this.value}) {
    // Set value if it's type sign and
    if (type.isSign && value == null) value = type.value();
  }

  // Point signs
  static const dot = '.';

  // Operation signs.
  static const addition = "+", subtraction = '-';
  static const multiplication = '*', division = '/', percentage = '%';

  // Variable tokens.
  static double number(double n) => n;
  static List<Token> subExpression(String exp) => Lexer().parseSubExpression(exp);
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
    ];

    return _signs.contains(this);
  }

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
    };

    if (length == 1 && !Validators.isNum(this)) {
      return values[this] ?? Type.undefined;
    }
    if (Validators.isNotCompletedPart(this)) return Type.subExpression;
    if (Validators.isNum(this)) return Type.number;
  }
}
