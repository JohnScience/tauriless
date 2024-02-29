use std::borrow::Cow;

// These error handlers are used in custom protocol handlers.

#[doc(hidden)]
pub fn handle_deserialization_error(
    cmd_name: &str,
    e: tauriless_serde::slice_to_deserialize::Error,
) -> wry::http::response::Response<Cow<'static, [u8]>> {
    #[cfg(debug_assertions)]
    println!("Failed to deserialize to `{cmd_name}::Args`: {e:?}");
    let mut body = Vec::new();
    body.extend_from_slice(b"Bad request: failed to deserialize `");
    body.extend_from_slice(cmd_name.as_bytes());
    body.extend_from_slice(b"::Args`.");
    wry::http::response::Response::builder()
        .status(wry::http::StatusCode::BAD_REQUEST)
        .header(
            wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            wry::http::HeaderValue::from_static("*"),
        )
        .body(Cow::<'static, [u8]>::Owned(body))
        .unwrap()
}

#[doc(hidden)]
pub fn handle_serialization_error(
    e: tauriless_serde::serialize_to_vec_u8::Error,
) -> wry::http::response::Response<Cow<'static, [u8]>> {
    #[cfg(debug_assertions)]
    println!("Failed to serialize the response: {e:?}");
    wry::http::response::Response::builder()
        .status(wry::http::StatusCode::INTERNAL_SERVER_ERROR)
        .header(
            wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            wry::http::HeaderValue::from_static("*"),
        )
        .body(Cow::<'static, [u8]>::Borrowed(
            b"Internal server error: failed to serialize the response.",
        ))
        .unwrap()
}

#[doc(hidden)]
pub fn handle_unknown_command(cmd_name: &str) -> wry::http::response::Response<Cow<'static, [u8]>> {
    #[cfg(debug_assertions)]
    println!("Unknown `tauriless` command: '{cmd_name}'.");
    wry::http::response::Response::builder()
        .status(wry::http::StatusCode::BAD_REQUEST)
        .header(
            wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            wry::http::HeaderValue::from_static("*"),
        )
        .body(Cow::<'static, [u8]>::Borrowed(
            b"Unknown `tauriless` command.",
        ))
        .unwrap()
}
