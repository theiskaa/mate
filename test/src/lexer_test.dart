import 'package:mate/src/lexer.dart';
import 'package:mate/src/tokens.dart';
import 'package:test/test.dart';

void main() {
  late Lexer lexer;

  setUpAll(() => lexer = Lexer());

  // Method to check if given two token equals to each other.
  // It checks each leaf-value of each token.
  isSameTokens(Token t1, Token t2) {
    expect(t1.type, t2.type);

    if ((t1.type == t2.type) && t1.type != Type.subExpression) {
      expect(t1.value, t2.value);
      return;
    }

    expect(t1.value.length, t2.value.length);
    if (t1.value.length != t2.value.length) return;

    for (var i = 0; i < t1.value.length; i++) {
      final Token t1Val = t1.value[i], t2Val = t2.value[i];

      expect(t1Val.type, t2Val.type);
      if (t1Val.type != t2Val.type) return;

      if ((t1Val.type == t2Val.type) && t1Val.type != Type.subExpression) {
        expect(t1Val.value, t2Val.value);
      } else {
        isSameTokens(t1Val, t2Val);
      }
    }
  }

  group('Lexer', () {
    test('should parse expression properly', () {
      final tests = {
        "2.5 + 2.5": [
          Token(type: Type.number, value: Token.number(2.5)),
          Token(type: Type.addition),
          Token(type: Type.number, value: Token.number(2.5)),
        ],
        "2 + 2 * 5 / 2": [
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
        "(5*5) * 2": [
          Token(
            type: Type.subExpression,
            value: [
              Token(type: Type.subExpression, value: [
                Token(type: Type.number, value: Token.number(5)),
                Token(type: Type.multiplication),
                Token(type: Type.number, value: Token.number(5)),
              ]),
              Token(type: Type.multiplication),
              Token(type: Type.number, value: Token.number(2)),
            ],
          ),
        ],
        "((10 % 2) * 4 + 0.2) / 2 * 4 - 2": [
          Token(type: Type.subExpression, value: [
            Token(type: Type.subExpression, value: [
              Token(type: Type.subExpression, value: [
                Token(type: Type.number, value: Token.number(10)),
                Token(type: Type.percentage),
                Token(type: Type.number, value: Token.number(2)),
              ]),
              Token(type: Type.multiplication),
              Token(type: Type.number, value: Token.number(4)),
              Token(type: Type.addition),
              Token(type: Type.number, value: Token.number(0.2)),
            ]),
            Token(type: Type.division),
            Token(type: Type.number, value: Token.number(2)),
            Token(type: Type.multiplication),
            Token(type: Type.number, value: Token.number(4)),
          ]),
          Token(type: Type.subtraction),
          Token(type: Type.number, value: Token.number(2)),
        ],
      };

      // Loop and test test cases.
      tests.forEach((expression, expected) {
        final got = lexer.parse(expression);

        expect(got.length, expected.length);

        if (got.length != expected.length) return;
        for (var i = 0; i < got.length; i++) {
          isSameTokens(got[i], expected[i]);
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
        "10%2+0.8": [
          Token(type: Type.number, value: Token.number(10)),
          Token(type: Type.percentage),
          Token(type: Type.number, value: Token.number(2)),
          Token(type: Type.addition),
          Token(type: Type.number, value: Token.number(0.8)),
        ],
        "(20/4)": [
          Token(type: Type.number, value: Token.number(20)),
          Token(type: Type.division),
          Token(type: Type.number, value: Token.number(4)),
        ],
        "(20/4)*5": [
          Token(type: Type.subExpression, value: [
            Token(type: Type.number, value: Token.number(20)),
            Token(type: Type.division),
            Token(type: Type.number, value: Token.number(4)),
          ]),
          Token(type: Type.multiplication),
          Token(type: Type.number, value: Token.number(5)),
        ],
        "((4+5)*(4+2))/2": [
          Token(type: Type.subExpression, value: [
            Token(type: Type.subExpression, value: [
              Token(type: Type.number, value: Token.number(4)),
              Token(type: Type.addition),
              Token(type: Type.number, value: Token.number(5)),
            ]),
            Token(type: Type.multiplication),
            Token(type: Type.subExpression, value: [
              Token(type: Type.number, value: Token.number(4)),
              Token(type: Type.addition),
              Token(type: Type.number, value: Token.number(2)),
            ]),
          ]),
          Token(type: Type.division),
          Token(type: Type.number, value: Token.number(2)),
        ],
      };

      tests.forEach((subExpression, expected) {
        final got = lexer.parseSubExpression(subExpression);

        expect(got.length, expected.length);

        if (got.length == expected.length) {
          for (var i = 0; i < got.length; i++) {
            isSameTokens(got[i], expected[i]);
          }
        }
      });
    });
  });
}
