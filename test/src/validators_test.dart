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

    test('isNummable should work properly', () {
      expect(Validators.isNummable('*'), false);
      expect(Validators.isNummable('/'), false);
      expect(Validators.isNummable('-'), true);
      expect(Validators.isNummable('+'), true);
    });

    test('isPoint should work properly', () {
      expect(Validators.isPoint('.'), true);
      expect(Validators.isPoint(','), true);
      expect(Validators.isPoint('-'), false);
      expect(Validators.isPoint('+'), false);
    });

    test('isNotCompletedPart should work properly', () {
      expect(Validators.isNotCompletedPart('10*2'), true);
      expect(Validators.isNotCompletedPart('10/2'), true);
      expect(Validators.isNotCompletedPart('10%2'), true);
      expect(Validators.isNotCompletedPart('10'), false);
    });

    test('isNumOrPoint should work properly', () {
      expect(Validators.isNumOrPoint('0.5'), true);
      expect(Validators.isNumOrPoint('5'), true);
      expect(Validators.isNumOrPoint('-'), false);
      expect(Validators.isNumOrPoint('+'), false);
    });

    test('isPr should work properly', () {
      expect(Validators.isPr('('), true);
      expect(Validators.isPr(')'), true);
      expect(Validators.isPr('-'), false);
      expect(Validators.isPr('+'), false);
    });

    test('isOpeningPr should work properly', () {
      expect(Validators.isOpeningPr('('), true);
      expect(Validators.isOpeningPr(')'), false);
    });

    test('isClosingPr should work properly', () {
      expect(Validators.isClosingPr(')'), true);
      expect(Validators.isClosingPr('('), false);
    });
  });
}
