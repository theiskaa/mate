import 'package:flutter_test/flutter_test.dart';
import 'package:mate/mate.dart';

void main() {
  late ExpressionParser parser;
  late ExpressionParser parserForKeepAddingOn;

  const invalidExpression = "-2*10*";
  const expression = "-2 + 5 + 10 * 2";

  setUpAll(() {
    parser = ExpressionParser();
    parserForKeepAddingOn = ExpressionParser(keepAddingOn: true);
  });

  group("[ExpressionParser]", () {
    test('isInvalidExp should work properly', () {
      expect(parser.isInvalidExp(invalidExpression), true);
      expect(parser.isInvalidExp(expression), false);
    });
    test('calculate should work properly', () {
      final invalidExpResult = parser.calculate(invalidExpression);

      expect(invalidExpResult, null);
      expect(parser.expression.parts, []);

      final res = parser.calculate(expression);

      expect(res, 23);
      expect(parser.expression.parts, ["-2", "+5", "+10*2"]);
    });

    test('calculating with enabled `keepAddingOn` should work properly', () {
      final firstRes = parserForKeepAddingOn.calculate(expression);
      expect(firstRes, 23);

      final secRes = parserForKeepAddingOn.calculate(expression);
      expect(secRes, firstRes! + firstRes);

      final thirdRes = parserForKeepAddingOn.calculate(expression);
      expect(thirdRes, secRes! + firstRes);
    });
  });
}
