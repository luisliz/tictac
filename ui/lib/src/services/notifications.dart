class NotificationsService {
  final String baseUrl;

  NotificationsService({this.baseUrl});

  Future<List<dynamic>> getNotificationsByUser(int userId) async {
    final response = await http.get('$baseUrl/notifications/user/$userId');
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Failed to load notifications');
    }
  }

  Future<void> createNotification(int userId, String message) async {
    final response = await http.post(
      '$baseUrl/notifications',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode({'userId': userId, 'message': message}),
    );
    if (response.statusCode != 201) {
      throw Exception('Failed to create notification');
    }
  }

  Future<void> updateNotification(int notificationId, String message) async {
    final response = await http.put(
      '$baseUrl/notifications/$notificationId',
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode({'message': message}),
    );
    if (response.statusCode != 200) {
      throw Exception('Failed to update notification');
    }
  }

  Future<void> deleteNotification(int notificationId) async {
    final response = await http.delete('$baseUrl/notifications/$notificationId');
    if (response.statusCode != 204) {
      throw Exception('Failed to delete notification');
    }
  }
}
