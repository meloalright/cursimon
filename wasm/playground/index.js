const js = import("../pkg/wasm.js");
js.then(js => {
  js.run_app()
});