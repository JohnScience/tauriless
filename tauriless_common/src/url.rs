/// The name of the custom protocol used for synchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_custom_protocol`].
pub const TAURILESS_SYNC_PROTOCOL: &str = "tauriless-sync";
/// The name of the custom protocol used for asynchronous `tauriless` commands.
/// It is used in [`wry::WebViewBuilder::with_asynchronous_custom_protocol`].
pub const TAURILESS_ASYNC_PROTOCOL: &str = "tauriless-async";

pub fn command_to_url(cmd_name: &str, is_sync: bool) -> String {
    format!(
        "http://{proto}.localhost/{command}",
        proto = if is_sync {
            TAURILESS_SYNC_PROTOCOL
        } else {
            TAURILESS_ASYNC_PROTOCOL
        },
        command = cmd_name.replace('_', "-")
    )
}
