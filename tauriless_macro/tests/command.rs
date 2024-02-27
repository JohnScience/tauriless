use serde::Deserialize;
use tauriless_macro::command;

#[command]
fn argsless_sync_command() {}

#[command]
async fn argsless_async_command() -> i32 {
    42
}

#[command]
fn command_with_args(_a: i32, _b: i32) {}

#[command]
fn command_with_args_and_return_type(a: i32, b: i32) -> i32 {
    a + b
}

// struct NotDeserializable;
// #[command]
// fn command_with_non_deserializable_arg(b: i32, _a: NotDeserializable) {}

// TODO: consider adding tests for de-serialization of arguments from JSON, and the serialization
// of the return type.

fn main() {
    // args as written by the WASM library user
    let args: serde_json::Value = serde_json::json!([1, 2]);
    // JS value as accepted by the WASM library
    let args: wasm_bindgen::JsValue = serde_wasm_bindgen::to_value(&args).unwrap();

    // internal details of the WASM library
    let args = serde_wasm_bindgen::Deserializer::from(args);
    let args: pot::Value<'static> =
        <pot::Value<'static> as Deserialize>::deserialize(args).unwrap();

    // args as sent to the custom protocol handler
    let args: Vec<u8> = pot::to_vec(&args).unwrap();

    // internal details of the custom protocol handler
    let args: <__command_command_with_args_and_return_type as tauriless::Command>::Args =
        pot::from_slice(&args).unwrap();
    let ret =
        <__command_command_with_args_and_return_type as tauriless::Command>::sync_command(args);

    let expected = command_with_args_and_return_type(1, 2);
    assert_eq!(ret, expected);
}
