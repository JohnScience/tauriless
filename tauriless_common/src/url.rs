/// The name of the custom protocol used for synchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_custom_protocol`].
pub const TAURILESS_SYNC_PROTOCOL: &str = "tauriless-sync";
/// The name of the custom protocol used for asynchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_asynchronous_custom_protocol`].
pub const TAURILESS_ASYNC_PROTOCOL: &str = "tauriless-async";

pub fn command_to_url(cmd_name: &str, is_async: bool) -> String {
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
