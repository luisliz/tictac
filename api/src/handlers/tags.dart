import 'package:http/http.dart' as http;
import 'dart:convert';

class Tag {
  final int id;
  final String name;

  Tag({this.id, this.name});

  factory Tag.fromJson(Map<String, dynamic> json) {
    return Tag(
      id: json['id'],
      name: json['name'],
    );
  }
}

class TagService {
  static const String url = 'http://localhost:8000/tags';

  static Future<List<Tag>> getTags() async {
    final response = await http.get(url);
    if (response.statusCode == 200) {
      List jsonResponse = json.decode(response.body);
      return jsonResponse.map((tag) => new Tag.fromJson(tag)).toList();
    } else {
      throw Exception('Failed to load tags from API');
    }
  }

  static Future<Tag> createTag(Tag tag) async {
    final response = await http.post(url,
        headers: {"Content-Type": "application/json"},
        body: jsonEncode(<String, String>{
          'name': tag.name,
        }));
    if (response.statusCode == 201) {
      return Tag.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to create tag');
    }
  }

  static Future<Tag> updateTag(int id, Tag tag) async {
    final response = await http.put('$url/$id',
        headers: {"Content-Type": "application/json"},
        body: jsonEncode(<String, String>{
          'name': tag.name,
        }));
    if (response.statusCode == 200) {
      return Tag.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to update tag');
    }
  }

  static Future<void> deleteTag(int id) async {
    final http.Response response = await http.delete('$url/$id');
    if (response.statusCode != 200) {
      throw Exception('Failed to delete tag');
    }
  }

  static Future<List<Tag>> getTasksByTag(int id) async {
    final response = await http.get('$url/$id/tasks');
    if (response.statusCode == 200) {
      List jsonResponse = json.decode(response.body);
      return jsonResponse.map((tag) => new Tag.fromJson(tag)).toList();
    } else {
      throw Exception('Failed to load tasks by tag from API');
    }
  }
}
