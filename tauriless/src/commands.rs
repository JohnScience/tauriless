pub trait Commands {
    fn register_commands(self, builder: wry::WebViewBuilder) -> wry::WebViewBuilder;
}

impl<F> Commands for F
where
    F: Fn(wry::WebViewBuilder) -> wry::WebViewBuilder,
{
    fn register_commands(self, builder: wry::WebViewBuilder) -> wry::WebViewBuilder {
        self(builder)
    }
}
