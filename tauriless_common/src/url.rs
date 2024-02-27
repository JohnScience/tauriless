/// The name of the custom protocol used for synchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_custom_protocol`].
///
/// [`wry::WebViewBuilder::with_custom_protocol`]: https://docs.rs/wry/latest/wry/struct.WebViewBuilder.html#method.with_custom_protocol
pub const TAURILESS_SYNC_PROTOCOL: &str = "tauriless-sync";
/// The name of the custom protocol used for asynchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_asynchronous_custom_protocol`].
///
/// [`wry::WebViewBuilder::with_asynchronous_custom_protocol`]: https://docs.rs/wry/latest/wry/struct.WebViewBuilder.html#method.with_asynchronous_custom_protocol
pub const TAURILESS_ASYNC_PROTOCOL: &str = "tauriless-async";

/// Converts a `tauriless` command name to the URL for its invocation. If the command name contains underscores,
/// they will be replaced with hyphens as required by [RFC 1738], "Uniform Resource Locators (URL)".
///
/// [RFC 1738]: https://datatracker.ietf.org/doc/html/rfc1738
pub fn command_to_url(cmd_name: &str, is_async: bool) -> String {
    // Q: Is this Windows-specific?
    format!(
        "http://{proto}.localhost/{command}",
        proto = if is_async {
            TAURILESS_ASYNC_PROTOCOL
        } else {
            TAURILESS_SYNC_PROTOCOL
        },
        command = cmd_name.replace('_', "-")
    )
}
