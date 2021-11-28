import 'package:mate/mate.dart';
import 'package:mate/src/lexer.dart';
import 'package:test/test.dart';

void main() {
  late Lexer lexer;
  late Mate mate, debugModeMate;

  setUpAll(() {
    lexer = Lexer();
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
      tests.forEach((exp, expected) {
        final tokens = lexer.parse(exp);
        expect(mate.isInvalidExp(tokens), expected);
      });
    });

    test('calculate should work properly', () {
      final tests = {
        "-2ab*10*": null,
        "-2 + 5 + 10 * 2 - 2.5 + 50 % 2": 21.5,
        "2+2*5": 12,
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
