import 'package:flutter/material.dart';
import '../services/tasks.dart';

class TasksView extends StatefulWidget {
  @override
  _TasksViewState createState() => _TasksViewState();
}

class _TasksViewState extends State<TasksView> {
  final ApiService apiService = ApiService('http://localhost:8000');
  Future<List<dynamic>> tasks;

  @override
  void initState() {
    super.initState();
    tasks = apiService.getTasks();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<dynamic>>(
      future: tasks,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return ListView.builder(
            itemCount: snapshot.data.length,
            itemBuilder: (context, index) {
              return ListTile(
                title: Text(snapshot.data[index]['title']),
                onTap: () {
                  // TODO: Navigate to task details view
                },
              );
            },
          );
        } else if (snapshot.hasError) {
          return Text("${snapshot.error}");
        }
        return CircularProgressIndicator();
      },
    );
  }
}
