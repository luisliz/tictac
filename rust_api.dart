import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

class RustApi {
  static final _api = RustWasmApi();

  Future<String> processData(String args) async {
    return _api.processData(args);
  }
}
