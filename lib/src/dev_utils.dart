import 'dart:convert';

import 'package:mate/src/tokens.dart';

// Takes list of tokens and logs each tokens JSON tree.
void logTree(List<Token> tokens, [String divider = '----- ----- ----- ']) {
  const JsonEncoder encoder = JsonEncoder.withIndent('  ');

  print(" ");
  print(divider.replaceAll(' ', '') * 3);
  for (var i = 0; i < tokens.length; i++) {
    final token = tokenToJsonTree(tokens[i]);
    final String formattedToken = encoder.convert(token);

    print(formattedToken);
    print(divider * 3);
  }
}

// Creates given token's JSON tree.
// Basically used, in development mode, to debug converted expression.
Map<String, dynamic> tokenToJsonTree(Token t) {
  Map<String, dynamic> tree = {'type': t.type.toString()};

  if (!t.type.isSubExpression) {
    tree['value'] = t.value;
    return tree;
  }

  List<Map<String, dynamic>> subValues = [];
  for (var i = 0; i < t.value.length; i++) {
    final Token subT = t.value[i];
    subValues.add(tokenToJsonTree(subT));
  }

  tree['value'] = subValues;

  return tree;
}
