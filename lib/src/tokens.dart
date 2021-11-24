// All tokens as identifier.
enum Type {
  addition,
  subtraction,
  multiplication,
  division,
  percentage,
  comma,
  dot,
  number,

  undefined
}

// Token is a main object class that'd used as lexer's tokens.
class Token {
    Type type;
    dynamic value;

    Token(this.type, this.value);
}

// Variable tokens.
double number(double n) => n;

// Operation signs.
const addition = "+";
const subtraction = '-';
const multiplication = '*';
const division = '/';
const percentage = '%';

// Point signs
const comma = ',';
const dot = '.';
