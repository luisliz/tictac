import 'dart:js' as js;
import 'dart:typed_data';

class WasmLoader {
  js.JsObject _wasmModule;

  Future<void> loadModule(String path) async {
    final response = await HttpRequest.request(
      path,
      method: 'GET',
      responseType: 'arraybuffer',
    );
    final buffer = response.response as ByteBuffer;
    final module = await js.context['WebAssembly'].callMethod('compile', [buffer]);
    _wasmModule = js.JsObject.fromBrowserObject(module);
  }

  dynamic callFunction(String name, List<dynamic> args) {
    return _wasmModule.callMethod(name, args);
  }
}
