use thiserror::Error;

// TODO: add a streaming APIs for serialization/deserialization

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

#[derive(Error, Debug)]
#[error(transparent)]
pub struct DeserializationError(#[from] pot::Error);

pub fn slice_to_deserialize<'a, T>(view: &'a [u8]) -> Result<T, DeserializationError>
where
    T: serde::Deserialize<'a>,
{
    pot::from_slice(view).map_err(DeserializationError)
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct SerializationError(pot::Error);

pub fn serialize_to_vec_u8<T>(value: &T) -> Result<Vec<u8>, SerializationError>
where
    T: serde::Serialize,
{
    pot::to_vec(value).map_err(SerializationError)
}

pub mod vec_to_js_value {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        FromSliceError(#[from] pot::Error),
        #[error(transparent)]
        ToJsValueError(#[from] serde_wasm_bindgen::Error),
    }
}

pub fn vec_to_js_value(vec: Vec<u8>) -> Result<wasm_bindgen::JsValue, vec_to_js_value::Error> {
    let pot_value: pot::Value = pot::from_slice(&vec)?;
    let js_value = serde_wasm_bindgen::to_value(&pot_value)?;
    Ok(js_value)
}

// placing the tests here would cause a "cannot call wasm-bindgen imported functions on non-wasm targets" error.
