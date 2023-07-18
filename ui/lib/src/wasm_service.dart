import 'wasm_loader.dart';
import 'wasm_interop.dart';

class WasmService {
  final WasmLoader _loader = WasmLoader();

  Future<void> init(String path) async {
    await _loader.loadModule(path);
  }

  dynamic callFunction(String name, List<dynamic> args) {
    return _loader.callFunction(name, args);
  }
}
