import 'package:flutter/material.dart';

class UpcomingView extends StatefulWidget {
  @override
  _UpcomingViewState createState() => _UpcomingViewState();
}

class _UpcomingViewState extends State<UpcomingView> {
  // TODO: Replace with your actual data
  final List<String> tasks = <String>['Task 1', 'Task 2', 'Task 3'];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Upcoming Tasks'),
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
