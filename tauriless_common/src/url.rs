/// The name of the custom protocol used by tauriless.
pub const TAURILESS_PROTOCOL: &str = "tauiriless";

/// Converts a `tauriless` command name to the URL for its invocation. If the command name contains underscores,
/// they will be replaced with hyphens as required by [RFC 1738], "Uniform Resource Locators (URL)".
///
/// [RFC 1738]: https://datatracker.ietf.org/doc/html/rfc1738
pub fn command_to_url(cmd_name: &str) -> String {
    // Q: Is this Windows-specific?
    format!(
        "http://{proto}.localhost/{command}",
        proto = TAURILESS_PROTOCOL,
        command = cmd_name.replace('_', "-")
    )
}
