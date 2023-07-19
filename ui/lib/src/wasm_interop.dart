import 'dart:js' as js;

class WasmInterop {
  static void callFunction(String name, List<dynamic> args) {
    js.context.callMethod(name, args);
  }
}
