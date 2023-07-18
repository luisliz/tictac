import 'package:flutter/material.dart';

class TaskDetailsView extends StatefulWidget {
  @override
  _TaskDetailsViewState createState() => _TaskDetailsViewState();
}

class _TaskDetailsViewState extends State<TaskDetailsView> {
  final ApiService apiService = ApiService('http://localhost:8000');
  Future<dynamic> task;

  @override
  void initState() {
    super.initState();
    // Replace '1' with the actual task ID
    task = apiService.getTask(1);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Task Details'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Text('Title: $task', style: TextStyle(fontSize: 18)),
            SizedBox(height: 8),
            Text('Description: This is a description of $task', style: TextStyle(fontSize: 18)),
            SizedBox(height: 8),
            RaisedButton(
              onPressed: () {
                // TODO: Mark task as complete
              },
              child: Text('Mark as Complete'),
            ),
          ],
        ),
      ),
    );
  }
}
