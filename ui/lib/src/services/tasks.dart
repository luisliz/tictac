import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  final String baseUrl;

  ApiService(this.baseUrl);

  Future<List<dynamic>> getTasks() async {
    final response = await http.get('$baseUrl/tasks');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }

  Future<List<dynamic>> getTasksByUser(int userId) async {
    final response = await http.get('$baseUrl/tasks/user/$userId');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }

  Future<List<dynamic>> getTasksByList(int listId) async {
    final response = await http.get('$baseUrl/tasks/list/$listId');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }

  Future<List<dynamic>> getTasksByProject(int projectId) async {
    final response = await http.get('$baseUrl/tasks/project/$projectId');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }

  Future<List<dynamic>> getTasksByStatus(String status) async {
    final response = await http.get('$baseUrl/tasks/status/$status');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }

  Future<dynamic> createTask(Map<String, dynamic> task) async {
    final response = await http.post(
      '$baseUrl/tasks',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(task),
    );
    if (response.statusCode == 201) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to create task');
    }
  }

  Future<dynamic> updateTask(int taskId, Map<String, dynamic> task) async {
    final response = await http.put(
      '$baseUrl/tasks/$taskId',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(task),
    );
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to update task');
    }
  }

  Future<void> deleteTask(int taskId) async {
    final response = await http.delete('$baseUrl/tasks/$taskId');
    if (response.statusCode != 204) {
      throw Exception('Failed to delete task');
    }
  }

  Future<List> getUpcomingTasks() {}
}
