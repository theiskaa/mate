import 'package:mate/src/expression.dart';
import 'package:mate/src/tokens.dart';
import 'package:test/test.dart';

void main() {
  late Expression expression;

  setUpAll(() => expression = Expression());

  group("[Expression]", () {
    test('calculate should work properly', () {
      final tests = {
        5 + 5 - 5 + 10: [
          Token(type: Type.number, value: Token.number(5)),
          Token(type: Type.addition),
          Token(type: Type.number, value: Token.number(5)),
          Token(type: Type.subtraction),
          Token(type: Type.number, value: Token.number(5)),
          Token(type: Type.addition),
          Token(type: Type.number, value: Token.number(10)),
        ],
        5 * 2 + 3 * 5 - 10 / 2: [
          Token(type: Type.subExpression, value: [
            Token(type: Type.number, value: Token.number(5)),
            Token(type: Type.multiplication),
            Token(type: Type.number, value: Token.number(2)),
          ]),
          Token(type: Type.addition),
          Token(type: Type.subExpression, value: [
            Token(type: Type.number, value: Token.number(3)),
            Token(type: Type.multiplication),
            Token(type: Type.number, value: Token.number(5)),
          ]),
          Token(type: Type.subtraction),
          Token(type: Type.subExpression, value: [
            Token(type: Type.number, value: Token.number(10)),
            Token(type: Type.division),
            Token(type: Type.number, value: Token.number(2)),
          ]),
        ],
        (10 / 100) * 2: [
          Token(type: Type.subExpression, value: [
            Token(type: Type.number, value: Token.number(10)),
            Token(type: Type.percentage),
            Token(type: Type.number, value: Token.number(2)),
          ]),
        ],
      };

      tests.forEach((expected, parts) {
        expression.parts = parts;
        final res = expression.calculate();

        expect(res, expected);
      });
    });

    test('takeRes should work properly', () {
      final minusSum = expression.takeRes('-', 10, 2);
      final plusSum = expression.takeRes('+', 10, 2);
      final prodSum = expression.takeRes('*', 10, 2);
      final divSum = expression.takeRes('/', 10, 2);
      final perSum = expression.takeRes('%', 10, 2);

      expect(minusSum, 8);
      expect(plusSum, 12);
      expect(prodSum, 20);
      expect(divSum, 5);
      expect(perSum, 0.2);
    });

    test('clear should work properly', () {
      expect(expression.parts.isEmpty, false);

      expression.clear();
      expect(expression.parts.isEmpty, true);
    });
  });
}
