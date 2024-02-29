use std::future::Future;

pub use tauriless_macro::{command, commands};
pub use tauriless_serde::{serialize_to_vec_u8, slice_to_deserialize};

mod commands;
mod handlers;
mod web_view_builder_ext;

pub use commands::Commands;
pub use handlers::{
    handle_deserialization_error, handle_serialization_error, handle_unknown_command,
};
pub use web_view_builder_ext::WebViewBuilderExt;

pub trait Command {
    const IS_ASYNC: bool;
    const NAME: &'static str;
    /// URLs are not allowed to contain underscores, so we use dashes instead.
    const URL_NAME: &'static str;
    type Args: for<'a> serde::Deserialize<'a>;
    type RetTy: serde::Serialize;

    fn sync_command(args: Self::Args) -> Self::RetTy;
    // Using `async fn` triggers a warning:
    //
    // ```
    // use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
    // you can suppress this lint if you plan to use the trait only in your own code, or do not care
    // about auto traits like `Send` on the `Future`.
    // ```
    fn async_command(args: Self::Args) -> impl Future<Output = Self::RetTy> + Send;
}
