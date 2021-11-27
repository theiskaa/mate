// import 'package:flutter/material.dart';
// import 'package:flutter/widgets.dart';

// import 'package:mate/mate.dart';

// void main() => runApp(const App());

// class App extends StatelessWidget {
//   const App({Key? key}) : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       debugShowCheckedModeBanner: false,
//       theme: ThemeData.dark(),
//       home: const MateView(),
//     );
//   }
// }

// class MateView extends StatefulWidget {
//   const MateView({Key? key}) : super(key: key);

//   @override
//   _MateViewState createState() => _MateViewState();
// }

// class _MateViewState extends State<MateView> {
//   final Mate mate = Mate();
//   final expressionController = TextEditingController();

//   String? result = '0';

//   void calculate() {
//     final String expression = expressionController.text;

//     // Calculate expression.
//     final res = mate.calculate(expression);

//     // Alert if expression is invalid.
//     if (res == null) {
//       setState(() => result = 'Invalid\nExpression');
//       return;
//     }

//     setState(() => result = res.toString());
//   }

//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(title: const Text('Mate')),
//       body: Center(
//         child: Column(
//           mainAxisAlignment: MainAxisAlignment.center,
//           children: [
//             Text(
//               result.toString().replaceAll('.0', ''),
//               style: const TextStyle(
//                 fontSize: 50,
//                 fontWeight: FontWeight.bold,
//               ),
//               textAlign: TextAlign.center,
//             ),
//             const SizedBox(height: 50),
//             Padding(
//               padding: const EdgeInsets.symmetric(horizontal: 20),
//               child: TextField(
//                 controller: expressionController,
//                 decoration: const InputDecoration(
//                   hintText: 'Expression',
//                   border: InputBorder.none,
//                 ),
//               ),
//             ),
//             const SizedBox(height: 30),
//             ElevatedButton(
//               onPressed: calculate,
//               child: const Text('Calculate'),
//             )
//           ],
//         ),
//       ),
//     );
//   }
// }
