use std::{borrow::Cow, future::Future};

pub use pot;
pub use tauriless_macro::{command, commands};

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

pub trait WebViewBuilderExt {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self;
}

impl<'a> WebViewBuilderExt for wry::WebViewBuilder<'a> {
    fn with_tauriless_commands<C: Commands>(self, commands: C) -> Self {
        commands.register_commands(self)
    }
}

pub trait Command {
    const IS_ASYNC: bool;
    const NAME: &'static str;
    /// URLs are not allowed to contain underscores, so we use dashes instead.
    const URL_NAME: &'static str;
    type Args: for<'a> serde::Deserialize<'a>;
    type RetTy: serde::Serialize;

    fn register_in(builder: wry::WebViewBuilder) -> wry::WebViewBuilder;
    fn sync_command(args: Self::Args) -> Self::RetTy;
    fn async_command(args: Self::Args) -> impl Future<Output = Self::RetTy>;
}

pub fn handle_deserialization_error(
    cmd_name: &str,
    e: pot::Error,
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

pub fn handle_serialization_error(
    e: pot::Error,
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

pub trait SyncCommand {
    type Args: for<'a> serde::Deserialize<'a>;
    type RetTy: serde::Serialize;
    const NAME: &'static str;
    const URL_NAME: &'static str;

    fn command(args: Self::Args) -> Self::RetTy;

    fn custom_protocol(
        req: wry::http::request::Request<Vec<u8>>,
    ) -> wry::http::response::Response<Cow<'static, [u8]>> {
        let (_parts, body): (wry::http::request::Parts, Vec<u8>) = req.into_parts();
        #[cfg(debug_assertions)]
        println!("Received a request: {:#?}", body);
        let args: Self::Args = match pot::from_slice(body.as_slice()) {
            Ok(args) => args,
            Err(e) => return handle_deserialization_error(Self::NAME, e),
        };
        let ret: Self::RetTy = Self::command(args);
        let resp_body: Vec<u8> = match pot::to_vec(&ret) {
            Ok(resp_body) => resp_body,
            Err(e) => return handle_serialization_error(e),
        };
        #[cfg(debug_assertions)]
        println!("Sending a response: {:#?}", resp_body);
        wry::http::response::Response::builder()
            .status(wry::http::StatusCode::OK)
            .header(
                wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                wry::http::HeaderValue::from_static("*"),
            )
            .body(Cow::Owned(resp_body))
            .unwrap()
    }
}

impl<C> Command for C
where
    C: SyncCommand + 'static,
{
    const IS_ASYNC: bool = false;
    const NAME: &'static str = C::NAME;
    const URL_NAME: &'static str = C::URL_NAME;
    type Args = C::Args;
    type RetTy = C::RetTy;

    fn register_in(builder: wry::WebViewBuilder) -> wry::WebViewBuilder {
        builder.with_custom_protocol(C::NAME.to_owned(), C::custom_protocol)
    }

    fn sync_command(args: Self::Args) -> Self::RetTy {
        C::command(args)
    }

    async fn async_command(_args: Self::Args) -> Self::RetTy {
        unreachable!()
    }
}
