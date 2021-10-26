<p align="center">
 <img width="350" src="https://user-images.githubusercontent.com/59066341/138723337-92fc06a7-a139-4b08-a770-6c62742ba0e7.png" alt="Package Logo">
</p>

## Usage 

Create local instance of expression parser.
```dart
final ExpressionParser expressionParser = ExpressionParser();
```

Then you can calculate your "string" expression like:
```dart
final String exp = "-2 + 5 + 10 * 2";
final double? result = expressionParser.calculate(exp); // --> 23
```

When we call calculate, it checks the validness of expression automatically.
So, we needn't to do some manual checking here. (If expression is invalid, then result would be null)

But in anyways, if you wanna check the validness of expression manually, you can do it, like:
```dart
final bool isInvalid = expressionParser.isInvalidExp(exp) // --> false
```

Also, you can enable, taking the sum of the last result and current by setting `keepAddingOn` to `true`.

So, when you enable it, parser allways store your last expression parts. By doing that it can take sum of the last result you got, and the current one.

For example: 
you calculate `"8+2"` and got `10` as result. Parser keeps `["8", "+2"]` as parts of expression. 
So, when you make operation after that one, e.g: `"2+3"`, then parts of expression would be: `["8", "+2", "2", "+3"]` So, the final result would be `(8+2) + (2+3) = 15`.

> **Check [official example](https://github.com/theiskaa/mate/blob/main/example/main.dart) to see the UI implementation of Mate**


## Explanation
Parser divides string expression as parts (Stores them in custom Expression class), then loops through the parts and takes sum of them.
If string expression is `"2+5"` then parts would be: `["2", "+5"]`. So the sum of parts would be `2+(+5)` --> `7`.

Let's see with default example: `"-2 + 5 + 10 * 2"`.
In this case, parts would be: `["-2", "+5", "10*2"]`. We got `"10*2"` at the end of the part, because of operation priority.
So, the final result would be: `-2+(+5)+(10*2)` --> `3+20` --> `23`.