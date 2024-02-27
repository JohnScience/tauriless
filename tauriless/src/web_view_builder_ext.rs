use crate::Commands;

pub trait WebViewBuilderExt {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self;
}

impl<'a> WebViewBuilderExt for wry::WebViewBuilder<'a> {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self {
        commands.register_commands(self)
    }
}
