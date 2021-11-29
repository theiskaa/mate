import 'package:mate/mate.dart';
import 'package:test/test.dart';

void main() {
  late Mate mate, debugModeMate;

  setUpAll(() {
    mate = Mate();
    debugModeMate = Mate(debugMode: true);
  });

  group("[Mate]", () {
    test('isInvalidExp should work properly', () {
      final tests = {
        "-2ab*10*": true,
        "-2 + 5 + 10 * 2 - 2.5 + 50 % 2": false,
        "2+2": false,
      };

      tests.forEach(
        (exp, expected) => expect(mate.isInvalidExp(exp), expected),
      );
    });

    test('calculate should work properly', () {
      final tests = {
        "-2ab*10*": null,
        "-2 + 5 + 10 * 2 - 2.5 + 50 % 2": 21.5,
        "2+2*5": 12,
        "((((2+2)+(2.5+2.5))*((1.5 * 2)* (1.5 + 1.5))))": 81,
        "((((2.5 + 2.5) * ((1+1) + ((0.5*(1+1))+(0.5*2)))) / ((1*2) * (1*2)))) * 0.1":
            0.5,
      };

      tests.forEach((exp, expected) {
        final got = mate.calculate(exp);
        expect(got, expected);
      });
    });

    test('[DebugMode enabled] calculate should work properly', () {
      final tests = {"2*2": 4};

      tests.forEach((exp, expected) {
        final got = debugModeMate.calculate(exp);
        expect(got, expected);
      });
    });
  });
}
