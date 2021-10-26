import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

import 'package:mate/mate.dart';

void main() => runApp(const App());

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),
      home: const ExpressionParserView(),
    );
  }
}

class ExpressionParserView extends StatefulWidget {
  const ExpressionParserView({Key? key}) : super(key: key);

  @override
  _ExpressionParserViewState createState() => _ExpressionParserViewState();
}

class _ExpressionParserViewState extends State<ExpressionParserView> {
  final ExpressionParser expressionParser = ExpressionParser();
  final operationController = TextEditingController();

  String? result = '0';

  void calculate() {
    final String operation = operationController.text;

    // We can catch expression's validness like:
    if (expressionParser.isInvalidExp(operation)) {
      setState(() => result = 'Invalid\nOperation');
      return;
    }

    // Calulcate operation if it's valid.
    final res = expressionParser.calculate(operation);

    setState(() => result = res.toString());
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Mate')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              result.toString().replaceAll('.0', ''),
              style: const TextStyle(
                fontSize: 50,
                fontWeight: FontWeight.bold,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 50),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: TextField(
                controller: operationController,
                decoration: const InputDecoration(
                  hintText: 'Operation',
                  border: InputBorder.none,
                ),
              ),
            ),
            const SizedBox(height: 30),
            ElevatedButton(
              onPressed: calculate,
              child: const Text('Calculate'),
            )
          ],
        ),
      ),
    );
  }
}
