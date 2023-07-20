import 'package:flutter/material.dart';

import '../services/tasks.dart';

class TodayView extends StatefulWidget {
  @override
  _TodayViewState createState() => _TodayViewState();
}

class _TodayViewState extends State<TodayView> {
  final ApiService apiService = ApiService('http://localhost:8000');
  Future<List<dynamic>> tasks;

  @override
  void initState() {
    super.initState();
    tasks = apiService.getTodaysTasks();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Today\'s Tasks'),
      ),
      body: ListView.builder(
        itemCount: tasks.length,
        itemBuilder: (BuildContext context, int index) {
          return ListTile(
            title: Text(tasks[index]),
            onTap: () {
              // TODO: Navigate to Task Details View
            },
          );
        },
      ),
    );
  }
}
