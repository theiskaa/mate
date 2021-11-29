## 0.0.5 - (29/11/2021)
- **Resolved: [#13](https://github.com/theiskaa/mate/issues/13) Re-structure parsing algorithm**
- **Resolved: [#3](https://github.com/theiskaa/mate/issues/3) Parse & Calculate parentheses**

Now, mate has 2x clean and fast parsing structure, It converts given expression to tokens list, and by using that list expression class can calculate the final result.
However, it looks like an interpreter now.

**And also mate can calculate highly-nested (with parentheses) expressions.**
#### Example:
```dart
final String exp = "((4 + 5) * ((2 * 2) + 2)) / (1.5 * 2)"
final double? result = mate.calculate(exp); // --> 18
```

## 0.0.4 - (29/10/2021)
- **Resolved: [#10](https://github.com/theiskaa/mate/issues/10) Percentage Calculation Support**

**Now, mate can calculate [X]'s [Y] percent (from string)**

Example:
```dart
final String exp = "10 % 2"; 
final double? result = expressionParser.calculate(exp); // --> 0,2
```

## 0.0.3 - (27/10/2021)
- **Resolved: [#8](https://github.com/theiskaa/mate/issues/8) Native Support**

**Now, mate is independet from flutter**

## 0.0.2 - (26/10/2021)
- **Resolved: [#4](https://github.com/theiskaa/mate/issues/4) and [#5](https://github.com/theiskaa/mate/issues/5)**

**Now, parser can understand rational numbers and calculate it.**

Example:
```dart
final String exp = "25,58 * 96 + 44,32";
final double? result = ExpressionParser.calculate(exp); // --> 2500
```

## 0.0.1 - (25/10/2021)

<p align="center">
 <img width="200" src="https://user-images.githubusercontent.com/59066341/138723337-92fc06a7-a139-4b08-a770-6c62742ba0e7.png" alt="Package Logo">
</p>

**The very first version of "Mate", that can calculate expressions written as string format.**

Example:
```dart
final String exp = "-2 + 5 + 10 * 2";
final double? result = expressionParser.calculate(exp); // --> 23
```