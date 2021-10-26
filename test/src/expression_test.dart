import 'package:flutter_test/flutter_test.dart';
import 'package:mate/src/expression.dart';

void main() {
  late Expression expression;

  setUpAll(() {
    expression = Expression();
    expression.parts = ["2", "-2", "2*10"];
  });

  group("[Expression]", () {
    test('calculate should work properly', () {
      final res = expression.calculate();
      expect(res, 2 + (-2) + 2 * 10);
    });

    test('takeSum should work properly', () {
      final minusSum = expression.takeSum('-', 10, 2);
      final plusSum = expression.takeSum('+', 10, 2);
      final prodSum = expression.takeSum('*', 10, 2);
      final divSum = expression.takeSum('/', 10, 2);

      expect(minusSum, 8);
      expect(plusSum, 12);
      expect(prodSum, 20);
      expect(divSum, 5);
    });

    test('clear should work properly', () {
      expect(expression.parts.isEmpty, false);

      expression.clear();
      expect(expression.parts.isEmpty, true);
    });
  });
}
