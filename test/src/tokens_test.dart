import 'package:test/test.dart';
import 'package:mate/src/tokens.dart';

void main() {
  group('Tokens', () {
    test('should expect constant tokens as string values', () {
      final tokensTestData = {
        addition: "+",
        subtraction: "-",
        multiplication: "*",
        division: "/",
        percentage: "%",
        comma: ",",
        dot: ".",
        number(20): 20,
      };

      tokensTestData.forEach(
        (key, value) => expect(tokensTestData[key], value),
      );
    });
  });
}
