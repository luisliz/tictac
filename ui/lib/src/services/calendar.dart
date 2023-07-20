import 'package:http/http.dart' as http;

class CalendarService {
  // Function to create an event in Google Calendar
  Future<void> createGoogleEvent(String title, DateTime start, DateTime end) async {
    var url = 'https://www.googleapis.com/calendar/v3/calendars/primary/events';
    var response = await http.post(url as Uri, body: {
      'summary': title,
      'start': {'dateTime': start.toIso8601String()},
      'end': {'dateTime': end.toIso8601String()},
    });
    if (response.statusCode != 200) {
      throw Exception('Failed to create event');
    }
  }
}
