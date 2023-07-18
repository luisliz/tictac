import 'package:http/http.dart' as http;
import 'dart:convert';

class ProjectService {
  final String apiUrl = "http://localhost:8000";

  Future<List<Project>> getProjects() async {
    final response = await http.get('$apiUrl/projects');
    if (response.statusCode == 200) {
      return (json.decode(response.body) as List)
          .map((data) => Project.fromJson(data))
          .toList();
    } else {
      throw Exception('Failed to load projects');
    }
  }

  Future<Project> createProject(Project project) async {
    final response = await http.post(
      '$apiUrl/projects',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(project),
    );

    if (response.statusCode == 201) {
      return Project.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to create project');
    }
  }

  Future<Project> getProject(int id) async {
    final response = await http.get('$apiUrl/projects/$id');
    if (response.statusCode == 200) {
      return Project.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to load project');
    }
  }

  Future<Project> updateProject(Project project) async {
    final response = await http.put(
      '$apiUrl/projects/${project.id}',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(project),
    );

    if (response.statusCode == 200) {
      return Project.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to update project');
    }
  }

  Future<void> deleteProject(int id) async {
    final http.Response response = await http.delete(
      '$apiUrl/projects/$id',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
    );

    if (response.statusCode != 204) {
      throw Exception('Failed to delete project');
    }
  }
}
