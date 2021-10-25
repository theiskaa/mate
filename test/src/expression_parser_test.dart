import 'package:flutter_test/flutter_test.dart';
import 'package:mate/mate.dart';

void main() {
  late ExpressionParser parser;

  const invalidExpression = "-2*10*";
  const expression = "-2 + 5 + 10 * 2";

  setUpAll(() {
    parser = ExpressionParser();
  });

  group("[ExpressionParser]", () {
    test('isInvalidOperation should work properly', () {
      expect(parser.isInvalidOperation(invalidExpression), true);
      expect(parser.isInvalidOperation(expression), false);
    });
    test('calculate should work properly', () {
      final invalidExpResult = parser.calculate(invalidExpression);

      expect(invalidExpResult, null);
      expect(parser.expression.parts, []);

      final res = parser.calculate(expression);

      expect(res, 23);
      expect(parser.expression.parts, ["-2", "+5", "+10*2"]);
    });
  });
}
