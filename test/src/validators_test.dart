import 'package:mate/src/validators.dart';
import 'package:test/test.dart';

void main() {
  group('[Validators]', () {
    test('isNum should work properly', () {
      expect(Validators.isNum('5'), true);
      expect(Validators.isNum('V'), false);
    });

    test('isPlusOrMinus should work properly', () {
      expect(Validators.isPlusOrMinus('-'), true);
      expect(Validators.isPlusOrMinus('+'), true);
      expect(Validators.isPlusOrMinus('*'), false);
      expect(Validators.isPlusOrMinus('/'), false);
    });

    test('isMultOrDiv should work properly', () {
      expect(Validators.isMultOrDiv('*'), true);
      expect(Validators.isMultOrDiv('/'), true);
      expect(Validators.isMultOrDiv('-'), false);
      expect(Validators.isMultOrDiv('+'), false);
    });

    test('isPoint should work properly', () {
      expect(Validators.isPoint('.'), true);
      expect(Validators.isPoint(','), true);
      expect(Validators.isPoint('-'), false);
      expect(Validators.isPoint('+'), false);
    });
  });
}
