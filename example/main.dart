import 'package:mate/mate.dart';

/*
  This file is just a simple implementation example of mate.

  See official documentation/explanation of working style of mate. - [https://github.com/theiskaa/mate#readme]  
  And here is UI/Flutter implementation of mate - [https://github.com/theiskaa/mate/blob/main/example/app.dart]

*/

void main() {
  // Create new mate instance.
  final Mate mate = Mate();

  const expressions = [
    '2+2', // 4
    '-2 + 5 + 10 * 2', // 23
    '10 % 2 + 0.8 + 19 / 2 - 0.5', // 10
    "*2abc-10/%" // null (invalid),
  ];

  for (var expression in expressions) {
    // use [calculate] function, to parse, and calculate expression.
    final result = mate.calculate(expression);

    print(result);
  }
}
