use crate::Commands;

/// An [extension trait] for [`wry::WebViewBuilder`]. Notably, implements
/// [`with_tauriless_commands`](WebViewBuilderExt::with_tauriless_commands).
///
/// [extension trait]: https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html
pub trait WebViewBuilderExt {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self;
}

impl<'a> WebViewBuilderExt for wry::WebViewBuilder<'a> {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self {
        commands.register_commands(self)
    }
}
