use tauriless_serde::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    // The invocations of the functions are meant to not contain the variable names.
    let supplied_args: serde_json::Value = serde_json::json!([1, 2]);
    // However, the arguments values are accessible in the `tauriless-js` WASM module as a `JsValue`.
    let supplied_args: wasm_bindgen::JsValue =
        serde_wasm_bindgen::to_value(&supplied_args).unwrap();

    // In the `tauriless-js` WASM module, the `JsValue` is deserialized into a `Vec<u8>`
    // which will be sent from the WebView process to the custom protocol handler.
    let supplied_args: Vec<u8> = crate::js_value_to_vec_u8(supplied_args).unwrap();
    // In the custom protocol handler, the `Vec<u8>` is deserialized into a type
    // of the command's arguments, which is a tuple of unknown arity.
    // In this case, we pretend that the command's arguments are a tuple of two integers.
    let supplied_args: (i32, i32) = crate::slice_to_deserialize(&supplied_args).unwrap();
    let (a, b) = supplied_args;
    // In the custom protocol handler, the command is executed with the arguments.
    let command = |a: i32, b: i32| a + b;
    let result = command(a, b);
    // The result then is serialized into a `Vec<u8>` to be sent back to the WebView process as response.
    let result: Vec<u8> = crate::serialize_to_vec_u8(&result).unwrap();
    // Then the result is serialized into a `JsValue` to be sent back to the WebView process as response.
    let result: wasm_bindgen::JsValue = crate::vec_to_js_value(result).unwrap();

    // Just ensuring that the result of addition is 3.
    let result: i32 = serde_wasm_bindgen::from_value(result).unwrap();
    assert_eq!(result, 3);
}
