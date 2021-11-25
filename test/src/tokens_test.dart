import 'package:test/test.dart';
import 'package:mate/src/tokens.dart';

void main() {
  group('Token', () {
    test('should expect constant tokens as string values', () {
      final tests = {
        Token.addition: "+",
        Token.subtraction: "-",
        Token.multiplication: "*",
        Token.division: "/",
        Token.percentage: "%",
        Token.dot: ".",
        Token.number(20): 20,
        Token.subExpression('2*2'): [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(2)),
        ]
      };

      tests.forEach((key, value) => expect(tests[key], value));
    });
    test('should initilaze new token properly', () {
      final tests = {
        Type.addition: "+",
        Type.subtraction: "-",
        Type.multiplication: "*",
        Type.division: "/",
        Type.percentage: "%",
        Type.dot: ".",
      };

      tests.forEach((key, expected) {
        final token = Token(type: key);
        expect(token.value, expected);
      });
    });
  });

  group('TypeUtils', () {
    test('isSign should expect values correctly', () {
      final tests = {
        Type.addition: true,
        Type.subtraction: true,
        Type.multiplication: true,
        Type.division: true,
        Type.percentage: true,
        Type.dot: true,
        Type.number: false,
        Type.subExpression: false,
      };

      tests.forEach((key, expected) => expect(key.isSign, expected));
    });

    test('value should expect right value from type', () {
      final tests = {
        Type.addition: "+",
        Type.subtraction: "-",
        Type.multiplication: "*",
        Type.division: "/",
        Type.percentage: "%",
        Type.dot: ".",
        Type.number: Token.number(2),
        Type.subExpression: Token.subExpression('2*2'),
      };

      tests.forEach(
        (key, dynamic expected) {
          final values = {Type.number: '2', Type.subExpression: '2*2'};

          if (key != Type.subExpression) {
            expect(key.value(values[key]), expected);
          } else {
            expect(key.value(values[key]).length, expected.length);

            for (var i = 0; i < expected.length; i++) {
              expect(key.value(values[key])[i].type, expected[i].type);
              expect(key.value(values[key])[i].value, expected[i].value);
            }
          }
        },
      );
    });
  });

  group('StringUtils', () {
    test('toType should convert string charachter to token type', () {
      final tests = {
        "+": Type.addition,
        "-": Type.subtraction,
        "*": Type.multiplication,
        "/": Type.division,
        "%": Type.percentage,
        ".": Type.dot,
        "2": Type.number,
        "2*2": Type.subExpression,
      };

      tests.forEach((key, expected) => expect(key.toType, expected));
    });
  });
}
