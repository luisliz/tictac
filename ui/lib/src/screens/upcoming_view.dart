import 'package:flutter/material.dart';

class UpcomingView extends StatefulWidget {
  @override
  _UpcomingViewState createState() => _UpcomingViewState();
}

class _UpcomingViewState extends State<UpcomingView> {
  final ApiService apiService = ApiService('http://localhost:8000');
  Future<List<dynamic>> tasks;

  @override
  void initState() {
    super.initState();
    tasks = apiService.getTasks();
  }

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
