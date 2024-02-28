#![doc = include_str!("../README.md")]

// TODO: add a streaming APIs for serialization/deserialization

/// A function for `tauriless-js` to deserialize a [`wasm_bindgen::JsValue`] into a [`Vec<u8>`](Vec)
/// that represents it.
///
/// This function exists to allow the `tauriless-js` WASM module to efficiently send a [`wasm_bindgen::JsValue`]
/// to the [custom protocol handler] in [`wry`].
///
/// [custom protocol handler]: https://docs.rs/wry/0.37.0/wry/struct.WebViewBuilder.html#method.with_custom_protocol
/// [`wry`]: https://docs.rs/wry/0.37.0/wry/
pub fn js_value_to_vec_u8(
    js_value: wasm_bindgen::JsValue,
) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let deserializer: serde_wasm_bindgen::Deserializer =
        serde_wasm_bindgen::Deserializer::from(js_value);
    let value = <pot::Value<'static> as serde::Deserialize>::deserialize(deserializer)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Failed to deserialize: {:?}", e)))?;
    pot::to_vec(&value)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Failed to serialize: {:?}", e)))
}

/// The dedicated module for the [`slice_to_deserialize()`] function.
pub mod slice_to_deserialize {
    /// The error type for the [`slice_to_deserialize()`](super::slice_to_deserialize()) function.
    #[derive(thiserror::Error, Debug)]
    #[error(transparent)]
    pub struct Error(#[from] pot::Error);
}

/// A function for the [custom protocol handler] to deserialize a `&[u8]` into a type
/// implementing [`serde::Deserialize`].
///
/// This function exists to allow the [custom protocol handler] to deserialize the accepted body of a request into
/// the type representing the command's arguments (a tuple of some arity) so that the command-function can be
/// called on the deserialized arguments.
///
/// [custom protocol handler]: https://docs.rs/wry/0.37.0/wry/struct.WebViewBuilder.html#method.with_custom_protocol
pub fn slice_to_deserialize<'a, T>(view: &'a [u8]) -> Result<T, slice_to_deserialize::Error>
where
    T: serde::Deserialize<'a>,
{
    let value = pot::from_slice(view)?;
    Ok(value)
}

/// The dedicated module for the [`serialize_to_vec_u8()`] function.
pub mod serialize_to_vec_u8 {
    /// The error type for the [`serialize_to_vec_u8()`](super::serialize_to_vec_u8()) function.
    #[derive(thiserror::Error, Debug)]
    #[error(transparent)]
    pub struct Error(#[from] pot::Error);
}

/// A function for the [custom protocol handler] to serialize a value implementing [`serde::Serialize`] into a
/// [`Vec<u8>`](Vec).
///
/// This function exists to allow the [custom protocol handler] to serialize the result of the command-function into
/// a [`Vec<u8>`](Vec) to be sent back to the WebView process as a response.
///
/// [custom protocol handler]: https://docs.rs/wry/0.37.0/wry/struct.WebViewBuilder.html#method.with_custom_protocol
pub fn serialize_to_vec_u8<T>(value: &T) -> Result<Vec<u8>, serialize_to_vec_u8::Error>
where
    T: serde::Serialize,
{
    let v = pot::to_vec(value)?;
    Ok(v)
}

/// The dedicated module for the [`vec_to_js_value()`] function.
pub mod vec_to_js_value {
    /// The error type for the [`vec_to_js_value()`](super::vec_to_js_value()) function.
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        /// The conversion from a slice to the internal representation failed.
        #[error(transparent)]
        FromSliceError(#[from] pot::Error),
        /// The conversion from the internal representation to [`wasm_bindgen::JsValue`] failed.
        #[error(transparent)]
        ToJsValueError(#[from] serde_wasm_bindgen::Error),
    }
}

/// A function for `tauriless-js` to serialize a [`Vec<u8>`](Vec) into a [`wasm_bindgen::JsValue`].
///
/// This function exists to allow the `tauriless-js` WASM module to efficiently receive the result
/// of the command-function from the [custom protocol handler] in [`wry`] and make it available to the
/// user of the `tauriless-js` WASM module.
///
/// [custom protocol handler]: https://docs.rs/wry/0.37.0/wry/struct.WebViewBuilder.html#method.with_custom_protocol
/// [`wry`]: https://docs.rs/wry/0.37.0/wry/
pub fn vec_to_js_value(vec: Vec<u8>) -> Result<wasm_bindgen::JsValue, vec_to_js_value::Error> {
    let pot_value: pot::Value = pot::from_slice(&vec)?;
    let js_value = serde_wasm_bindgen::to_value(&pot_value)?;
    Ok(js_value)
}
