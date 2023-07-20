import 'package:flutter/material.dart';
import 'package:tictac/src/screens/task_list_view.dart';
import 'screens/task_list_view.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Task Manager',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: TaskListView(),
    );
  }
}
