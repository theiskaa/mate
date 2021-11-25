import 'package:mate/src/lexer.dart';
import 'package:mate/src/tokens.dart';
import 'package:test/test.dart';

void main() {
  late Lexer lexer;

  setUpAll(() => lexer = Lexer());

  group('Lexer', () {
    test('should parse expression properly', () {
      final tests = {
        "2+2": [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.addition),
          Token(type: Type.number, value: Token.number(2)),
        ],
        "2+2*5/2": [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.addition),
          Token(
            type: Type.subExpression,
            value: [
              Token(type: Type.number, value: Token.number(2)),
              Token(type: Type.multiplication),
              Token(type: Type.number, value: Token.number(5)),
              Token(type: Type.division),
              Token(type: Type.number, value: Token.number(2)),
            ],
          ),
        ],
        "100 % 2 + 5 * 10 - 100 / 2": [
          // 100 % 2
          Token(
            type: Type.subExpression,
            value: [
              Token(type: Type.number, value: Token.number(100)),
              Token(type: Type.percentage),
              Token(type: Type.number, value: Token.number(2)),
            ],
          ),
          Token(type: Type.addition),
          // 5 * 10
          Token(
            type: Type.subExpression,
            value: [
              Token(type: Type.number, value: Token.number(5)),
              Token(type: Type.multiplication),
              Token(type: Type.number, value: Token.number(10)),
            ],
          ),
          Token(type: Type.subtraction),
          Token(
            type: Type.subExpression,
            value: [
              Token(type: Type.number, value: Token.number(100)),
              Token(type: Type.division),
              Token(type: Type.number, value: Token.number(2)),
            ],
          ),
        ],
      };

      // Loop and test test cases.
      tests.forEach((expression, expected) {
        final got = lexer.parse(expression);

        expect(got.length, expected.length);
        for (var i = 0; i < got.length; i++) {
          expect(got[i].type, expected[i].type);

          if (got[i].type != Type.subExpression) {
            expect(got[i].value, expected[i].value);
          } else {
            expect(got[i].value.length, expected[i].value.length);

            for (var j = 0; j < got[i].value.length; j++) {
              final subGot = got[i].value[j];
              final subExp = expected[i].value[j];

              expect(subGot.type, subExp.type);
              expect(subGot.value, subExp.value);
            }
          }
        }
      });
    });

    test('should parse sub expression correctly', () {
      final tests = {
        "2*2": [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(2)),
        ],
        "2*2/2%2": [
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.division),
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.percentage),
          Token(type: Type.number, value: Token.number(2)),
        ],
      };

      tests.forEach((subExpression, expected) {
        final got = lexer.parseSubExpression(subExpression);

        expect(got.length, expected.length);

        if (got.length == expected.length) {
          for (var i = 0; i < got.length; i++) {
            expect(got[i].type, expected[i].type);
            expect(got[i].value, expected[i].value);
          }
        }
      });
    });
  });
}
