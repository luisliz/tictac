import 'dart:convert';
import 'package:http/http.dart' as http;

class ListsService {
  final String baseUrl;

  ListsService({this.baseUrl});

  Future<List<dynamic>> getListsByUser(int userId) async {
    final response = await http.get('$baseUrl/lists/user/$userId');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load lists');
    }
  }

  Future<void> reorderTasks(int listId, List<int> newOrder) async {
    final response = await http.put(
      '$baseUrl/lists/$listId/reorder',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(newOrder),
    );
    if (response.statusCode != 200) {
      throw Exception('Failed to reorder tasks');
    }
  }

  Future<List<dynamic>> getTasksByList(int listId) async {
    final response = await http.get('$baseUrl/lists/$listId/tasks');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load tasks');
    }
  }
}
