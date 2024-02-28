# tauriless_serde

[![Crates.io](https://img.shields.io/crates/v/tauriless_serde)](https://crates.io/crates/tauriless_serde)
[![Downloads](https://img.shields.io/crates/d/tauriless_serde.svg)](https://crates.io/crates/tauriless_serde)
[![Documentation](https://docs.rs/tauriless_serde/badge.svg)](https://docs.rs/tauriless_serde)
[![License](https://img.shields.io/crates/l/tauriless_serde)](https://crates.io/crates/tauriless_serde)

Implementation details of serialization and deserialization of messages for [`tauriless`].

## Testing

The tests are written with [`wasm-bindgen-test`] to avoid `cannot call wasm-bindgen imported functions on non-wasm targets` error
when dealing with [`wasm_bindgen::JsValue`].

```console
wasm-pack test --node
```

[`tauriless`]: https://github.com/JohnScience/tauriless/
[`wasm-bindgen-test`]: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html
[`wasm_bindgen::JsValue`]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html
