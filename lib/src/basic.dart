import 'expression_parser.dart';

/// A simple lightweight math library.
class SimpleMath extends ExpressionParser {
  // Add takes list of nums, and adds them to each other
  static num? add(List<num> nums) {
    if (nums.isEmpty) return null;

    num res = 0;
    for (var num in nums) {
      res += num;
    }

    return res;
  }

  // Subtract takes list of nums, and subracts them to each other.
  static num? subtract(List<num> nums) {
    if (nums.isEmpty) return null;

    num res = nums[0];
    for (var i = 1; i < nums.length; i++) {
      res -= nums[i];
    }

    return res;
  }

  // Multiplicate takes list of nums, and multiplicates them to each other.
  static num? multiplicate(List<num> nums) {
    if (nums.isEmpty) return null;

    num res = nums[0];
    for (var i = 1; i < nums.length; i++) {
      res *= nums[i];
    }

    return res;
  }

  // Divide takes list of nums, and divides them to each other.
  static num? divide(List<num> nums) {
    if (nums.isEmpty) return null;

    num res = nums[0];
    for (var i = 1; i < nums.length; i++) {
      res /= nums[i];
    }

    return res;
  }
}
