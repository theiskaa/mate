import 'package:mate/src/tokens.dart';
import 'package:mate/src/dev_utils.dart';
import 'package:test/test.dart';

void main() {
  group('[Developer Utilities]', () {
    test('logTree should work correctly', () {
      final testData = [
        Token(type: Type.subExpression, value: [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(2)),
        ]),
        Token(type: Type.division),
        Token(type: Type.number, value: Token.number(2)),
      ];

      logTree(testData);
    });

    test('tokenToJsonTree should work correctly', () {
      final tests = {
        Token(type: Type.subExpression, value: [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(2)),
        ]): {
          "type": "Type.subExpression",
          "value": [
            {
              "type": "Type.number",
              "value": 2.0,
            },
            {
              "type": "Type.multiplication",
              "value": "*",
            },
            {
              "type": "Type.number",
              "value": 2.0,
            }
          ],
        },
      };

      tests.forEach((t, expected) {
        final got = tokenToJsonTree(t);
        expect(got, expected);
      });
    });
  });
}
